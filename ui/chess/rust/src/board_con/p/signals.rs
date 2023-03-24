pub enum BoardSignals {
    Reset,
}

impl From<BoardSignals> for Signals {
    fn from(signals: BoardSignals) -> Self {
        Signals::Board(signals)
    }
}

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

pub enum HintSignals {
    Place { square: u8 },
    Capture { square: u8 },
    Reset,
}

impl From<HintSignals> for Signals {
    fn from(signals: HintSignals) -> Self {
        Signals::Hint(signals)
    }
}

pub enum PhantomPieceSignals {
    Show { id: u8, x: f32, y: f32 },
    Update { x: f32, y: f32 },
    Hide,
}

impl From<PhantomPieceSignals> for Signals {
    fn from(signals: PhantomPieceSignals) -> Self {
        Signals::Phantom(signals)
    }
}

pub enum HighlightRectSignals {
    Show { square: u8 },
    Hide,
}

impl From<HighlightRectSignals> for Signals {
    fn from(signals: HighlightRectSignals) -> Self {
        Signals::Highlight(signals)
    }
}

pub enum HoverRectSignals {
    Show { square: u8 },
    Hide,
}

impl From<HoverRectSignals> for Signals {
    fn from(signals: HoverRectSignals) -> Self {
        Signals::Hover(signals)
    }
}

pub enum Signals {
    Board(BoardSignals),
    Piece(PieceSignals),
    Hint(HintSignals),
    Phantom(PhantomPieceSignals),
    Highlight(HighlightRectSignals),
    Hover(HoverRectSignals),
}
