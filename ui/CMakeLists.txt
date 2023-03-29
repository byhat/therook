qt_add_library(ui STATIC)

qt_add_resources(ui "conf"
    PREFIX "/"
    FILES
        qtquickcontrols2.conf
)

add_subdirectory(chess)

set_target_properties(ui PROPERTIES AUTOMOC ON)
target_link_libraries(
    ui PRIVATE Qt6::Quick Qt6::Svg
    ui_chessplugin
)

qt_add_qml_module(ui
    URI ui
    VERSION 1.0
    RESOURCE_PREFIX /imports
    QML_FILES
        qml/RoundPane.qml
        qml/PlayerInfo.qml
        qml/EvalBar.qml

        qml/SidePanel.qml

        main.qml
)
