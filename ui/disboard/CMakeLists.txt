qt_add_library(disboard STATIC)

add_subdirectory(assets)

set_target_properties(disboard PROPERTIES AUTOMOC ON)

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

set(CRATE disboard-rust)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})

set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
        "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
        "QMAKE=${QMAKE}"
        )

add_library(librustdisboard INTERFACE)
target_include_directories(librustdisboard INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")

target_link_libraries(librustdisboard INTERFACE
        # WHOLE_ARCHIVE is needed for the generated QML plugin to register on startup,
        # otherwise the linker will discard the static variables that initialize it.
        "$<LINK_LIBRARY:WHOLE_ARCHIVE,${CRATE}-static>"
        Qt6::Core
        Qt6::Quick
        )

target_link_libraries(disboard PRIVATE Qt6::Quick librustdisboard)

qt_add_qml_module(disboard
        URI ui.disboard
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
