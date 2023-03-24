mod signals;
pub use signals::*;

mod slots;
pub use slots::*;

mod board;
use board::{BoardImpl, LegalMove, PieceAtSquare};

pub struct BoardConImpl {
    tx: std::sync::mpsc::Sender<Signals>,
    rx: std::sync::mpsc::Receiver<Slots>,

    board: BoardImpl,

    highlighted_square: Option<chess::Square>,
    dragged_piece: Option<PieceAtSquare>,
}

impl BoardConImpl {
    pub fn from_rx(
        slots_rx: std::sync::mpsc::Receiver<Slots>,
    ) -> (Self, std::sync::mpsc::Receiver<Signals>) {
        let (signals_tx, signals_rx) = std::sync::mpsc::channel::<Signals>();

        let _self = Self {
            tx: signals_tx,
            rx: slots_rx,

            board: BoardImpl::default(),

            highlighted_square: None,
            dragged_piece: None,
        };

        (_self, signals_rx)
    }
}

impl BoardConImpl {
    pub fn run(&mut self) {
        while let Ok(slot) = self.rx.recv() {
            self.handle_slot(slot);
        }
    }

    pub fn handle_slot(&mut self, slot: Slots) {
        match slot {
            Slots::Resync => self.resync_board(),

            Slots::MouseEvent { slot, piece_size } => match slot {
                MouseEventSlots::Clicked { x, y } => self.coord_clicked(x, y, piece_size),
                MouseEventSlots::Drag(slot) => match slot {
                    DragSlots::Started {
                        src_x,
                        src_y,
                        dest_x,
                        dest_y,
                    } => self.coord_drag_started(src_x, src_y, dest_x, dest_y, piece_size),
                    DragSlots::Updated { x, y } => self.coord_drag_event(x, y, piece_size),
                    DragSlots::Ended {
                        src_x,
                        src_y,
                        dest_x,
                        dest_y,
                    } => self.coord_drag_ended(src_x, src_y, dest_x, dest_y, piece_size),
                },
            },
        }
    }

    pub fn emit(&self, signal: impl Into<Signals>) {
        self.tx.send(signal.into()).unwrap();
    }
}

impl BoardConImpl {
    pub fn resync_board(&self) {
        println!("Synchronizing board...");

        self.emit(BoardSignals::Reset);

        for piece in self.board.pieces() {
            self.emit(PieceSignals::Place {
                id: piece.piece_id(),
                square: piece.square.to_int(),
            });
        }
    }

    pub fn coord_clicked(&mut self, x: f32, y: f32, piece_size: u32) {
        self.reset_highlights();

        let sq = BoardConImpl::coord_to_square(x, y, piece_size);
        if let Some(src_sq) = self.highlighted_square.take() {
            if src_sq == sq {
                // Clicked the same square twice
                return;
            }

            if let Some(legal_move) = self.legal_move(src_sq, sq) {
                // A legal move!
                // TODO: promotion

                self.emit(PieceSignals::Move {
                    src_square: src_sq.to_int(),
                    dest_square: sq.to_int(),
                });

                self.en_passant_castling(&legal_move);

                self.board.apply_move(legal_move);

                return;
            }
        }

        // No new piece selected
        if self.board.piece_on(sq).is_none() {
            return;
        }

        self.highlighted_square.replace(sq);
        self.emit(HighlightRectSignals::Show {
            square: sq.to_int(),
        });

        self.show_hints(sq);
    }

    pub fn coord_drag_started(
        &mut self,
        src_x: f32,
        src_y: f32,
        dest_x: f32,
        dest_y: f32,
        piece_size: u32,
    ) {
        self.reset_highlights();

        self.highlighted_square.take();

        let sq = BoardConImpl::coord_to_square(src_x, src_y, piece_size);

        let piece = if let Some(val) = self.board.piece_on(sq) {
            val
        } else {
            return;
        };

        self.highlighted_square.replace(sq);

        let piece_id = piece.piece_id();
        self.dragged_piece.replace(piece);

        {
            let square = sq.to_int();
            self.emit(PieceSignals::Remove { square });
            self.emit(HighlightRectSignals::Show { square });
            self.emit(HoverRectSignals::Show { square });
        }

        self.emit(PhantomPieceSignals::Show {
            id: piece_id,
            x: dest_x,
            y: dest_y,
        });

        self.show_hints(sq);
    }

    pub fn coord_drag_event(&mut self, x: f32, y: f32, piece_size: u32) {
        if self.dragged_piece.is_none() {
            return;
        }

        self.emit(PhantomPieceSignals::Update { x, y });

        let sq = BoardConImpl::coord_to_square(x, y, piece_size);
        self.emit(HoverRectSignals::Show {
            square: sq.to_int(),
        });
    }

    pub fn coord_drag_ended(
        &mut self,
        src_x: f32,
        src_y: f32,
        dest_x: f32,
        dest_y: f32,
        piece_size: u32,
    ) {
        {
            self.emit(PhantomPieceSignals::Hide);
            self.emit(HoverRectSignals::Hide);
        }

        let piece = if let Some(val) = self.dragged_piece.take() {
            val
        } else {
            return;
        };

        let src_sq = piece.square;
        let dest_sq = BoardConImpl::coord_to_square(dest_x, dest_y, piece_size);

        let legal_move = if let Some(val) = self.legal_move(src_sq, dest_sq) {
            val
        } else {
            self.emit(PieceSignals::Place {
                id: piece.piece_id(),
                square: src_sq.to_int(),
            });
            return;
        };

        // A legal move
        self.highlighted_square.take();

        self.emit(HintSignals::Reset);
        self.emit(HighlightRectSignals::Hide);

        // TODO: promotion

        // Place down dragged piece
        self.emit(PieceSignals::Place {
            id: piece.piece_id(),
            square: dest_sq.to_int(),
        });

        self.en_passant_castling(&legal_move);

        self.board.apply_move(legal_move);
    }
}

impl BoardConImpl {
    fn reset_highlights(&self) {
        self.emit(HintSignals::Reset);
        self.emit(PhantomPieceSignals::Hide);
        self.emit(HighlightRectSignals::Hide);
        self.emit(HoverRectSignals::Hide);
    }

    fn show_hints(&self, sq: chess::Square) {
        let legal_move_vec = self.board.moves_of(sq);
        for legal_move in legal_move_vec {
            if legal_move.capture.is_some() {
                self.emit(HintSignals::Capture {
                    square: legal_move.dest().to_int(),
                });
            } else {
                self.emit(HintSignals::Place {
                    square: legal_move.dest().to_int(),
                });
            }
        }
    }

    fn legal_move(&self, src_sq: chess::Square, dest_sq: chess::Square) -> Option<LegalMove> {
        let legal_move_vec = self.board.moves_of(src_sq);
        for legal_move in legal_move_vec {
            if legal_move.dest() == dest_sq {
                // A legal move!
                return Some(legal_move);
            }
        }

        None
    }

    fn en_passant_castling(&self, legal_move: &LegalMove) {
        if let Some(ep_square) = legal_move.en_passant {
            self.emit(PieceSignals::Remove {
                square: ep_square.to_int(),
            });
        }

        if let Some((src_sq, dest_sq)) = legal_move.castling {
            self.emit(PieceSignals::Move {
                src_square: src_sq.to_int(),
                dest_square: dest_sq.to_int(),
            });
        }
    }

    pub fn coord_to_square(x: f32, y: f32, piece_size: u32) -> chess::Square {
        let file: u32 = x as u32 / piece_size;
        let rank: u32 = 7 - (y as u32 / piece_size);

        chess::Square::make_square(
            chess::Rank::from_index(rank as usize),
            chess::File::from_index(file as usize),
        )
    }
}
