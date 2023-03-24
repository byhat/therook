import QtQuick

Square {
    z: 11

    id: legal_move_canvas

    Rectangle {
        anchors.centerIn: parent

        width: parent.width / 3
        height: parent.height / 3
        radius: width / 2

        color: "#000000"
        opacity: 0.2
    }
}
