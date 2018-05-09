extern crate rand;
extern crate termion;

use rand::Rng;
use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

type Tile = u64;

struct Game {
    board: [[Tile; 4]; 4],
    points: u8,
}

impl Game {
    pub fn gen_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let tile_x = rng.gen_range(0, 4);
        let tile_y = rng.gen_range(0, 4);
        if rng.gen_weighted_bool(10) {
            let value = 2;
        } else {
            let value = 4;
        };

        self.board[tile_x][tile_y] = 2;
    }
    pub fn get_text_board(&self) -> std::string::String {
        let mut finished_string = String::new();
        for row in self.board.iter() {
            finished_string = finished_string + &"+---+---+---+---+\n".to_string();
            for cell in row.iter() {
                finished_string = finished_string + "| " + &cell.to_string() + " ";
            }
            finished_string = finished_string + "|\n"
        }
        finished_string = finished_string + &"+---+---+---+---+\n".to_string();
        return finished_string;
    }
    pub fn new() -> Game {
        let board = [[0; 4]; 4];
        Game {
            board: board,
            points: 0,
        }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut game_state = Game::new();
    termion::clear::All;
    // loop {
        Game::gen_tile(&mut game_state);
        writeln!(stdout, "{}", Game::get_text_board(&game_state)).unwrap();
    // }
}
