import QtQuick

Square {
    property int mouseX: 0
    property int mouseY: 0

    z: 1

    id: hoverRect

    square: (7 - Math.floor(mouseY / size)) * 8 + Math.floor(mouseX / size)

    Rectangle {
        anchors.fill: parent

        border.color: "white"
        border.width: size / 16
        color: "transparent"
        opacity: 0.6
    }
}
