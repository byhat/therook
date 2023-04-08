qt_add_library(ui STATIC)

qt_add_resources(ui "conf"
        PREFIX "/"
        FILES
        qtquickcontrols2.conf
        )

set_target_properties(ui PROPERTIES AUTOMOC ON)
target_link_libraries(
        ui PRIVATE Qt6::Quick Qt6::Svg
)

qt_add_qml_module(ui
        URI ui
        VERSION 1.0
        RESOURCE_PREFIX /imports
        QML_FILES
        RoundPane.qml
        PlayerInfo.qml
        EvalBar.qml

        SidePanel.qml

        main.qml
        )
