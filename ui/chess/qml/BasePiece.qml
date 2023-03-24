import QtQuick

Item {
    property int pieceId: 0

    property int sourceSize: 0

    readonly property string pieceUri: "qrc:/chess/pieces/" + get_piece_uri(pieceId) + ".svg"

    function get_piece_uri(piece_num) {
        var is_white = piece_num < 10;
        if (!is_white) piece_num -= 10;

        var result = "";

        switch (piece_num) {
        case 0: result = "P"; break;
        case 1: result = "N"; break;
        case 2: result = "B"; break;
        case 3: result = "R"; break;
        case 4: result = "Q"; break;
        case 5: result = "K"; break;
        }

        return (is_white ? "w" : "b") + result;
    }

    Image {
        anchors.fill: parent

        sourceSize.width: parent.sourceSize
        sourceSize.height: parent.sourceSize
        fillMode: Image.Pad
        smooth: false

        source: pieceUri
        cache: false
        asynchronous: true
    }
}
