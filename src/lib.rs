extern crate rand;
extern crate termion;

use rand::Rng;

type Tile = usize;
type Board = [[Tile; 4]; 4];

pub enum Move {
    North,
    West,
    East,
    South,
}

pub struct Game {
    pub board: Board,
    pub score: usize,
}

impl Game {
    pub fn gen_tile(board: &mut Board) -> &mut Board {
        let mut rng = rand::thread_rng();
        let value = if rng.gen_weighted_bool(10) { 4 } else { 2 };
        let mut zero_row = vec![];
        let mut zero_column = vec![];
        for row in 0..4 {
            for column in 0..4 {
                if board[row][column] == 0 {
                    zero_row.push(row);
                    zero_column.push(column);
                }
            }
        }
        if zero_row.len() != 0 {
            let index_x = rng.gen_range(0, zero_row.len());
            let index_y = rng.gen_range(0, zero_column.len());
            board[zero_row[index_x]][zero_column[index_y]] = value;
        }
        board
    }
    pub fn handle_move(board: &mut Board, game_move: Option<Move>) {
        match game_move {
            Some(Move::North) => {
                for column in 0..4 {
                    for row in (0..4).rev() {
                        for next_row in (row + 1)..4 {
                            if board[next_row][column] != 0 {
                                if board[row][column] == 0 as Tile {
                                    board[row][column] = board[next_row][column];
                                    board[next_row][column] = 0;
                                } else if board[row][column] == board[next_row][column] {
                                    board[row][column] += board[row][column];
                                    board[next_row][column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::West) => {
                for mut row in &mut board.iter_mut() {
                    for column in 0..3 {
                        for next_column in (column + 1)..4 {
                            if row[next_column] != 0 {
                                if row[column] == 0 as Tile {
                                    row[column] = row[next_column];
                                    row[next_column] = 0;
                                } else if row[column] == row[next_column] {
                                    row[column] += row[column];
                                    row[next_column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::East) => {
                for mut row in &mut board.iter_mut() {
                    for column in (0..4).rev() {
                        for next_column in (0..column).rev() {
                            if row[next_column] != 0 {
                                if row[column] == 0 as Tile {
                                    row[column] = row[next_column];
                                    row[next_column] = 0;
                                } else if row[column] == row[next_column] {
                                    row[column] += row[column];
                                    row[next_column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::South) => {
                for column in 0..4 {
                    for row in (0..4).rev() {
                        for next_row in (0..row).rev() {
                            if board[next_row][column] != 0 {
                                if board[row][column] == 0 as Tile {
                                    board[row][column] = board[next_row][column];
                                    board[next_row][column] = 0;
                                } else if board[row][column] == board[next_row][column] {
                                    board[row][column] += board[row][column];
                                    board[next_row][column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }
    pub fn get_text_board(board: Board) -> std::string::String {
        let mut max_digits = [0; 4];
        for column in 0..4 {
            for row in 0..4 {
                let digits = digits(board[row][column]);
                if digits > max_digits[column] {
                    max_digits[column] = digits
                }
            }
        }
        let col1_dash = "-".repeat(max_digits[0] + 2);
        let col2_dash = "-".repeat(max_digits[1] + 2);
        let col3_dash = "-".repeat(max_digits[2] + 2);
        let col4_dash = "-".repeat(max_digits[3] + 2);
        let divider: std::string::String = format!(
            "+{c1}+{c2}+{c3}+{c4}+\r\n",
            c1 = col1_dash,
            c2 = col2_dash,
            c3 = col3_dash,
            c4 = col4_dash
        );

        let mut finished_string = String::new();
        for row in &board {
            finished_string = finished_string + &divider;
            for column in 0..4 {
                let padding_spaces = " ".repeat(1 + max_digits[column] - digits(row[column]));
                finished_string =
                    finished_string + "| " + &row[column].to_string() + &padding_spaces;
            }
            finished_string += "|\r\n"
        }
        finished_string = finished_string + &divider;
        return finished_string;
    }
    pub fn new() -> Game {
        let board = [[0; 4]; 4];
        Game {
            board,
            score: 0,
        }
    }
}

fn digits(number: usize) -> usize {
    if number != 0 {
        (number as f32).log10().ceil() as usize
    } else {
        1
    }
}
