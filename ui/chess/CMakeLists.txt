qt_add_library(ui_chess STATIC)

add_subdirectory(assets)

set_target_properties(ui_chess PROPERTIES AUTOMOC ON)

find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
            Corrosion
            GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
            GIT_TAG v0.3.1
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE therook-ui-chess)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})

set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
        "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
        "QMAKE=${QMAKE}"
        )

add_library(ui_chess_lib INTERFACE)
target_include_directories(ui_chess_lib INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")

target_link_libraries(ui_chess_lib INTERFACE
        # WHOLE_ARCHIVE is needed for the generated QML plugin to register on startup,
        # otherwise the linker will discard the static variables that initialize it.
        "$<LINK_LIBRARY:WHOLE_ARCHIVE,${CRATE}-static>"
        Qt6::Core
        Qt6::Quick
        )

target_link_libraries(ui_chess PRIVATE Qt6::Quick ui_chess_lib)

qt_add_qml_module(ui_chess
        URI ui.chess
        VERSION 1.0
        RESOURCE_PREFIX /imports
        QML_FILES

        qml/Square.qml
        qml/BasePiece.qml

        qml/HighlightRect.qml
        qml/HoverRect.qml
        qml/HintRect.qml
        qml/CaptureHintRect.qml

        qml/Piece.qml
        qml/PhantomPiece.qml

        qml/PromotionWindow.qml
        qml/DragArea.qml
        qml/BoardBackground.qml

        qml/BoardImpl.qml
        qml/Board.qml

        js/BoardView.mjs

        )
