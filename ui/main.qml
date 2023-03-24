import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material

import ui.chess

ApplicationWindow {
    id: window

    title: qsTr("The Rook")

    visible: true

    width: 900
    height: 600

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            Label {
                Layout.fillWidth: true
                elide: Label.ElideRight
                horizontalAlignment: Qt.AlignHCenter
                text: "Title"
                verticalAlignment: Qt.AlignVCenter
            }
            ToolButton {
                text: qsTr("⋮")

                onClicked: menu.open()

                Menu {
                    id: menu
                    MenuItem {
                        text: "About"
                    }
                    MenuItem {
                        text: "Exit"

                        onClicked: Qt.quit()
                    }
                }
            }
        }
    }

    RowLayout {
        anchors.fill: parent

        Board {
            Layout.preferredWidth: height
            Layout.fillHeight: true
            Layout.margins: 8
        }

        RoundPane {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 8

            Material.elevation: 6
            radius: 8
            padding: 16

            contentItem: ColumnLayout {
                Button {
                    text: "press!"
                }

                TextArea {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }
            }
        }
    }
}
