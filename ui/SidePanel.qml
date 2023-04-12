import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import QtQml.Models

import disboard.impl.controller

RoundPane {
    required property var controller

    id: root

    Material.elevation: 6

    radius: 8
    padding: 0

    contentItem: RowLayout {
        spacing: 0

        EvalBar {
            Layout.preferredWidth: 8
            Layout.fillHeight: true

            evaluation: -20
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 8

            spacing: 8

            PlayerInfo {
                Layout.fillWidth: true

                avatarUri: "qrc:/chess/pieces/bK.svg"
                username: "black"
                rating: 401
            }

            // TextArea {
            //     Layout.fillWidth: true
            //     Layout.fillHeight: true
            //
            //     id: pgn_viewer
            //
            //     placeholderText: "PGN"
            //     wrapMode: TextEdit.WordWrap
            //     readOnly: true
            // }

            TableView {
                Layout.fillWidth: true
                Layout.fillHeight: true

                id: tableView

                clip: true

                model: MoveListModel {
                    controller: root.controller
                }

                columnWidthProvider: function (col) {
                    return width / 2;
                }
                rowHeightProvider: function (row) {
                    return 48;
                }

                ScrollBar.vertical: ScrollBar {}

                delegate: ItemDelegate {
                    property var model2: model

                    required property bool selected
                    required property bool current

                    id: delegate

                    visible: model.node != null
                    highlighted: controller.curNode == model.node || row % 2 == 0

                    horizontalPadding: 12

                    contentItem: RowLayout {
                        Text {
                            Layout.alignment: Qt.AlignVCenter

                            text: model.node != null ? model.display : ""
                        }

                        ListView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true

                            visible: model2.variations != null
                            orientation: ListView.Horizontal
                            spacing: 1
                            clip: true

                            model: model2.variations

                            delegate: Control {
                                anchors.verticalCenter: parent.verticalCenter

                                padding: 4

                                contentItem: Text {
                                    text: modelData.display
                                    color: "#FFFFFF"
                                    font.pointSize: 8
                                }

                                background: Rectangle {
                                    color: Material.accent
                                }
                            }
                        }
                    }

                    onClicked: {
                        controller.curNode = model.node;
                    }
                }
            }

            PlayerInfo {
                Layout.fillWidth: true

                avatarUri: "qrc:/chess/pieces/wK.svg"
                username: "white"
                rating: 400
            }
        }
    }
}
