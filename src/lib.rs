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

#[derive(Default)]
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
        for (row_index, row) in board.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                if tile == &0 {
                    zero_row.push(row_index);
                    zero_column.push(column_index);
                }
            }
        }
        if !zero_row.is_empty() {
            let index_x = rng.gen_range(0, zero_row.len());
            let index_y = rng.gen_range(0, zero_column.len());
            board[zero_row[index_x]][zero_column[index_y]] = value;
        }
        board
    }
    pub fn move_north(board: &mut Board) {
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
    pub fn move_west(board: &mut Board) {
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
    pub fn move_east(board: &mut Board) {
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
    pub fn move_south(board: &mut Board) {
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
    pub fn handle_move(board: &mut Board, game_move: &Move) {
        match game_move {
            Move::North => Game::move_north(board),
            Move::West => Game::move_west(board),
            Move::East => Game::move_east(board),
            Move::South => Game::move_south(board),
        }
    }
    pub fn get_text_board(board: Board) -> std::string::String {
        let mut max_digits = [0; 4];
        for row in board.iter() {
            for (col_index, tile) in row.iter().enumerate() {
                let digits = digits(tile);
                if digits > max_digits[col_index] {
                    max_digits[col_index] = digits
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
        for row in board.iter() {
            finished_string = finished_string + &divider;
            for (col_index, column) in row.iter().enumerate() {
                let padding_spaces =
                    " ".repeat(1 + max_digits[col_index] - digits(&row[col_index]));
                finished_string =
                    finished_string + "| " + &row[col_index].to_string() + &padding_spaces;
            }
            finished_string += "|\r\n"
        }
        finished_string + &divider
    }
    pub fn new() -> Game {
        let board = [[0; 4]; 4];
        Game { board, score: 0 }
    }
}

fn digits(number: &usize) -> usize {
    if *number != 0 {
        (*number as f32).log10().ceil() as usize
    } else {
        1
    }
}
