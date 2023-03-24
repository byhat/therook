import QtQuick

Square {
    z: 1

    id: hoverRect

    visible: false

    Rectangle {
        anchors.fill: parent

        border.color: "white"
        border.width: size / 16
        color: "transparent"
        opacity: 0.6
    }

    function show(square) {
        hoverRect.square = square;
        hoverRect.visible = true;
    }

    function hide() {
        hoverRect.visible = false;
    }
}
