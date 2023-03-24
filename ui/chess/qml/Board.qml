import QtQuick

Item {
    readonly property int boardSize: pieceSize * 8
    readonly property int pieceSize: Math.floor(Math.min(width, height) / 8)

    id: canvas

    BoardImpl {
        anchors.centerIn: parent

        width: canvas.boardSize
        height: canvas.boardSize

        id: board

        pieceSize: canvas.pieceSize
    }

    Component.onCompleted: {
    }
}
