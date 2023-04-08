import QtQuick

Image {
    required property int pieceSize

    readonly property int fileRankLabelSize: pieceSize < 1 ? 4 : pieceSize / 8

    readonly property color primary: "#eae9d2"
    readonly property color secondary: "#4b7399"
    readonly property int padding: 4

    id: root

    source: "qrc:/chess/blue.svg"

    smooth: false
    cache: false
    asynchronous: true

    Item {
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: root.pieceSize

        Repeater {
            model: ["A", "B", "C", "D", "E", "F", "G", "H"]

            Text {
                x: root.pieceSize * (index + 1) - width - root.padding
                y: root.pieceSize - height - root.padding

                text: modelData
                color: (index % 2) ? root.secondary : root.primary
                font.bold: true
                font.pointSize: root.fileRankLabelSize
            }
        }
    }

    Item {
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        width: root.pieceSize

        Repeater {
            model: ["1", "2", "3", "4", "5", "6", "7", "8"]

            Text {
                x: root.padding
                y: root.pieceSize * index + root.padding

                text: modelData
                color: (index % 2) ? root.primary : root.secondary
                font.bold: true
                font.pointSize: root.fileRankLabelSize
            }
        }
    }
}
