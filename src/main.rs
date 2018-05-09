extern crate termion;

use std::io::{Read, Write};

type Tile = u64;

struct Game {
    board: [[Tile; 4]; 4],
    points: u8
}

impl Game {
    pub fn get_text_board(&self) -> std::string::String {
        let mut finished_string = String::new();
        for row in self.board.iter() {
            finished_string = finished_string + &"+---+---+---+---+".to_string();
            for cell in row.iter() {
                finished_string = finished_string + &cell.to_string();
            }
        }
        return finished_string;
    }
    pub fn new() -> Game {
        Game {
            board: [[0; 4]; 4],
            points: 0
        }
    }
}

fn main() {
    let mut game_state = Game::new();
    println!("+---+---+---+---+");
    println!("| 2 | 0 | 2 | 0 |");
    println!("+---+---+---+---+");
    println!("| 2 | 0 | 2 | 0 |");
    println!("+---+---+---+---+");
    println!("| 0 | 2 | 8 | 0 |");
    println!("+---+---+---+---+");
    println!("| 4 | 2 | 0 | 0 |");
    println!("+---+---+---+---+");
    Game::get_text_board(&game_state);
}

/*
 * 1. Print board
 * 2. Capture keypress
 * 3. Game logic
 */
