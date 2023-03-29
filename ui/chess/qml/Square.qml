import QtQuick

Item {
    property int square: -1
    required property int size

    readonly property int rank: square >= 0 ? square >> 3 : 0
    readonly property int file: square >= 0 ? square & 7 : 0

    visible: square >= 0

    width: size
    height: size
    x: file * size
    y: (7 - rank) * size
}
