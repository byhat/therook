import QtQuick

Square {
    z: 11

    id: legal_capture_canvas

    Rectangle {
        anchors.fill: parent

        radius: (width / 2) - 1
        border.width: width / 8
        border.color: "#000000"
        color: "transparent"
        opacity: 0.2
    }
}
