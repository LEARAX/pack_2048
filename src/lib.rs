extern crate rand;
extern crate termion;

use rand::Rng;

type Tile = u64;

pub enum Move {
    North,
    West,
    East,
    South,
    None,
}

pub struct Game {
    pub board: [[Tile; 4]; 4],
    pub score: u8,
}

impl Game {
    pub fn gen_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let value = if rng.gen_weighted_bool(10) { 4 } else { 2 };
        // TODO: Make this only work for empty cells, instead of brute-forcing it
        loop {
            let tile_x = rng.gen_range(0, 4);
            let tile_y = rng.gen_range(0, 4);
            if self.board[tile_x][tile_y] == 0 {
                self.board[tile_x][tile_y] = value;
                break;
            }
        }
    }
    pub fn handle_move(&mut self, game_move: Move) {
        match game_move {
            Move::West => {
                for row_index in 0..4 {
                    for column_index in (1..4).rev() {
                        if self.board[row_index][column_index] != (0 as u64) {
                            // TODO: Iterate from left, to resolve bad merging
                            while self.board[row_index][column_index - 1] == (0 as u64) {
                                self.board[row_index][column_index - 1] =
                                    self.board[row_index][column_index];
                                self.board[row_index][column_index] = 0 as Tile;
                            }
                            if self.board[row_index][column_index - 1]
                                == self.board[row_index][column_index]
                            {
                                self.board[row_index][column_index - 1] +=
                                    self.board[row_index][column_index];
                                self.board[row_index][column_index] = 0 as Tile;
                            }
                        }
                    }
                }
            }
            _ => {} // TODO: Make this error.
        }
    }
    pub fn get_text_board(&self) -> std::string::String {
        let divider: std::string::String = "+---+---+---+---+\r\n".to_string();
        let mut finished_string = String::new();
        for row in self.board.iter() {
            finished_string = finished_string + &divider;
            for tile in row.iter() {
                finished_string = finished_string + "| " + &tile.to_string() + " ";
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
