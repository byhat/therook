import QtQuick

Item {
    required property int pieceId
    required property int sourceSize

    QtObject {
        readonly property bool isWhite: pieceId < 10
        readonly property int normPieceId: isWhite ? pieceId : pieceId - 10

        readonly property var uriMapping: ["P", "N", "B", "R", "Q", "K"]
        readonly property var uriName: (isWhite ? "w" : "b") + uriMapping[normPieceId]

        readonly property string pieceUri: "qrc:/chess/pieces/" + uriName + ".svg"

        id: inner
    }

    Image {
        anchors.fill: parent

        sourceSize.width: parent.sourceSize
        sourceSize.height: parent.sourceSize

        source: inner.pieceUri

        fillMode: Image.Pad
        smooth: false
        cache: false
        asynchronous: true
    }
}
