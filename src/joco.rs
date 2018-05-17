extern crate pack_2048;
extern crate termion;

use pack_2048::*;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut game_state = Game::new();
    loop {
        Game::gen_tile(&mut game_state.board);
        writeln!(
            stdout,
            "{}{}{}{}Score: {}",
            termion::cursor::Hide,
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            Game::get_text_board(&game_state.board),
            game_state.score
        ).unwrap();
        stdout.flush().unwrap();
        /*
         * let game_move = match keypress.unwrap() {
         *     Key::Up | Key::Char('k') | Key::Char('w') => Some(Move::North),
         *     Key::Left | Key::Char('h') | Key::Char('a') => Some(Move::West),
         *     Key::Right | Key::Char('l') | Key::Char('d') => Some(Move::East),
         *     Key::Down | Key::Char('j') | Key::Char('s') => Some(Move::South),
         *     Key::Char('q') => break,
         *     _ => None,
         * };
         */
        let game_move = Move::East;
        Game::handle_move(&mut game_state.board, &game_move);
        if Game::is_gameover(&game_state.board) {
            break;
        }
    }
    write!(stdout, "\r\n{}", termion::cursor::Show).unwrap();
}
