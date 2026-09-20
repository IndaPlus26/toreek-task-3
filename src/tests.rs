// --------------------------
// ######### TESTS ##########
// --------------------------
#[cfg(test)]
mod tests {
    use crate::Game;
    use crate::GameState;
    use crate::position::*;
    use crate::piece::Colour;
    use crate::{GameTraits, create_default_board};
    use std::io;

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

        assert_eq!(game.board, create_default_board());
    }


    //Test game loop
    #[test]
    fn test_game_functionality_in_console() {
        let mut game = Game::new();
        println!();

        while game.state.eq(&GameState::InProgress) {
            print!("Your turn ");
            println!("{}", get_colour_string(game.turn));

            print_board_and_possible_moves(&game, &vec![]);

            let mut possible_moves: Vec<String> = vec![];
            let mut cord: String;
            loop {
                cord = get_player_input("Please select a piece (pattern E1, A2 etc): ");

                possible_moves = game.get_possible_moves(cord.as_str());

                if possible_moves.is_empty() {
                    println!("No possible moves found");
                    continue;
                }
                else {
                    break;
                }
            }


            print_board_and_possible_moves(&game, &possible_moves);
            loop {
                let cord2 = get_player_input("Please select a position to move to ");

                if game.make_move(&*cord, &*cord2).is_some() {
                    break;
                }
                
            }

        }


        assert_eq!(game.board, create_default_board());
    }


    ///Gets a two-dimensional coordinate input from the player.
    fn get_player_input(message: &str) -> String {
        loop {
            println!("{}", message);
            let input = io::stdin();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {

                }
                Err(error) => println!("error: {error}"),
            }

            return input.trim().to_string();
        }
    }

    fn get_colour_string(colour: Colour) -> &'static str {
        if colour == Colour::White {
            "White"
        }
        else {
            "Black"
        }
    }

    pub(crate) fn print_board_and_possible_moves(game: &Game, possible_moves: &Vec<String>) {
        println!();

        for colum in (1..9).rev() {
            for row in 1..9 {
                if let Some(piece) = game.get_piece_at(&Position::new(row, colum)) {
                    if possible_moves.contains(&Position::new(row, colum).to_symbolic_representation()) {
                        highlight(piece.get_character_representation());
                    }
                    else {
                        print!("{}", piece.get_character_representation());
                    }
                }
                else {
                    if possible_moves.contains(&Position::new(row, colum).to_symbolic_representation()) {
                        highlight(' ');
                    }
                    else {
                        print!("{}", " ");
                    }
                }
            }
            println!();
        }
    }

    fn highlight(text: char) {
        print!("\x1b[42m{}\x1b[0m", text);
    }
}

