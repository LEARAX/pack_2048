extern crate pack_2048;
extern crate termion;

use pack_2048::*;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut game_state = Game::new();
    Game::gen_tile(&mut game_state);
    writeln!(
        stdout,
        "{}{}{}\r\nScore: {}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        Game::get_text_board(&game_state),
        game_state.score,
        termion::cursor::Hide
    ).unwrap();
    stdout.flush().unwrap();
    for keypress in stdin.keys() {
        let game_move = match keypress.unwrap() {
            Key::Char('h') => Move::West,
            Key::Char('j') => Move::South,
            Key::Char('k') => Move::North,
            Key::Char('l') => Move::East,
            Key::Char('q') => break,
            _ => break,
        };
        Game::handle_move(&mut game_state, game_move);
        Game::gen_tile(&mut game_state);
        // game_state.board.iter().position
        writeln!(
            stdout,
            "{}{}{}\r\nScore: {}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            Game::get_text_board(&game_state),
            game_state.score,
            termion::cursor::Hide
        ).unwrap();
        stdout.flush().unwrap();
    }
    write!(stdout, "\r\n{}", termion::cursor::Show).unwrap();
}
