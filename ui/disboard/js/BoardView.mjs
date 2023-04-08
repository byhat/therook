export class Piece {
    constructor(componentConstructor) {
        this.componentConstructor = componentConstructor;
        this.pieceVec = Array(64).fill(null);
    }

    connect(boardCon) {
        const this_out = this;
        boardCon.placePiece.connect(function (id, square) {
            this_out.place(
                id, square
            );
        });
        boardCon.movePiece.connect(function (src, dest) {
            this_out.move(src, dest);
        });
        boardCon.removePiece.connect(function (square) {
            this_out.remove(square);
        });
        boardCon.resetBoard.connect(function (squares, pieces) {
            this_out.reset(squares, pieces)
        })
    }

    remove(square) {
        const piece = this.pieceVec[square];
        if (piece == null) return;
        this.pieceVec[square] = null;
        piece.destroy();
    }

    place(id, square) {
        this.remove(square);

        const new_piece = this.componentConstructor(id, square);
        this.pieceVec[square] = new_piece;
    }

    move(src, dest) {
        const srcPiece = this.pieceVec[src];
        if (srcPiece == null) return;

        this.remove(dest);

        srcPiece.animationEnabled = true;
        srcPiece.square = dest;
        srcPiece.animationEnabled = false;

        this.pieceVec[src] = null;
        this.pieceVec[dest] = srcPiece;
    }

    reset(squares, pieces) {
        const initial = squares.map(function(e, i) {
            return [e, pieces[i]];
        });

        const prevPieceVec = this.pieceVec;
        this.pieceVec = Array(64).fill(null);

        for (const val of initial) {
            const square = val[0];
            const pieceId = val[1];

            if (prevPieceVec[square] != null) {
                if (prevPieceVec[square].pieceId == pieceId) {
                    // Move piece to new array
                    this.pieceVec[square] = prevPieceVec[square];
                    prevPieceVec[square] = null;
                    continue;
                }
            }

            const new_piece = this.componentConstructor(pieceId, square);
            this.pieceVec[square] = new_piece;
        }

        for (const piece of prevPieceVec) {
            if (piece != null) {
                piece.destroy();
            }
        }
    }
}
