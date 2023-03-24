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

    QtObject {
        readonly property var component1: Qt.createComponent("HintRect.qml")
        readonly property var component2: Qt.createComponent("CaptureHintRect.qml")

        function componentConstructor1(square) {
            return component1.createObject(board, {
                "square": square,
                "size": Qt.binding(function () {
                    return board.pieceSize;
                })
            })
        }

        function componentConstructor2(square) {
            return component2.createObject(board, {
                "square": square,
                "size": Qt.binding(function () {
                    return board.pieceSize;
                })
            })
        }

        readonly property var inner: new BoardView.Hint(componentConstructor1, componentConstructor2)

        id: hintView
    }

    Component.onCompleted: {
        boardCon.initialize();

        pieceView.inner.connect(boardCon);
        hintView.inner.connect(boardCon);

        boardCon.resyncBoard();
    }

    BoardCon {
        id: boardCon
        pieceSize: board.pieceSize

        onShowHoverRect: function (square) {
            hoverRect.show(square)
        }
        onHideHoverRect: hoverRect.hide()

        onShowHighlightRect: function (square) {
            highlightRect.show(square)
        }
        onHideHighlightRect: highlightRect.hide()

        onShowPhantomPiece: function (id, x, y) {
            phantomPiece.show(id, x, y);
        }
        onUpdatePhantomPiece: function (x, y) {
            phantomPiece.update(x, y);
        }
        onHidePhantomPiece: phantomPiece.hide()
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
        onDragEvent: function (x, y) {
            boardCon.coordDragEvent(x, y);
        }
        onDragStarted: function (srcX, srcY, destX, destY) {
            boardCon.coordDragStarted(srcX, srcY, destX, destY);
        }
    }

    PhantomPiece {
        id: phantomPiece

        size: board.pieceSize
        sourceSize: board.pieceSize
    }
    HighlightRect {
        id: highlightRect

        size: board.pieceSize
    }
    HoverRect {
        id: hoverRect

        size: board.pieceSize
    }

    BoardBackground {
        anchors.fill: parent

        z: -1

        pieceSize: board.pieceSize
    }
}
