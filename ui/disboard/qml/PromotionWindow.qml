import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Controls.Material.impl

Pane {
    required property int pieceSize
    required property int file
    required property bool side // 0: bottom, 1: top
    required property bool isWhite

    signal selected(int piece)

    width: pieceSize
    height: pieceSize * 4
    x: pieceSize * file
    y: side ? 0 : pieceSize * 4
    z: 20
    Material.elevation: 6

    id: promotionWindow

    padding: 0

    contentItem: Column {
        spacing: 0

        Repeater {
            model: promotionWindow.side ? [4, 1, 3, 2] : [2, 3, 1, 4]

            ItemDelegate {
                width: parent.width
                height: parent.width

                padding: 4

                contentItem: BasePiece {
                    pieceId: promotionWindow.isWhite ? modelData : modelData + 10
                    sourceSize: width
                }

                onClicked: promotionWindow.selected(modelData)
            }
        }
    }
}
