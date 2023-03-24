import QtQuick

Square {
    z: 0

    id: highlightRect

    visible: false

    Rectangle {
        anchors.fill: parent

        color: "#00A5FF"
        opacity: 0.5
    }

    function show(square) {
        highlightRect.square = square;
        highlightRect.visible = true;
    }

    function hide() {
        highlightRect.visible = false;
    }
}
