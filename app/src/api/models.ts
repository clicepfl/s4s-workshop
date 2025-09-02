export interface GameState {
  board: Board;
  current_player: Player;
}
export interface TurnStatus {
  game: GameState;
  available_moves: RichMoveSequence[];
  ai_output: string; // Everything printed by the AI on stderr since the start/last move.
}

export enum AIErrorType {
  NoSubmission = "noSubmission",
  InvalidMove = "invalidMove",
  InvalidOutput = "invalidOutput",
}

export interface AIError {
  error: AIErrorType;
  ai_output: string;
  move: MoveSequence | null;
}

export type Board = (Piece | null)[][];

export interface Piece {
  type: PieceType;
  player: Player;
}

export enum Player {
  White = "white",
  Black = "black",
}

export enum PieceType {
  Man = "man",
  King = "king",
}
export type SingleMove = { from: [number, number]; to: [number, number] };
export type MoveSequence = { from: [number, number]; to: [number, number] }[];
export type RichMoveSequence = [MoveSequence, [number, number][]];

export type Score = {
    elo: number;
    wins: number;
    losses: number;
    draws: number;
};

export type Scoreboard = Record<string, Score>;

export enum SubmissionLanguage {
  Java = "java",
  Cpp = "cpp",
  Python = "python",
}

export const emptyBoard: Board = Array.from({ length: 10 }, () =>
  Array(10).fill(null)
);
export const initialBoards = generateInitialBoards();

function generateInitialBoards(): Record<Player, Board> {
  const boards: Record<Player, Board> = {
    [Player.Black]: [],
    [Player.White]: [],
  };

  for (let i = 0; i < 10; i++) {
    boards[Player.Black].push([]);
    boards[Player.White].push([]);

    for (let j = 0; j < 10; j++) {
      if ((i + j) % 2 === 0 || (i >= 4 && i < 6)) {
        boards[Player.Black][i].push(null);
        boards[Player.White][i].push(null);
      } else {
        const type = PieceType.Man;
        boards[Player.Black][i].push({
          player: i < 4 ? Player.White : Player.Black,
          type: type,
        });
        boards[Player.White][i].push({
          player: i < 4 ? Player.Black : Player.White,
          type: type,
        });
      }
    }
  }

  return boards;
}
