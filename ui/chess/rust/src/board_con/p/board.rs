pub trait SquareExt {
    fn file(&self) -> u8;
    fn rank(&self) -> u8;
}

impl SquareExt for chess::Square {
    fn file(&self) -> u8 {
        self.get_file().to_index() as u8
    }

    fn rank(&self) -> u8 {
        self.get_rank().to_index() as u8
    }
}

#[derive(Debug, Clone)]
pub struct PieceAtSquare {
    pub square: chess::Square,
    pub piece: chess::Piece,
    pub color: chess::Color,
}

impl PieceAtSquare {
    pub fn piece_id(&self) -> u8 {
        if self.color == chess::Color::White {
            self.piece.to_index() as u8
        } else {
            self.piece.to_index() as u8 + 10
        }
    }
}

#[derive(Debug, Clone)]
pub struct LegalMove {
    pub inner: chess::ChessMove,

    pub capture: Option<chess::Square>, // En passant does not count as capture

    pub castling: Option<(chess::Square, chess::Square)>,
    pub en_passant: Option<chess::Square>,
}

impl LegalMove {
    pub fn src(&self) -> chess::Square {
        self.inner.get_source()
    }

    pub fn dest(&self) -> chess::Square {
        self.inner.get_dest()
    }
}

#[derive(Default)]
pub struct BoardImpl {
    inner: chess::Board,
}

impl BoardImpl {
    pub fn side_to_move(&self) -> chess::Color {
        self.inner.side_to_move()
    }

    pub fn pieces(&self) -> Vec<PieceAtSquare> {
        let bb = *self.inner.combined();

        let mut piece_vec: Vec<PieceAtSquare> = Vec::new();
        for square in bb {
            let piece = self.inner.piece_on(square).unwrap();
            let color = self.inner.color_on(square).unwrap();

            piece_vec.push(PieceAtSquare {
                square,
                piece,
                color,
            });
        }

        piece_vec
    }

    pub fn piece_on(&self, square: chess::Square) -> Option<PieceAtSquare> {
        let piece = self.inner.piece_on(square)?;
        let color = self.inner.color_on(square)?;

        Some(PieceAtSquare {
            square,
            piece,
            color,
        })
    }

    pub fn moves_of(&self, src: chess::Square) -> Vec<LegalMove> {
        let mut iterable = chess::MoveGen::new_legal(&self.inner);

        let mut move_vec: Vec<LegalMove> = Vec::new();
        for chess_move in &mut iterable {
            if chess_move.get_source() == src {
                let dest = chess_move.get_dest();

                let mut capture: Option<chess::Square> = None;
                if let Some(piece) = self.piece_on(dest) {
                    if piece.color != self.inner.side_to_move() {
                        // Opponent's piece
                        capture.replace(dest);
                    }
                }

                let mut castling: Option<(chess::Square, chess::Square)> = None;
                if self.piece_on(src).unwrap().piece == chess::Piece::King {
                    // A king move
                    if src.file().abs_diff(dest.file()) == 2 {
                        // it moved 2 squares
                        if dest.get_file() == chess::File::G {
                            // Kingside castling
                            castling.replace((
                                chess::Square::make_square(src.get_rank(), chess::File::H),
                                chess::Square::make_square(src.get_rank(), chess::File::F),
                            ));
                        } else if dest.get_file() == chess::File::C {
                            castling.replace((
                                chess::Square::make_square(src.get_rank(), chess::File::A),
                                chess::Square::make_square(src.get_rank(), chess::File::D),
                            ));
                        }
                    }
                }

                let mut en_passant: Option<chess::Square> = None;
                if self.piece_on(src).unwrap().piece == chess::Piece::Pawn {
                    // A pawn
                    if src.file() != dest.file() {
                        // it captured something
                        if self.piece_on(dest).is_none() {
                            // holy hell
                            en_passant.replace(chess::Square::make_square(
                                src.get_rank(),
                                dest.get_file(),
                            ));
                        }
                    }
                }

                move_vec.push(LegalMove {
                    inner: chess_move,
                    capture,
                    castling,
                    en_passant,
                });
            }
        }

        move_vec
    }

    pub fn apply_move(&mut self, chess_move: LegalMove) {
        self.inner = self.inner.make_move_new(chess_move.inner);
    }
}
