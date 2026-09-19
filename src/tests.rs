
// --------------------------
// ######### TESTS ##########
// --------------------------
#[cfg(test)]
mod tests {
    use crate::{create_default_board, GameTraits};
    use crate::Game;
    use crate::GameState;

    // example test
    // check that game state is in progress after initialization
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }


    //Checks if the board has been properly initialized
    #[test]
    fn test_board_initialization() {
        let game = Game::new();

        game.print_board();

        assert_eq!(game.board, create_default_board());
    }
}

