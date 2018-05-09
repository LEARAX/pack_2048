extern crate rand;
extern crate termion;

use rand::Rng;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type Tile = u64;

pub enum Move {
    North,
    West,
    East,
    South,
}

struct Game {
    board: [[Tile; 4]; 4],
    points: u8,
}

impl Game {
    pub fn gen_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let tile_x = rng.gen_range(0, 4);
        let tile_y = rng.gen_range(0, 4);
        let value = if rng.gen_weighted_bool(10) { 4 } else { 2 };

        self.board[tile_x][tile_y] = value;
    }
    pub fn handle_move(&mut self, game_move: Move) {
        // TODO
    }
    pub fn get_text_board(&self) -> std::string::String {
        let mut finished_string = String::new();
        for row in self.board.iter() {
            finished_string = finished_string + &"+---+---+---+---+\r\n".to_string();
            for cell in row.iter() {
                finished_string = finished_string + "| " + &cell.to_string() + " ";
            }
            finished_string = finished_string + "|\r\n"
        }
        finished_string = finished_string + &"+---+---+---+---+".to_string();
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
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut game_state = Game::new();
    // loop {
    Game::gen_tile(&mut game_state);
    writeln!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        Game::get_text_board(&game_state),
        termion::cursor::Hide
    ).unwrap();
    stdout.flush().unwrap();
    for keypress in stdin.keys() {
        let game_move = match keypress.unwrap() {
            Key::Char('h') => Move::West,
            Key::Char('j') => Move::South,
            Key::Char('k') => Move::North,
            Key::Char('l') => Move::East,
            _ => Move::West,
        };
        Game::handle_move(&mut game_state, game_move);
        break;
    }
    // }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
