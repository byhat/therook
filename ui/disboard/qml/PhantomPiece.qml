import QtQuick

BasePiece {
    property real centerX: 0
    property real centerY: 0

    required property int size

    width: size
    height: size
    x: centerX - size / 2
    y: centerY - size / 2

    id: phantomPiece

    visible: false

    pieceId: 0
}
