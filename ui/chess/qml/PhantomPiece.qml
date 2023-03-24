import QtQuick

BasePiece {
    property real centerX: 0
    property real centerY: 0

    property int size: 0

    width: size
    height: size
    x: centerX - size / 2
    y: centerY - size / 2
    z: 11

    id: phantomPiece

    visible: false

    pieceId: 0

    function show(id, x, y) {
        phantomPiece.pieceId = id;
        phantomPiece.centerX = x;
        phantomPiece.centerY = y;
        phantomPiece.visible = true;
    }

    function update(x, y) {
        phantomPiece.centerX = x;
        phantomPiece.centerY = y;
    }

    function hide() {
        phantomPiece.visible = false;
    }
}
