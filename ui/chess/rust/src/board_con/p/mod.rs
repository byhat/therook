mod signals;
mod slots;

pub use signals::*;
pub use slots::*;

use std::sync::mpsc::{Receiver, Sender};

use sac::Position;
use uuid::Uuid;

mod util {
    pub fn piece_to_u8(piece: &sac::Piece) -> u8 {
        if piece.color == sac::Color::White {
            u8::from(piece.role) - 1
        } else {
            u8::from(piece.role) + 9
        }
    }

    pub fn coord_to_square(x: f32, y: f32, piece_size: u32) -> sac::Square {
        let file: u32 = x as u32 / piece_size;
        let rank: u32 = 7 - (y as u32 / piece_size);

        sac::Square::from_coords(sac::File::new(file), sac::Rank::new(rank))
    }

    pub fn squares_from_move(m: &sac::Move) -> (sac::Square, sac::Square) {
        let src_sq = m.from().unwrap();
        let mut dest_sq = m.to();

        if let sac::Move::Castle { king: _, rook: _ } = m {
            let castling_side = m.castling_side().unwrap();

            let to_rank = m.from().unwrap().rank();
            let to_file = castling_side.king_to_file();

            dest_sq = sac::Square::from_coords(to_file, to_rank);
        }

        (src_sq, dest_sq)
    }
}

pub struct BoardConImpl {
    tx: std::sync::mpsc::Sender<Signals>,
    rx: std::sync::mpsc::Receiver<Slots>,

    game: sac::Game,
    cur_node: Uuid,

    dragged_piece: Option<(sac::Square, sac::Piece)>,
    highlighted_square: Option<sac::Square>,
    promotion: Option<sac::Move>,
}

pub trait PipelinedBackend {
    type Signals;
    type Slots;

    fn from_rx(
        signals_tx: std::sync::mpsc::Sender<Self::Signals>,
        slots_rx: std::sync::mpsc::Receiver<Self::Slots>,
    ) -> Self;

    fn recv(&self) -> Option<Self::Slots>;
    fn handle_slot(&mut self, slot: Self::Slots);
    fn emit(&self, signal: impl Into<Self::Signals>);

    fn run(&mut self) {
        while let Some(slot) = self.recv() {
            self.handle_slot(slot);
        }
    }
}

impl PipelinedBackend for BoardConImpl {
    type Signals = Signals;
    type Slots = Slots;

    fn from_rx(signals_tx: Sender<Self::Signals>, slots_rx: Receiver<Self::Slots>) -> Self {
        let game = sac::Game::default();
        let cur_node = game.root();

        Self {
            tx: signals_tx,
            rx: slots_rx,

            game,
            cur_node,

            dragged_piece: None,
            highlighted_square: None,
            promotion: None,
        }
    }

    fn recv(&self) -> Option<Self::Slots> {
        self.rx.recv().ok()
    }

    fn handle_slot(&mut self, slot: Self::Slots) {
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
            Slots::Traverse { forward } => {
                if forward {
                    self.to_next_mainline_node()
                } else {
                    self.to_prev_node()
                }
            }
        }
    }

    fn emit(&self, signal: impl Into<Self::Signals>) {
        self.tx.send(signal.into()).unwrap();
    }
}

impl BoardConImpl {
    pub fn resync_board(&self) {
        let board = self.position().board().clone();

        let mut initial: Vec<(u8, u8)> = Vec::new();
        for (square, piece) in board {
            initial.push((square.into(), util::piece_to_u8(&piece)));
        }

        self.emit(Signals::Reset { initial });
    }

    pub fn coord_clicked(&mut self, x: f32, y: f32, piece_size: u32) {
        self.reset_highlights();
        let highlighted_square = self.highlighted_square.take();

        if self.cancel_promotion() {
            return;
        }

        let sq = util::coord_to_square(x, y, piece_size);
        if let Some(src_sq) = highlighted_square {
            if src_sq == sq {
                // Clicked the same square twice
                return;
            }

            if let Some(m) = self.legal_move(src_sq, sq) {
                // A legal move!

                let (src_sq, dest_sq) = util::squares_from_move(&m);

                self.emit(PieceSignals::Move {
                    src_square: src_sq.into(),
                    dest_square: dest_sq.into(),
                });

                self.try_apply_move(m);

                return;
            }
        }

        // No new piece selected
        if self.position().board().clone().piece_at(sq).is_none() {
            return;
        }

        self.highlighted_square.replace(sq);
        self.emit(Signals::Highlight {
            square: Some(sq.into()),
        });

        self.show_hints(sq);
    }

    pub fn coord_drag_started(&mut self, src_x: f32, src_y: f32, piece_size: u32) {
        self.reset_highlights();
        self.highlighted_square.take();

        if self.cancel_promotion() {
            return;
        }

        let sq = util::coord_to_square(src_x, src_y, piece_size);

        let piece = if let Some(val) = self.position().board().clone().piece_at(sq) {
            val
        } else {
            return;
        };

        self.highlighted_square.replace(sq);

        self.dragged_piece.replace((sq, piece));

        {
            let square: u8 = sq.into();
            self.emit(PieceSignals::Remove { square });
            self.emit(Signals::Highlight {
                square: Some(square),
            });
        }

        self.emit(Signals::Phantom {
            id: Some(util::piece_to_u8(&piece)),
        });

        self.show_hints(sq);
    }

    pub fn coord_drag_ended(&mut self, dest_x: f32, dest_y: f32, piece_size: u32) {
        {
            self.emit(Signals::Phantom { id: None });
        }

        let (src_sq, piece) = if let Some(val) = self.dragged_piece.take() {
            val
        } else {
            return;
        };

        let dest_sq = util::coord_to_square(dest_x, dest_y, piece_size);

        let m = if let Some(val) = self.legal_move(src_sq, dest_sq) {
            val
        } else {
            self.emit(PieceSignals::Place {
                id: util::piece_to_u8(&piece),
                square: src_sq.into(),
            });
            return;
        };
        let (_, dest_sq) = util::squares_from_move(&m);

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
            id: util::piece_to_u8(&piece),
            square: dest_sq.into(),
        });

        self.try_apply_move(m);
    }

    pub fn finalize_promotion(&mut self, id: u8) {
        if self.promotion.is_none() {
            println!("nothing to promote");
            return;
        }

        let promotion_move = self.promotion.take().unwrap();
        let role = sac::Role::ALL[id as usize];
        let promotion_move = sac::Move::Normal {
            role: sac::Role::Pawn, // duh
            from: promotion_move.from().unwrap(),
            capture: promotion_move.capture(),
            to: promotion_move.to(),
            promotion: Some(role),
        };

        let mut piece_id = id;
        if self.position().turn() == sac::Color::Black {
            piece_id += 10;
        }
        self.emit(PieceSignals::Place {
            id: piece_id,
            square: promotion_move.to().into(),
        });
        self.emit(Signals::Promoting { file: None });

        self.apply_move(promotion_move);
    }

    // The moved piece has to be placed before calling this method.
    fn try_apply_move(&mut self, m: sac::Move) {
        if let Some(_) = m.promotion() {
            // A promotion!

            let mut promotion_file: u8 = m.to().file().into();
            if self.position().turn() == sac::Color::Black {
                promotion_file += 10;
            }

            self.promotion.replace(sac::Move::Normal {
                role: sac::Role::Pawn,
                from: m.from().unwrap(),
                capture: m.capture(),
                to: m.to(),
                promotion: None, // clear promotion role
            });

            self.emit(Signals::Promoting {
                file: Some(promotion_file),
            });

            return;
        }

        self.emit(Signals::LastMove {
            src_square: Some(m.from().unwrap().into()),
            dest_square: Some(m.to().into()),
        });

        if let sac::Move::EnPassant { from, to } = m {
            let ep_square = sac::Square::from_coords(to.file(), from.rank());
            self.emit(PieceSignals::Remove {
                square: ep_square.into(),
            });
        }

        // move the rook
        if let sac::Move::Castle { king: _, rook } = m {
            let castling_side = m.castling_side().unwrap();
            let from_rank = m.from().unwrap().rank();
            let to_file = castling_side.rook_to_file();

            self.emit(PieceSignals::Move {
                src_square: rook.into(),
                dest_square: sac::Square::from_coords(to_file, from_rank).into(),
            });
        }

        self.apply_move(m);
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

    fn show_hints(&self, sq: sac::Square) {
        let move_vec = self
            .position()
            .legal_moves()
            .into_iter()
            .filter(|v| v.from().unwrap() == sq)
            .collect::<Vec<sac::Move>>();

        let (castling_move_vec, move_vec): (Vec<sac::Move>, Vec<sac::Move>) =
            move_vec.into_iter().partition(|m| m.is_castle());

        let (capture_vec, move_vec): (Vec<sac::Move>, Vec<sac::Move>) =
            move_vec.into_iter().partition(|m| m.capture().is_some());
        let mut move_vec = move_vec
            .into_iter()
            .map(|m| m.to().into())
            .collect::<Vec<u8>>();
        let mut capture_vec = capture_vec
            .into_iter()
            .map(|m| m.to().into())
            .collect::<Vec<u8>>();

        for castling_move in castling_move_vec {
            if let sac::Move::Castle { king: _, rook } = castling_move {
                let castling_side = castling_move.castling_side().unwrap();

                let to_rank = castling_move.from().unwrap().rank();
                let to_file = castling_side.king_to_file();

                move_vec.push(sac::Square::from_coords(to_file, to_rank).into());

                capture_vec.push(rook.into());
            }
        }

        self.emit(Signals::Hint { squares: move_vec });
        self.emit(Signals::Capture {
            squares: capture_vec,
        });
    }

    fn cancel_promotion(&mut self) -> bool {
        if let Some(promotion_move) = self.promotion.take() {
            self.emit(Signals::Promoting { file: None });
            self.emit(PieceSignals::Move {
                src_square: promotion_move.to().into(),
                dest_square: promotion_move.from().unwrap().into(),
            });

            if let Some(piece) = self
                .position()
                .board()
                .clone()
                .piece_at(promotion_move.to())
            {
                // Captured piece
                self.emit(PieceSignals::Place {
                    id: util::piece_to_u8(&piece),
                    square: promotion_move.to().into(),
                });
            }

            return true;
        }

        false
    }
}

trait CurPosition {
    fn position(&self) -> sac::Chess;

    fn legal_move(&self, src_sq: sac::Square, dest_sq: sac::Square) -> Option<sac::Move>;

    fn to_prev_node(&mut self);
    fn to_next_mainline_node(&mut self);

    fn apply_move(&mut self, m: sac::Move);
}

impl CurPosition for BoardConImpl {
    fn position(&self) -> sac::Chess {
        self.game
            .board_at(self.cur_node)
            .expect("faulty cur_node or board")
    }

    fn legal_move(&self, src_sq: sac::Square, dest_sq: sac::Square) -> Option<sac::Move> {
        let move_vec: Vec<sac::Move> = self
            .position()
            .legal_moves()
            .into_iter()
            .filter(|v| v.from().unwrap() == src_sq)
            .collect::<Vec<sac::Move>>();

        for m in move_vec {
            if let sac::Move::Castle { king, rook } = m {
                let castling_side = m.castling_side().unwrap();
                let to_file = castling_side.king_to_file();
                let to_king_sq = sac::Square::from_coords(to_file, king.rank());
                if to_king_sq == dest_sq {
                    return Some(m);
                }
                if rook == dest_sq {
                    return Some(m);
                }
                continue;
            }

            if m.to() == dest_sq {
                // A legal move!
                return Some(m);
            }
        }

        None
    }

    fn to_prev_node(&mut self) {
        let parent_node = if let Some(val) = self.game.parent(self.cur_node) {
            val
        } else {
            return;
        };
        self.cur_node = parent_node;

        self.resync_board();

        if let Some(m) = self.game.prev_move(self.cur_node) {
            self.emit(Signals::LastMove {
                src_square: Some(m.from().unwrap().into()),
                dest_square: Some(m.to().into()),
            });
        } else {
            self.emit(Signals::LastMove {
                src_square: None,
                dest_square: None,
            });
        }
    }

    fn to_next_mainline_node(&mut self) {
        let child_node = if let Some(val) = self.game.mainline(self.cur_node) {
            val
        } else {
            return;
        };
        self.cur_node = child_node;

        self.resync_board();

        if let Some(m) = self.game.prev_move(self.cur_node) {
            self.emit(Signals::LastMove {
                src_square: Some(m.from().unwrap().into()),
                dest_square: Some(m.to().into()),
            });
        }
    }

    fn apply_move(&mut self, m: sac::Move) {
        let new_node = if let Some(val) = self.game.add_node(self.cur_node, m) {
            val
        } else {
            println!("apply_move failed");
            return;
        };

        self.cur_node = new_node;

        // Update PGN
        self.emit(Signals::PGN {
            string: format!("{}", self.game),
        })
    }
}
