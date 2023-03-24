import QtQuick

Item {
    property int square: 0
    property int size: 0

    readonly property int rank: square >> 3
    readonly property int file: square & 7

    width: size
    height: size
    x: file * size
    y: (7 - rank) * size
}
