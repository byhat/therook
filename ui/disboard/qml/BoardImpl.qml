import QtQuick
import fr.therook.ui

import "../js/BoardView.mjs" as BoardView

Item {
    required property int pieceSize

    property alias pgn: boardCon.pgn

    id: board

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
        property int file: boardCon.promotingFile

        readonly property var component: Qt.createComponent("PromotionWindow.qml")
        property var createdObject: null

        function componentConstructor(new_file) {
            return component.createObject(board, {
                "file": new_file < 10 ? new_file : new_file - 10,
                "side": new_file < 10,
                "isWhite": new_file < 10,
                "pieceSize": Qt.binding(function () {
                    return board.pieceSize
                }),
            });
        }

        id: promotionWindow

        function open(new_file) {
            close();
            createdObject = componentConstructor(new_file);
            createdObject.selected.connect(function (piece) {
                boardCon.promote(piece);
            });
        }

        function close() {
            if (createdObject == null) {
                return;
            }
            createdObject.destroy();
            createdObject = null;
        }

        onFileChanged: {
            if (file < 0) close();
            else open(file);
        }
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
        onScrolled: function (delta) {
            if (delta > 0) {
                boardCon.nextMove();
            } else {
                boardCon.prevMove();
            }
        }
    }

    Item {
        anchors.fill: parent

        id: hintCanvas

        Repeater {
            model: boardCon.hintSq

            HintRect {
                required property int modelData

                square: modelData
                size: board.pieceSize
            }
        }

        Repeater {
            model: boardCon.captureSq

            CaptureHintRect {
                required property int modelData

                square: modelData
                size: board.pieceSize
            }
        }
    }

    Item {
        anchors.fill: parent

        HighlightRect {
            square: boardCon.highlightSq
            size: board.pieceSize
        }
        HighlightRect {
            square: boardCon.lastSrcSq
            size: board.pieceSize
        }
        HighlightRect {
            square: boardCon.lastDestSq
            size: board.pieceSize
        }
    }

    Item {
        anchors.fill: parent

        z: 11

        PhantomPiece {
            id: phantomPiece

            z: 1

            visible: boardCon.phantomId >= 0
            pieceId: boardCon.phantomId < 0 ? 0 : boardCon.phantomId

            centerX: dragArea.dragPos.x
            centerY: dragArea.dragPos.y

            size: board.pieceSize
            sourceSize: board.pieceSize
        }

        HoverRect {
            id: hoverRect

            z: 0

            visible: boardCon.phantomId >= 0
            mouseX: dragArea.dragPos.x
            mouseY: dragArea.dragPos.y

            size: board.pieceSize
        }
    }

    BoardBackground {
        anchors.fill: parent

        z: -1

        pieceSize: board.pieceSize
    }
}
