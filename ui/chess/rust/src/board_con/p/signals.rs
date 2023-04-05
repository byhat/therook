pub enum PieceSignals {
    Place { id: u8, square: u8 },
    Move { src_square: u8, dest_square: u8 },
    Remove { square: u8 },
}

impl From<PieceSignals> for Signals {
    fn from(signals: PieceSignals) -> Self {
        Signals::Piece(signals)
    }
}

pub enum Signals {
    Reset {
        initial: Vec<(u8, u8)>,
    },

    Piece(PieceSignals),

    Phantom {
        id: Option<u8>,
    },

    Hint {
        squares: Vec<u8>,
    },
    Capture {
        squares: Vec<u8>,
    },

    Highlight {
        square: Option<u8>,
    },
    LastMove {
        src_square: Option<u8>,
        dest_square: Option<u8>,
    },

    Promoting {
        file: Option<u8>,
    },

    PGN {
        string: String,
    },
}
