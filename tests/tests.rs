extern crate pack_2048;

#[cfg(test)]
mod core {
    use pack_2048;
    #[test]
    fn create_gamestate() {
        let manual_state = pack_2048::Game {
            board: [[0; 4]; 4],
            score: 0,
        };
        let game_state = pack_2048::Game::new();
        assert_eq!(game_state.board, manual_state.board, "Boards do not match");
        assert_eq!(game_state.score, manual_state.score, "Scores do not match");
    }
    #[test]
    fn top_row_2() {
        let mut game_board = [[2; 4], [0; 4], [0; 4], [0; 4]];
        pack_2048::Game::move_west(&mut game_board);
        assert_eq!(game_board, [[4, 4, 0, 0], [0; 4], [0; 4], [0; 4]])
    }
}
