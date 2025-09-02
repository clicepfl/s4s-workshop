// component that takes in a checkers board (csv, M for man and K for king)
// and renders it

import React, { use, useEffect, useState } from "react";
import {
    AIError,
    AIErrorType,
    Board as BoardState,
    Piece,
    Player,
    PieceType,
    emptyBoard,
    initialBoards,
    MoveSequence,
    SingleMove,
    TurnStatus, RichMoveSequence,
} from "../api/models";
import {
  calculateBoardAfterMove,
  calculatePossibleMoves,
  MoveWithTakenAndRaffle,
  rotateMove,
} from "@/util/checkersCalculator";
import { makeMove } from "@/api/api";
import { ConsoleMessage } from "@/pages";

type BoardProps = {
  username: string;
  player: Player;
  gameOngoing: boolean;
  currentTurn: Player | null;
  setCurrentTurn: (player: Player) => void;
  board: BoardState;
  setBoard: (board: BoardState) => void;
  updateGame: (
    turnStatus: TurnStatus | AIError,
    initialConsoleOutput?: ConsoleMessage[]
  ) => void;
  availableSequences: RichMoveSequence[]
};

export default function Board({
  username,
  player,
  gameOngoing,
  currentTurn,
  setCurrentTurn,
  board,
  setBoard,
  updateGame,
  availableSequences,
}: BoardProps) {
  const [currentMoveSequence, setCurrentMoveSequence] = useState<MoveSequence>(
    []
  );

  const [selectedPiece, setSelectedPiece] = useState<{
    x: number;
    y: number;
  } | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<MoveWithTakenAndRaffle[]>(
    []
  );

  // handle click on a cell
  function handleClick(x: number, y: number) {
    if (!gameOngoing || currentTurn != player) {
      setSelectedPiece(null);
      setPossibleMoves([]);
      return;
    }
    const clickedPiece = board[y][x];
    if (clickedPiece != null && clickedPiece.player == player) {
      // select piece
      setSelectedPiece({ x, y });
      setPossibleMoves(calculatePossibleMoves(availableSequences, currentMoveSequence, x, y, player));
    } else if (selectedPiece != null) {
      const piece = board[selectedPiece.y][selectedPiece.x];

      if (piece != null) {
        let playedMove = possibleMoves.find(
          (move) => move.x == x && move.y == y
        );
        if (playedMove != null) {
          // add move to current sequence
          let singleMove: SingleMove = {
            from: [selectedPiece.y, selectedPiece.x],
            to: [y, x],
          };
          let newMoveSequence = currentMoveSequence.concat(singleMove);
          setCurrentMoveSequence(newMoveSequence);

          // move piece on board
          let nextBoard = calculateBoardAfterMove(
            board,
            playedMove,
            selectedPiece.x,
            selectedPiece.y
          );
          setBoard(nextBoard);

          // check if the move sequence is finished
          if (playedMove.raffle) {
            setSelectedPiece({ x, y });
            setPossibleMoves(calculatePossibleMoves(availableSequences, newMoveSequence, x, y, player));
          } else {
            setSelectedPiece(null);
            setPossibleMoves([]);
            setCurrentTurn(
              player == Player.White ? Player.Black : Player.White
            ); // switch turn
            makeMove(rotateMove(newMoveSequence, player), username).then(
              (turnStatus) => {
                updateGame(turnStatus);
              },
              (error) => {
                console.error(error);
              }
            ); // send move to server
            setCurrentMoveSequence([]); // reset move sequence
          }
        } else {
          setSelectedPiece(null);
          setPossibleMoves([]);
        }
      }
    } else {
      setSelectedPiece(null);
      setPossibleMoves([]);
    }
  }

  return (
    <div className="board">
      {board.map((row, rowIndex) => (
        <div key={rowIndex} className="row">
          {row.map((cell, cellIndex) => (
            <div
              key={cellIndex}
              className={`cell 
              ${cellIndex % 2 === rowIndex % 2 ? "whitecell" : "blackcell"}
              ${
                selectedPiece?.y === rowIndex && selectedPiece?.x === cellIndex
                  ? "selected"
                  : ""
              }
              ${
                possibleMoves.find(
                  (move) => move.y === rowIndex && move.x === cellIndex
                )
                  ? "possible"
                  : ""
              }`}
              onClick={() => handleClick(cellIndex, rowIndex)}
            >
              {cell == null ? (
                ""
              ) : (
                <div className={`piece ${cell.player}`}>
                  {cell.type == PieceType.King ? (
                    <div className="crown"></div>
                  ) : (
                    ""
                  )}
                </div>
              )}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
