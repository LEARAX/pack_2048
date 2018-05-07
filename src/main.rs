extern crate termion;

use std::io::{Read, Write};

struct Tile {
    value: u64
}

struct Game<R, W: Write> {
    board: [[Tile; 4]; 4],
    points: u8,
    stdin: R,
    stdout: W
}

fn main() {
    println!("Hello, world!");
}
