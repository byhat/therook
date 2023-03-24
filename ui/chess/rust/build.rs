use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new().file("src/board_con/mod.rs").build();
}
