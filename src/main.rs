extern crate rand;
extern crate termion;

type Tile = u64;

struct Game {
    board: [[Tile; 4]; 4],
    points: u8
}

impl Game {
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
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0, 4);
        let start_y = rng.gen_range(0, 4);
        let mut board = [[0; 4]; 4];
        board[start_x][start_y] = 2;
        Game {
            board: board,
            points: 0
        }
    }
}

fn main() {
    let mut game_state = Game::new();
    println!{"{}", Game::get_text_board(&game_state)};
}

/*
 * 1. Print board
 * 2. Capture keypress
 * 3. Game logic
 * println!("+---+---+---+---+");
 * println!("| 2 | 0 | 2 | 0 |");
 * println!("+---+---+---+---+");
 * println!("| 2 | 0 | 2 | 0 |");
 * println!("+---+---+---+---+");
 * println!("| 0 | 2 | 8 | 0 |");
 * println!("+---+---+---+---+");
 * println!("| 4 | 2 | 0 | 0 |");
 * println!("+---+---+---+---+");
 */
