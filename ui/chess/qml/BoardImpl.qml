import QtQuick
import fr.therook.ui

import "../js/BoardView.mjs" as BoardView

Item {
    id: board

    property int pieceSize

    QtObject {
        readonly property var component: Qt.createComponent("Piece.qml")

        function componentConstructor(id, square) {
            return component.createObject(board, {
                "pieceId": id,
                "square": square,
                "size": Qt.binding(function () {
                    return board.pieceSize
                }),
                "sourceSize": Qt.binding(function () {
                    return board.pieceSize
                })
            });
        }

        readonly property var inner: new BoardView.Piece(componentConstructor)

        id: pieceView
    }

    Component.onCompleted: {
        boardCon.initialize();

        pieceView.inner.connect(boardCon);

        boardCon.resyncBoard();
    }

    BoardCon {
        id: boardCon
        pieceSize: board.pieceSize
    }

    DragArea {
        id: dragArea
        anchors.fill: parent

        onClicked: function (x, y) {
            boardCon.coordClicked(x, y);
        }
        onDragEnded: function (srcX, srcY, destX, destY) {
            boardCon.coordDragEnded(srcX, srcY, destX, destY);
        }
        onDragStarted: function (srcX, srcY, destX, destY) {
            boardCon.coordDragStarted(srcX, srcY, destX, destY);
        }
    }

    Item {
        anchors.fill: parent

        id: hintCanvas

        Repeater {
            model: boardCon.hintSq

            HintRect {
                square: modelData

                size: board.pieceSize
            }
        }

        Repeater {
            model: boardCon.captureSq

            CaptureHintRect {
                square: modelData

                size: board.pieceSize
            }
        }
    }

    PhantomPiece {
        id: phantomPiece

        visible: boardCon.phantomId >= 0
        pieceId: boardCon.phantomId < 0 ? 0 : boardCon.phantomId

        centerX: dragArea.dragPos.x
        centerY: dragArea.dragPos.y

        size: board.pieceSize
        sourceSize: board.pieceSize
    }
    HighlightRect {
        id: highlightRect

        visible: boardCon.highlightSq >= 0
        square: boardCon.highlightSq < 0 ? 0 : boardCon.highlightSq

        size: board.pieceSize
    }
    HighlightRect {
        id: lastSrcRect

        visible: boardCon.lastSrcSq >= 0
        square: boardCon.lastSrcSq < 0 ? 0 : boardCon.lastSrcSq

        size: board.pieceSize
    }
    HighlightRect {
        id: lastDestRect

        visible: boardCon.lastDestSq >= 0
        square: boardCon.lastDestSq < 0 ? 0 : boardCon.lastDestSq

        size: board.pieceSize
    }
    HoverRect {
        id: hoverRect

        visible: boardCon.phantomId >= 0
        mouseX: dragArea.dragPos.x
        mouseY: dragArea.dragPos.y

        size: board.pieceSize
    }

    BoardBackground {
        anchors.fill: parent

        z: -1

        pieceSize: board.pieceSize
    }
}
