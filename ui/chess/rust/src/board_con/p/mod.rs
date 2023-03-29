mod signals;
pub use signals::*;

mod slots;
pub use slots::*;

mod board;
use board::{BoardImpl, LegalMove, PieceAtSquare, SquareExt};

pub struct BoardConImpl {
    tx: std::sync::mpsc::Sender<Signals>,
    rx: std::sync::mpsc::Receiver<Slots>,

    board: BoardImpl,

    dragged_piece: Option<PieceAtSquare>,
    highlighted_square: Option<chess::Square>,
    promotion_squares: Option<(chess::Square, chess::Square)>,
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

            dragged_piece: None,
            highlighted_square: None,
            promotion_squares: None,
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
                    DragSlots::Started { src_x, src_y } => {
                        self.coord_drag_started(src_x, src_y, piece_size)
                    }
                    DragSlots::Ended { dest_x, dest_y } => {
                        self.coord_drag_ended(dest_x, dest_y, piece_size)
                    }
                },
            },
            Slots::Promote { id } => self.finalize_promotion(id),
        }
    }

    pub fn emit(&self, signal: impl Into<Signals>) {
        self.tx.send(signal.into()).unwrap();
    }
}

impl BoardConImpl {
    pub fn resync_board(&self) {
        self.emit(Signals::Reset);

        for piece in self.board.pieces() {
            self.emit(PieceSignals::Place {
                id: piece.piece_id(),
                square: piece.square.to_int(),
            });
        }
    }

    pub fn coord_clicked(&mut self, x: f32, y: f32, piece_size: u32) {
        self.reset_highlights();
        let highlighted_square = self.highlighted_square.take();

        if self.check_promoting() {
            return;
        }

        let sq = BoardConImpl::coord_to_square(x, y, piece_size);
        if let Some(src_sq) = highlighted_square {
            if src_sq == sq {
                // Clicked the same square twice
                return;
            }

            if let Some(legal_move) = self.legal_move(src_sq, sq) {
                // A legal move!
                self.emit(PieceSignals::Move {
                    src_square: src_sq.to_int(),
                    dest_square: sq.to_int(),
                });

                self.try_apply_move(legal_move);

                return;
            }
        }

        // No new piece selected
        if self.board.piece_on(sq).is_none() {
            return;
        }

        self.highlighted_square.replace(sq);
        self.emit(Signals::Highlight {
            square: Some(sq.to_int()),
        });

        self.show_hints(sq);
    }

    pub fn coord_drag_started(&mut self, src_x: f32, src_y: f32, piece_size: u32) {
        self.reset_highlights();
        self.highlighted_square.take();

        if self.check_promoting() {
            return;
        }

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
            self.emit(Signals::Highlight {
                square: Some(square),
            });
        }

        self.emit(Signals::Phantom { id: Some(piece_id) });

        self.show_hints(sq);
    }

    pub fn coord_drag_ended(&mut self, dest_x: f32, dest_y: f32, piece_size: u32) {
        {
            self.emit(Signals::Phantom { id: None });
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

        self.emit(Signals::Hint {
            squares: Vec::new(),
        });
        self.emit(Signals::Capture {
            squares: Vec::new(),
        });
        self.emit(Signals::Highlight { square: None });

        // Place down dragged piece
        self.emit(PieceSignals::Place {
            id: piece.piece_id(),
            square: dest_sq.to_int(),
        });

        self.try_apply_move(legal_move);
    }

    pub fn finalize_promotion(&mut self, id: u8) {
        if self.promotion_squares.is_none() {
            println!("nothing to promote");
            return;
        }

        let (src_sq, dest_sq) = self.promotion_squares.take().unwrap();
        let piece = chess::ALL_PIECES[id as usize];

        let chess_move = chess::ChessMove::new(src_sq, dest_sq, Some(piece));
        let legal_move = LegalMove {
            inner: chess_move,
            // fields not used
            capture: None,
            castling: None,
            en_passant: None,
        };

        let mut piece_id = id;
        if self.board.side_to_move() == chess::Color::Black {
            piece_id += 10;
        }
        self.emit(PieceSignals::Remove {
            square: dest_sq.to_int(),
        });
        self.emit(PieceSignals::Place {
            id: piece_id,
            square: dest_sq.to_int(),
        });
        self.emit(Signals::Promoting { file: None });

        self.board.apply_move(legal_move);
    }
}

impl BoardConImpl {
    fn reset_highlights(&self) {
        self.emit(Signals::Phantom { id: None });

        self.emit(Signals::Hint {
            squares: Vec::new(),
        });
        self.emit(Signals::Capture {
            squares: Vec::new(),
        });
        self.emit(Signals::Highlight { square: None });
    }

    fn show_hints(&self, sq: chess::Square) {
        let (legal_capture_vec, legal_move_vec): (Vec<LegalMove>, Vec<LegalMove>) = self
            .board
            .moves_of(sq)
            .into_iter()
            .partition(|m| m.capture.is_some());
        let legal_move_vec = legal_move_vec
            .into_iter()
            .map(|m| m.dest().to_int())
            .collect::<Vec<u8>>();
        let legal_capture_vec = legal_capture_vec
            .into_iter()
            .map(|m| m.dest().to_int())
            .collect::<Vec<u8>>();
        self.emit(Signals::Hint {
            squares: legal_move_vec,
        });
        self.emit(Signals::Capture {
            squares: legal_capture_vec,
        });
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

    fn check_promoting(&mut self) -> bool {
        if let Some((src_sq, dest_dq)) = self.promotion_squares.take() {
            self.emit(Signals::Promoting { file: None });
            self.emit(PieceSignals::Move {
                src_square: dest_dq.to_int(),
                dest_square: src_sq.to_int(),
            });

            if let Some(piece) = self.board.piece_on(dest_dq) {
                // Captured piece
                self.emit(PieceSignals::Place {
                    id: piece.piece_id(),
                    square: dest_dq.to_int(),
                });
            }

            return true;
        }

        false
    }

    // The moved piece has to be placed before calling this method.
    fn try_apply_move(&mut self, legal_move: LegalMove) {
        if let Some(promotion) = legal_move.inner.get_promotion() {
            // A promotion!
            let src_sq = legal_move.inner.get_source();
            let dest_sq = legal_move.inner.get_dest();
            let mut promotion_file = dest_sq.file();
            if self.board.side_to_move() == chess::Color::Black {
                promotion_file += 10;
            }

            self.promotion_squares.replace((src_sq, dest_sq));

            self.emit(Signals::Promoting {
                file: Some(promotion_file),
            });

            return;
        }

        self.emit(Signals::LastMove {
            src_square: Some(legal_move.src().to_int()),
            dest_square: Some(legal_move.dest().to_int()),
        });

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

        self.board.apply_move(legal_move);
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
