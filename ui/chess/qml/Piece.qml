import QtQuick

Square {
    property alias pieceId: basePiece.pieceId
    property alias sourceSize: basePiece.sourceSize
    property bool animationEnabled: false

    readonly property int transitionDuration: 150

    z: 10

    id: piece_canvas

    BasePiece {
        anchors.fill: parent

        id: basePiece
    }

    Behavior on x {
        enabled: animationEnabled

        PropertyAnimation {
            duration: transitionDuration
        }
    }
    Behavior on y {
        enabled: animationEnabled

        PropertyAnimation {
            duration: transitionDuration
        }
    }

    MouseArea {
        anchors.fill: parent

        cursorShape: Qt.DragMoveCursor

        enabled: false
    }
}
