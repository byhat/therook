mod p;

#[cxx_qt::bridge(cxx_file_stem = "board_con")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvector.h");
        type QVector_u8 = cxx_qt_lib::QVector<u8>;
    }

    #[cxx_qt::qsignals(BoardCon)]
    pub enum Signals {
        ResetBoard,

        PlacePiece { id: u8, square: u8 },
        MovePiece { src_square: u8, dest_square: u8 },
        RemovePiece { square: u8 },
    }

    use super::p;

    #[cxx_qt::qobject(qml_uri = "fr.therook.ui", qml_version = "1.0")]
    pub struct BoardCon {
        #[qproperty]
        piece_size: u32,

        #[qproperty]
        phantom_id: i8,

        #[qproperty]
        hint_sq: QVector_u8,
        #[qproperty]
        capture_sq: QVector_u8,

        #[qproperty]
        highlight_sq: i8,
        #[qproperty]
        last_src_sq: i8,
        #[qproperty]
        last_dest_sq: i8,

        tx: Option<std::sync::mpsc::Sender<p::Slots>>,
    }

    impl Default for BoardCon {
        fn default() -> Self {
            Self {
                piece_size: 8,

                phantom_id: -1,

                hint_sq: QVector_u8::default(),
                capture_sq: QVector_u8::default(),

                highlight_sq: -1,
                last_src_sq: -1,
                last_dest_sq: -1,

                tx: None,
            }
        }
    }

    impl qobject::BoardCon {
        pub fn handle_signal(mut self: Pin<&mut Self>, signal: p::Signals) {
            struct OptionU8(Option<u8>);

            impl Into<i8> for OptionU8 {
                fn into(self) -> i8 {
                    self.0.map(|v| v as i8).unwrap_or(-1)
                }
            }

            match signal {
                p::Signals::Reset => self.as_mut().emit(Signals::ResetBoard),
                p::Signals::Piece(piece) => match piece {
                    p::PieceSignals::Place { id, square } => {
                        self.as_mut().emit(Signals::PlacePiece { id, square })
                    }
                    p::PieceSignals::Move {
                        src_square,
                        dest_square,
                    } => self.as_mut().emit(Signals::MovePiece {
                        src_square,
                        dest_square,
                    }),
                    p::PieceSignals::Remove { square } => {
                        self.as_mut().emit(Signals::RemovePiece { square })
                    }
                },
                p::Signals::Phantom { id } => self.set_phantom_id(OptionU8(id).into()),
                p::Signals::Hint { squares } => self.set_hint_sq(squares.into()),
                p::Signals::Capture { squares } => self.set_capture_sq(squares.into()),
                p::Signals::Highlight { square } => self.set_highlight_sq(OptionU8(square).into()),
                p::Signals::LastMove {
                    src_square,
                    dest_square,
                } => {
                    self.as_mut().set_last_src_sq(OptionU8(src_square).into());
                    self.as_mut().set_last_dest_sq(OptionU8(dest_square).into());
                }
            }
        }

        #[qinvokable]
        pub fn initialize(self: Pin<&mut Self>) {
            let (slots_tx, slots_rx) = std::sync::mpsc::channel::<p::Slots>();

            let (mut _impl, signals_rx) = p::BoardConImpl::from_rx(slots_rx);

            std::thread::spawn(move || _impl.run());

            let qt_thread = self.qt_thread();

            std::thread::spawn(move || {
                while let Ok(signal) = signals_rx.recv() {
                    qt_thread
                        .queue(move |qt_object| qt_object.handle_signal(signal))
                        .unwrap();
                }
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
        pub fn coord_drag_started(&self, src_x: f32, src_y: f32, _dest_x: f32, _dest_y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::DragSlots::Started { src_x, src_y }.into(),
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }

        #[qinvokable]
        pub fn coord_drag_ended(&self, _src_x: f32, _src_y: f32, dest_x: f32, dest_y: f32) {
            if let Some(tx) = self.try_tx() {
                tx.send(p::Slots::MouseEvent {
                    slot: p::DragSlots::Ended { dest_x, dest_y }.into(),
                    piece_size: *self.piece_size(),
                })
                .unwrap()
            }
        }
    }
}
