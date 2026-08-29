use crate::api::Error;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    ops::{Add, Div, Mul},
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Player {
    White,
    Black,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Player::White => 'W',
            Player::Black => 'B',
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PieceType {
    Man,
    King,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PieceType::Man => 'M',
            PieceType::King => 'K',
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Piece {
    #[serde(rename = "type")]
    pub type_: PieceType,
    pub player: Player,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.type_, self.player)
    }
}

const BOARD_SIZE: usize = 10;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

fn default_board() -> Board {
    fn fill_row(board: &mut Board, row: usize, player: Player) {
        for i in 0..BOARD_SIZE {
            if (i + row) % 2 == 1 {
                board[row][i] = Some(Piece {
                    type_: PieceType::Man,
                    player,
                })
            }
        }
    }

    let mut board: Board = Board::default();

    fill_row(&mut board, 0, Player::Black);
    fill_row(&mut board, 1, Player::Black);
    fill_row(&mut board, 2, Player::Black);
    fill_row(&mut board, 3, Player::Black);

    fill_row(&mut board, BOARD_SIZE - 4, Player::White);
    fill_row(&mut board, BOARD_SIZE - 3, Player::White);
    fill_row(&mut board, BOARD_SIZE - 2, Player::White);
    fill_row(&mut board, BOARD_SIZE - 1, Player::White);

    board
}

#[derive(Debug, Serialize, Clone)]
pub struct TurnStatus {
    pub game: GameState,
    pub move_: String,
    pub available_moves: Vec<MoveSequence>,
    pub ai_output: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "status", content = "player", rename_all = "camelCase")]
pub enum GameStatus {
    Running,
    Draw,
    Victory(Player),
}

#[derive(Debug, Serialize, Clone)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub status: GameStatus,
    pub turns: u16, //TEMP
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: default_board(),
            current_player: Player::White,
            status: GameStatus::Running,
            turns: 0, //TEMP
        }
    }
}

pub type Position = (usize, usize);
pub type MoveSequence = (Vec<Move>, Vec<Position>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

pub fn sequence_to_string(sequence: &MoveSequence) -> String {
    let out = sequence.0.iter().fold(String::new(), |out, mov| {
        format!(
            "{}{}{},{}{}:",
            out, mov.from.0, mov.from.1, mov.to.0, mov.to.1
        )
    });
    // Remove trailing ':'
    let mut out_chars = out.chars();
    out_chars.next_back();
    out_chars.as_str().into()
}
fn is_valid_pos(pos: Pos) -> bool {
    pos.x >= 0 && pos.x < BOARD_SIZE as i32 && pos.y >= 0 && pos.y < BOARD_SIZE as i32
}

fn at(board: &Board, pos: Pos) -> Option<&Piece> {
    board[pos.x as usize][pos.y as usize].as_ref()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<i32> for Pos {
    type Output = Pos;

    fn div(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn p(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

fn mov(from: Pos, to: Pos) -> Move {
    Move {
        from: (from.x as usize, from.y as usize),
        to: (to.x as usize, to.y as usize),
    }
}

impl GameState {
    pub fn to_csv_string(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|piece| {
                        piece
                            .as_ref()
                            .map_or_else(|| "".to_owned(), Piece::to_string)
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    pub fn apply_sequence(&mut self, seq: &[Move]) -> Result<(), Error> {
        let available_moves = self.list_valid_moves();

        let move_ = available_moves.into_iter().find(|m| m.0 == seq);

        if let Some((moves, captures)) = move_ {
            self.turns += 1; //TEMP
            let from = moves.first().unwrap().from;
            let to = moves.last().unwrap().to;

            self.board[to.0][to.1] = self.board[from.0][from.1].take();
            if to.0
                == match self.current_player {
                    Player::White => 0,
                    Player::Black => BOARD_SIZE - 1,
                }
            {
                self.board[to.0][to.1].as_mut().unwrap().type_ = PieceType::King;
            }

            for captured in captures {
                self.board[captured.0][captured.1] = None;
            }

            self.current_player = match self.current_player {
                Player::White => Player::Black,
                Player::Black => Player::White,
            };
            self.status = self.compute_status();

            Ok(())
        } else {
            Err(Error::InvalidMove)
        }
    }

    // TODO: Disable networking in the runner
    // TODO: Should the code being ran have access to a function which gives it the possible moves for any position? Probably
    // TODO: We should have a second runner image
    //       that doesnt need to get restarted every time
    fn compute_status(&self) -> GameStatus {
        // If status is already decided as a victory or draw, return that
        if self.status != GameStatus::Running {
            return self.status.clone();
        }

        // Does the current player have any moves available?
        let moves = self.list_valid_moves();
        // No -> opponent wins
        if moves.is_empty() {
            return GameStatus::Victory(if self.current_player == Player::White {
                Player::Black
            } else {
                Player::White
            });
        }

        // Has there been a draw condition?
        // Same position repeated 3+ times? ( NOTE: check if games aren't prone to memory leaks!!)
        // TODO:

        // For testing, not a good solution!
        if self.turns > 200 {
            println!("Game too long!");
            return GameStatus::Draw;
        }

        GameStatus::Running
    }

    pub fn list_valid_moves(&self) -> Vec<MoveSequence> {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Intermediate {
            pos: Pos,
            captures: Vec<Pos>,
            moves: Vec<Move>,
        }

        fn update_intermediate(
            new_pos: Pos,
            captured_pos: Option<Pos>,
            mut i: Intermediate,
        ) -> Option<Intermediate> {
            if captured_pos.is_some_and(|c| i.captures.contains(&c)) {
                None
            } else {
                i.moves.push(mov(i.pos, new_pos));
                i.pos = new_pos;

                if let Some(captured_pos) = captured_pos {
                    i.captures.push(captured_pos);
                }

                Some(i)
            }
        }

        fn list_valid_moves_for_man(state: &GameState, i: Intermediate) -> Vec<Intermediate> {
            let mut moves: Vec<Intermediate> = Vec::new();

            // Check for captures in all four diagonal directions
            let capture_dirs = vec![p(2, 2), p(2, -2), p(-2, 2), p(-2, -2)];
            for d in capture_dirs {
                let new_pos = i.pos + d;
                let captured_pos = i.pos + d / 2;

                if is_valid_pos(new_pos)
                    && (at(&state.board, new_pos).is_none() ||
                        // Allow returning to the starting position
                        if !i.moves.is_empty() {
                            new_pos.x == i.moves[0].from.0 as i32 &&
                            new_pos.y == i.moves[0].from.1 as i32
                        } else { false })
                    && at(&state.board, captured_pos)
                        .is_some_and(|p| p.player != state.current_player)
                {
                    if let Some(new_i) = update_intermediate(new_pos, Some(captured_pos), i.clone())
                    {
                        let subsequent_moves = list_valid_moves_for_man(state, new_i.clone());
                        if subsequent_moves.is_empty() {
                            // No more captures possible, this is a terminal move
                            moves.push(new_i);
                        } else {
                            // Continue the capture sequence
                            moves.extend(subsequent_moves);
                        }
                    }
                }
            }

            // If no captures were found, and this is not a multi-jump sequence,
            // check for non-capture moves.
            if moves.is_empty() && i.captures.is_empty() {
                let dv = match state.current_player {
                    Player::White => -1,
                    Player::Black => 1,
                };

                let non_capture_dirs = vec![p(dv, 1), p(dv, -1)];
                for d in non_capture_dirs {
                    let new_pos = i.pos + d;
                    if is_valid_pos(new_pos) && at(&state.board, new_pos).is_none() {
                        if let Some(new_i) = update_intermediate(new_pos, None, i.clone()) {
                            moves.push(new_i);
                        }
                    }
                }
            }

            moves
        }

        fn list_valid_moves_for_king(state: &GameState, i: Intermediate) -> Vec<Intermediate> {
            let mut moves: Vec<Intermediate> = Vec::new();
            let dirs = vec![p(1, 1), p(1, -1), p(-1, 1), p(-1, -1)];

            // Check for captures
            for d in &dirs {
                let mut captured_piece_pos = None;
                for dist in 1..BOARD_SIZE as i32 {
                    let current_pos = i.pos + *d * dist;
                    if !is_valid_pos(current_pos) {
                        break;
                    }
                    match at(&state.board, current_pos) {
                        Some(p) => {
                            if p.player != state.current_player {
                                if captured_piece_pos.is_none() {
                                    captured_piece_pos = Some(current_pos);
                                } else {
                                    break; // Two pieces in a row, invalid
                                }
                            } else {
                                break; // Blocked by own piece
                            }
                        }
                        None => {
                            if let Some(c_pos) = captured_piece_pos {
                                if !i.captures.contains(&c_pos) {
                                    let mut new_i = i.clone();
                                    new_i.moves.push(mov(i.pos, current_pos));
                                    new_i.captures.push(c_pos);
                                    new_i.pos = current_pos;
                                    let subsequent_moves =
                                        list_valid_moves_for_king(state, new_i.clone());
                                    if subsequent_moves.is_empty() {
                                        moves.push(new_i);
                                    } else {
                                        moves.extend(subsequent_moves);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // If no captures were found, and this isn't a continuation of a capture sequence,
            // generate non-capture moves
            if moves.is_empty() && i.captures.is_empty() {
                for d in &dirs {
                    for dist in 1..BOARD_SIZE as i32 {
                        let new_pos = i.pos + *d * dist;
                        if is_valid_pos(new_pos) && at(&state.board, new_pos).is_none() {
                            let mut new_i = i.clone();
                            new_i.moves.push(mov(i.pos, new_pos));
                            new_i.pos = new_pos;
                            moves.push(new_i);
                        } else {
                            break; // Stop at the first piece or out-of-bounds square
                        }
                    }
                }
            }

            moves
        }

        let mut available_moves = vec![];

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let pos = p(col as i32, row as i32);
                let piece = at(&self.board, pos);

                let moves = match piece {
                    Some(Piece {
                        type_: PieceType::Man,
                        player,
                    }) if *player == self.current_player => list_valid_moves_for_man(
                        self,
                        Intermediate {
                            pos,
                            captures: vec![],
                            moves: vec![],
                        },
                    ),
                    Some(Piece {
                        type_: PieceType::King,
                        player,
                    }) if *player == self.current_player => list_valid_moves_for_king(
                        self,
                        Intermediate {
                            pos,
                            captures: vec![],
                            moves: vec![],
                        },
                    ),
                    _ => continue,
                };

                available_moves.append(
                    &mut moves
                        .into_iter()
                        .map(|i| {
                            (
                                i.moves,
                                i.captures
                                    .into_iter()
                                    .map(|c| (c.x as usize, c.y as usize))
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect(),
                );
            }
        }

        let max = available_moves.iter().map(|m| m.1.len()).max();

        if let Some(max) = max {
            available_moves
                .into_iter()
                .filter(|m| m.1.len() == max)
                .collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Board, GameState, Move, MoveSequence, Piece};
    use core::hash::Hash;
    use std::collections::HashSet;

    fn p(type_: char, player: char) -> Option<Piece> {
        Some(Piece {
            type_: match type_ {
                'M' => super::PieceType::Man,
                'K' => super::PieceType::King,
                _ => panic!(),
            },
            player: match player {
                'W' => super::Player::White,
                'B' => super::Player::Black,
                _ => panic!(),
            },
        })
    }

    fn m(x1: usize, y1: usize, x2: usize, y2: usize) -> Move {
        Move {
            from: (x1, y1),
            to: (x2, y2),
        }
    }

    fn list(board: Board) -> (Vec<MoveSequence>, Vec<MoveSequence>) {
        (
            GameState {
                board: board.clone(),
                current_player: crate::game::Player::White,
                status: super::GameStatus::Running,
            }
            .list_valid_moves(),
            GameState {
                board,
                current_player: crate::game::Player::Black,
                status: super::GameStatus::Running,
            }
            .list_valid_moves(),
        )
    }

    fn iters_equal_anyorder<T: Eq + Hash>(
        mut i1: impl Iterator<Item = T>,
        i2: impl Iterator<Item = T>,
    ) -> bool {
        let set: HashSet<T> = i2.collect();
        i1.all(|x| set.contains(&x))
    }

    #[test]
    fn empty() {
        let state = GameState {
            board: [
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
            ],
            current_player: super::Player::White,
            ..Default::default()
        };
        assert!(state.list_valid_moves().is_empty());
    }

    #[test]
    fn trivial_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 2);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 4, 1)], vec![]), (vec![m(5, 2, 4, 3)], vec![])].iter()
        ));

        assert!(black_moves.is_empty());
    }

    #[test]
    fn single_capture_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                p('M', 'B'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 1);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 3, 0)], vec![(4, 1)])].iter()
        ));

        assert_eq!(black_moves.len(), 1);
        assert!(iters_equal_anyorder(
            black_moves.iter(),
            [(vec![m(4, 1, 6, 3)], vec![(5, 2)])].iter()
        ));
    }

    #[test]
    fn stuck_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                p('M', 'B'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                p('M', 'B'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, _) = list(board);

        assert_eq!(white_moves.len(), 1);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 4, 3)], vec![])].iter()
        ));
    }

    #[test]
    fn multiple_capture_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                p('M', 'B'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                p('M', 'B'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 1);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 3, 0), m(3, 0, 1, 2)], vec![(4, 1), (2, 1)])].iter()
        ));

        assert_eq!(black_moves.len(), 1);
        assert!(iters_equal_anyorder(
            black_moves.iter(),
            [(vec![m(4, 1, 6, 3)], vec![(5, 2)])].iter()
        ));
    }

    #[test]
    fn multiple_capture_backwards_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                p('M', 'B'),
                None,
                p('M', 'B'),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None, //
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 1);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 3, 4), m(3, 4, 5, 6)], vec![(4, 3), (4, 5)])].iter()
        ));

        assert_eq!(black_moves.len(), 1);
        assert!(iters_equal_anyorder(
            black_moves.iter(),
            [(vec![m(4, 3, 6, 1)], vec![(5, 2)])].iter()
        ));
    }

    #[test]
    fn trivial_king() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                p('K', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 13);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [
                (vec![m(5, 2, 3, 0)], vec![]),
                (vec![m(5, 2, 4, 1)], vec![]),
                (vec![m(5, 2, 6, 3)], vec![]),
                (vec![m(5, 2, 7, 4)], vec![]),
                (vec![m(5, 2, 8, 5)], vec![]),
                (vec![m(5, 2, 9, 6)], vec![]),
                (vec![m(5, 2, 7, 0)], vec![]),
                (vec![m(5, 2, 6, 1)], vec![]),
                (vec![m(5, 2, 4, 3)], vec![]),
                (vec![m(5, 2, 3, 4)], vec![]),
                (vec![m(5, 2, 2, 5)], vec![]),
                (vec![m(5, 2, 1, 6)], vec![]),
                (vec![m(5, 2, 0, 7)], vec![]),
            ]
            .iter()
        ));

        assert!(black_moves.is_empty());
    }
}
