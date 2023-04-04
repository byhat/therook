use uuid::Uuid;

use sac::Position;

#[derive(Debug, Clone)]
pub struct PieceAtSquare {
    pub square: sac::Square,
    pub piece: sac::Piece,
}

impl PieceAtSquare {
    // For some reason shakmaty::Role starts at 1.
    pub fn piece_id(&self) -> u8 {
        if self.piece.color == sac::Color::White {
            u8::from(self.piece.role) - 1
        } else {
            u8::from(self.piece.role) + 9
        }
    }
}

pub struct BoardImpl {
    inner: sac::Game,
    cur_node: Uuid,
}

impl Default for BoardImpl {
    fn default() -> Self {
        let inner = sac::Game::default();
        let cur_node = inner.root();

        Self { inner, cur_node }
    }
}

impl BoardImpl {
    fn cur_board(&self) -> sac::Chess {
        self.inner.board_at(self.cur_node).unwrap()
    }

    pub fn turn(&self) -> sac::Color {
        self.cur_board().turn()
    }

    pub fn pieces(&self) -> Vec<PieceAtSquare> {
        let bb = self.cur_board().board().clone();

        let mut piece_vec: Vec<PieceAtSquare> = Vec::new();
        for (square, piece) in bb {
            piece_vec.push(PieceAtSquare { square, piece });
        }

        piece_vec
    }

    pub fn piece_on(&self, square: sac::Square) -> Option<PieceAtSquare> {
        let piece = self.cur_board().board().piece_at(square)?;
        Some(PieceAtSquare { square, piece })
    }

    pub fn moves_of(&self, src: sac::Square) -> Vec<sac::Move> {
        let mut move_vec: Vec<sac::Move> = self
            .cur_board()
            .legal_moves()
            .into_iter()
            .collect::<Vec<sac::Move>>();
        move_vec.retain(|m| m.from().unwrap() == src);

        move_vec
    }

    pub fn apply_move(&mut self, m: sac::Move) {
        let new_node = if let Some(val) = self.inner.add_node(self.cur_node, m) {
            val
        } else {
            println!("apply_move failed");
            return;
        };

        self.cur_node = new_node;
    }
}
