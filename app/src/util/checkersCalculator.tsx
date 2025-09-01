import {Board, MoveSequence, Piece, PieceType, Player, SingleMove} from "@/api/models";

export type MoveWithTakenAndRaffle = {
  x: number;
  y: number;
  taken: { x: number; y: number } | null;
  raffle: boolean;
};

export function calculateBoardAfterMove(
  board: Board,
  move: MoveWithTakenAndRaffle,
  x: number,
  y: number
): Board {
  const newBoard = JSON.parse(JSON.stringify(board));
  newBoard[move.y][move.x] = board[y][x];
  newBoard[y][x] = null;

  if (move.taken != null) {
    newBoard[move.taken.y][move.taken.x] = null;
  }

  if (!move.raffle) {
    // Promote the piece to a king if it reaches the opposite side of the board
    if (move.y === 0) {
      newBoard[move.y][move.x].type = PieceType.King;
    }
  }

  return newBoard;
}

function moveIsEqual(lhs: SingleMove, rhs: SingleMove): boolean {
    return lhs.from[0] == rhs.from[0] &&
        lhs.from[1] == rhs.from[1] &&
        lhs.to[0] == rhs.to[0] &&
        lhs.to[1] == rhs.to[1];
}

export function calculatePossibleMoves(
  board: Board,
  availableSequences: MoveSequence[],
  currentSequence: MoveSequence,
  piece: Piece,
  x: number,
  y: number
): MoveWithTakenAndRaffle[] {
    // TODO: flip moves vertically if white is at the top.
  let moveSequences2 = availableSequences.filter((sequence) =>
      sequence.length > currentSequence.length &&
      sequence[currentSequence.length].from[0] == y &&
      sequence[currentSequence.length].from[1] == x &&
      currentSequence.every((move, index) => moveIsEqual(move, sequence[index]))
  );

  let moveSequences = moveSequences2.map((sequence) => sequence.slice(currentSequence.length));

  return moveSequences.map((moveSequence) => {
      // TODO: taken is not null in general, but that requires API changes, so...
    return { x: moveSequence[0].to[1], y: moveSequence[0].to[0], taken: null, raffle: moveSequence.length > 1 };
  });
}

// Rotate the board so that the player is always at the bottom
export function rotateBoard(board: Board, player: Player): Board {
  if (player === Player.White) {
    return board;
  }

  let rotatedBoard = [...board].reverse().map(row => [...row].reverse());

  return rotatedBoard;
}

// Rotate the move so that the player is always at the bottom
export function rotateMove(move: MoveSequence, player: Player): MoveSequence {
  if (player === Player.White) {
    return move;
  }

  return move.map((m) => {
    return {
      from: [9 - m.from[0], 9 - m.from[1]],
      to: [9 - m.to[0], 9 - m.to[1]],
    };
  });
}
