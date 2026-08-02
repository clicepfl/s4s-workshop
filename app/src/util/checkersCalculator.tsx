import {Board, MoveSequence, PieceType, Player, RichMoveSequence, SingleMove} from "@/api/models";

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
  availableSequences: RichMoveSequence[],
  currentSequence: MoveSequence,
  x: number,
  y: number,
  player: Player,
): MoveWithTakenAndRaffle[] {
  currentSequence = rotateMove(currentSequence, player);
  if (player == Player.Black) {
      x = 9-x;
      y = 9-y;
  }
  let moveSequences = availableSequences.filter((sequence) =>
      sequence[0].length > currentSequence.length &&
      sequence[0][currentSequence.length].from[0] == y &&
      sequence[0][currentSequence.length].from[1] == x &&
      currentSequence.every((move, index) => moveIsEqual(move, sequence[0][index]))
  );

  let possibleMoves = moveSequences.map((sequence) => {
      let move = sequence[0][currentSequence.length].to;
      let taken: [number, number] | undefined = sequence[1][currentSequence.length];
      return {
          x: move[1],
          y: move[0],
          taken: taken ? {x: taken[1], y: taken[0]} : null,
          raffle: sequence[0].length > currentSequence.length + 1
      };
  });

  return player == Player.White ? possibleMoves : possibleMoves.map(({x, y, taken, raffle}) => {
      return {
          x: 9 - x,
          y: 9 - y,
          taken: taken ? {x: 9 - taken.x, y: 9 - taken.y} : null,
          raffle: raffle,
      };
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
