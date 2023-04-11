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
                selectionModel: ItemSelectionModel {}

                columnWidthProvider: function (col) {
                    return width / 2;
                }
                rowHeightProvider: function (row) {
                    return 48;
                }

                ScrollBar.vertical: ScrollBar {}

                delegate: ItemDelegate {
                    required property bool selected
                    required property bool current

                    id: delegate

                    visible: model.node != null
                    highlighted: row % 2 == 0

                    text: model.node != null ? model.move : ""
                    onClicked: {
                        const idx = tableView.index(row, column);
                        tableView.selectionModel.setCurrentIndex(idx, ItemSelectionModel.NoUpdate);
                    }

                    Rectangle {
                        anchors.horizontalCenter: parent.horizontalCenter
                        anchors.bottom: parent.bottom
                        anchors.bottomMargin: 4

                        width: parent.width * 0.8
                        height: 2

                        visible: delegate.current
                        color: palette.buttonText
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
