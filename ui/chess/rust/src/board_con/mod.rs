mod p;

#[cxx_qt::bridge(cxx_file_stem = "board_con")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qsignals(BoardCon)]
    pub enum Signals {
        ResetBoard,
        PlacePiece { id: u8, square: u8 },
        MovePiece { src_square: u8, dest_square: u8 },
        RemovePiece { square: u8 },

        PlaceHint { square: u8 },
        CaptureHint { square: u8 },
        ResetHints,

        ShowPhantomPiece { id: u8, x: f32, y: f32 },
        UpdatePhantomPiece { x: f32, y: f32 },
        HidePhantomPiece,

        ShowHighlightRect { square: u8 },
        HideHighlightRect,

        ShowHoverRect { square: u8 },
        HideHoverRect,
    }

    use super::p;

    #[cxx_qt::qobject(qml_uri = "fr.therook.ui", qml_version = "1.0")]
    pub struct BoardCon {
        #[qproperty]
        piece_size: u32,

        tx: Option<std::sync::mpsc::Sender<p::Slots>>,
    }

    impl Default for BoardCon {
        fn default() -> Self {
            Self {
                piece_size: 8,

                tx: None,
            }
        }
    }

    impl qobject::BoardCon {
        #[qinvokable]
        pub fn initialize(self: Pin<&mut Self>) {
            let (slots_tx, slots_rx) = std::sync::mpsc::channel::<p::Slots>();

            let (mut _impl, signals_rx) = p::BoardConImpl::from_rx(slots_rx);

            std::thread::spawn(move || {
                println!("spawning qobj->rust thread...");

                _impl.run();

                println!("exiting qobj->rust thread...");
            });

            let qt_thread = self.qt_thread();

            std::thread::spawn(move || {
                println!("spawning rust->qobj thread...");

                while let Ok(signal) = signals_rx.recv() {
                    qt_thread
                        .queue(move |mut qobject| qobject.as_mut().emit(signal.into()))
                        .unwrap();
                }

                println!("exiting rust->qobj thread...");
            });

            self.set_tx(Some(slots_tx));
        }

        pub fn try_tx(&self) -> &Option<std::sync::mpsc::Sender<p::Slots>> {
            if self.tx().is_none() {
                println!("backend uninitialized!!");
            }

            &self.tx()
        }

        #[qinvokable]
        pub fn resync_board(&self) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::Resync).unwrap()
            }
        }

        #[qinvokable]
        pub fn coord_clicked(&self, x: f32, y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::MouseEventSlots::Clicked { x, y },
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }

        #[qinvokable]
        pub fn coord_drag_started(&self, src_x: f32, src_y: f32, dest_x: f32, dest_y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::MouseEventSlots::Drag(p::DragSlots::Started {
                        src_x,
                        src_y,
                        dest_x,
                        dest_y,
                    }),
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }

        #[qinvokable]
        pub fn coord_drag_event(&self, x: f32, y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::MouseEventSlots::Drag(p::DragSlots::Updated { x, y }),
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }

        #[qinvokable]
        pub fn coord_drag_ended(&self, src_x: f32, src_y: f32, dest_x: f32, dest_y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::MouseEventSlots::Drag(p::DragSlots::Ended {
                        src_x,
                        src_y,
                        dest_x,
                        dest_y,
                    }),
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }
    }
}

#[allow(clippy::all)]
impl From<p::Signals> for Signals {
    fn from(signals: p::Signals) -> Self {
        match signals {
            p::Signals::Board(board) => match board {
                p::BoardSignals::Reset => Signals::ResetBoard,
            },
            p::Signals::Piece(piece) => match piece {
                p::PieceSignals::Place { id, square } => Signals::PlacePiece { id, square },
                p::PieceSignals::Move {
                    src_square,
                    dest_square,
                } => Signals::MovePiece {
                    src_square,
                    dest_square,
                },
                p::PieceSignals::Remove { square } => Signals::RemovePiece { square },
            },
            p::Signals::Hint(hint) => match hint {
                p::HintSignals::Place { square } => Signals::PlaceHint { square },
                p::HintSignals::Capture { square } => Signals::CaptureHint { square },
                p::HintSignals::Reset => Signals::ResetHints,
            },
            p::Signals::Phantom(phantom) => match phantom {
                p::PhantomPieceSignals::Show { id, x, y } => Signals::ShowPhantomPiece { id, x, y },
                p::PhantomPieceSignals::Update { x, y } => Signals::UpdatePhantomPiece { x, y },
                p::PhantomPieceSignals::Hide => Signals::HidePhantomPiece,
            },
            p::Signals::Highlight(highlight) => match highlight {
                p::HighlightRectSignals::Show { square } => Signals::ShowHighlightRect { square },
                p::HighlightRectSignals::Hide => Signals::HideHighlightRect,
            },
            p::Signals::Hover(hover) => match hover {
                p::HoverRectSignals::Show { square } => Signals::ShowHoverRect { square },
                p::HoverRectSignals::Hide => Signals::HideHoverRect,
            },
        }
    }
}
