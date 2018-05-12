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
        for row in board.iter() {
            for tile in row {
                if tile != &mut 0 {
                    // TODO
                }
            }
        }
        loop {
            let tile_x = rng.gen_range(0, 4);
            let tile_y = rng.gen_range(0, 4);
            if board[tile_x][tile_y] == 0 {
                board[tile_x][tile_y] = value;
                break;
            }
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
    // TODO: Scale with number of digits in number (log(n) + 1)
    pub fn get_text_board(board: Board) -> std::string::String {
        let mut max_digits = [3; 4];
        for column in 0..4 {
            for row in 0..4 {
                let digits = (board[row][column] as f32).log10().ceil() as usize + 2;
                println!("DGITS: {}", digits);
                if digits > max_digits[column] {
                    max_digits[column] = digits
                }
            }
        }
        println!("ARRY: {:?}", max_digits);
        let col1_dash = "-".repeat(max_digits[0]);
        println!("DASH: {}", col1_dash);
        let col2_dash = "-".repeat(max_digits[1]);
        let col3_dash = "-".repeat(max_digits[2]);
        let col4_dash = "-".repeat(max_digits[3]);
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
            for column in 0..4 {
                finished_string = finished_string + "| " + &row[column].to_string() + &" ".repeat(max_digits[column]);
            }
            finished_string = finished_string + "|\r\n"
        }
        finished_string = finished_string + &divider;
        return finished_string;
    }
    pub fn new() -> Game {
        let board = [[0; 4]; 4];
        Game {
            board: board,
            score: 0,
        }
    }
}
