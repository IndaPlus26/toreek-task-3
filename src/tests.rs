// --------------------------
// ######### TESTS ##########
// --------------------------
#[cfg(test)]
mod tests {
    use crate::Game;
    use crate::GameState;
    use crate::piece::Colour;
    use crate::position::*;
    use crate::{GameTraits, create_default_board};
    use std::io;

    // check that game state is in progress after initialization.
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }


    //Checks if the board has been properly initialized.
    #[test]
    fn test_board_initialization() {
        let game = Game::new();

        assert_eq!(game.board, create_default_board());
    }

    #[test]
    fn test_fen() {
        let game = Game::new();

        assert_eq!(game.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn test_pawn_start_available_moves() {
        let mut game = Game::new();

        // Tests where a white pawn can move
        assert_eq!(vec!["E3", "E4"], game.get_possible_moves("E2"));

        //Make a move
        game.make_move("E2","E4");
        assert_eq!(game.to_fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

        //Tests where a black pawn can move
        assert_eq!(vec!["E6", "E5"], game.get_possible_moves("E7"));

        //Make a move
        game.make_move("E7","E5");
        assert_eq!(game.to_fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2");
    }

    #[test]
    fn test_pawn_available_moves_after_one_move() {
        let mut game = Game::new();

        //Make a move
        game.make_move("E2","E3");
        game.make_move("E7","E6");

        //Check if the pawn only is allowed to move step
        assert_eq!(vec!["E4"], game.get_possible_moves("E3"));
        game.make_move("E3","E4");

        //Repeat for black
        assert_eq!(vec!["E5"], game.get_possible_moves("E6"));
        game.make_move("E6","E5");

        assert_eq!(game.to_fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 3");
    }

    #[test]
    fn test_pawn_sideways_moves() {
        let mut game = Game::new();

        game.make_move("E2","E4");
        game.make_move("D7","D5");

        // Check if pawn can move sideways
        assert_eq!(vec!["E5", "D5"], game.get_possible_moves("E4"));
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut game = Game::new();

        game.make_move("E2","E4");

        game.make_move("A7", "A6");
        game.make_move("E4","E5");
        game.make_move("D7","D5");

        // Check if the en passant move is registered in the fen string
        assert_eq!(game.to_fen(), "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3");

        // Check if en passant move is allowed
        assert_eq!(vec!["E6", "D6"], game.get_possible_moves("E5"));
        game.make_move("E5","D6"); // Test the move

        assert_eq!(game.to_fen(), "rnbqkbnr/1pp1pppp/p2P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3");
    }

    #[test]
    fn test_bishop_moves() {
        let mut game = Game::new();

        // Bishop shouldn't be able to move in the beginning
        assert_eq!(true, game.get_possible_moves("C1").is_empty());
        game.make_move("D2","D4");

        //repeat for black
        assert_eq!(true, game.get_possible_moves("C8").is_empty());
        game.make_move("D7","D5");

        // Check bishop move after opening for white
        assert_eq!(vec!["D2", "E3", "F4", "G5", "H6"], game.get_possible_moves("C1"));
        game.make_move("C1","E3");

        // Check bishop move after opening for black
        assert_eq!(vec!["D7", "E6", "F5", "G4", "H3"], game.get_possible_moves("C8"));
        game.make_move("C8","G4");

        assert_eq!(game.to_fen(), "rn1qkbnr/ppp1pppp/8/3p4/3P2b1/4B3/PPP1PPPP/RN1QKBNR w KQkq - 2 3");
    }

    #[test]
    fn test_queen_moves() {
        let mut game = Game::new();

        // Queen shouldn't be able to move in the beginning
        assert_eq!(true, game.get_possible_moves("D1").is_empty());
        game.make_move("E2","E4");

        //repeat for black
        assert_eq!(true, game.get_possible_moves("D8").is_empty());
        game.make_move("E7","E5");

        // Check queen move after opening for white
        assert_eq!(vec!["E2", "F3", "G4", "H5"], game.get_possible_moves("D1"));
        game.make_move("D1","F3");

        // Repeat for black
        assert_eq!(vec!["E7", "F6", "G5", "H4"], game.get_possible_moves("D8"));
        game.make_move("D8","G5");

        // Extra fen test
        assert_eq!(game.to_fen(), "rnb1kbnr/pppp1ppp/8/4p1q1/4P3/5Q2/PPPP1PPP/RNB1KBNR w KQkq - 2 3");


        // Check queen horizontal and vertical moves
        assert_eq!(vec!["E2", "D1", "G4", "H5", "E3", "D3", "C3", "B3", "A3", "F4", "F5", "F6", "F7", "G3", "H3"], game.get_possible_moves("F3"));
        game.make_move("F3","A3");

        // Repeat for black
        assert_eq!(vec!["F4", "E3", "D2", "F6", "E7", "D8", "H4", "H6", "G4", "G3", "G2", "F5", "G6", "H5"], game.get_possible_moves("G5"));
        game.make_move("G5","F4");

        assert_eq!(game.to_fen(), "rnb1kbnr/pppp1ppp/8/4p3/4Pq2/Q7/PPPP1PPP/RNB1KBNR w KQkq - 4 4")
    }

    #[test]
    fn test_rook_moves() {
        let mut game = Game::new();

        // Rook shouldn't be able to move in the beginning
        assert_eq!(true, game.get_possible_moves("A1").is_empty());
        game.make_move("A2","A4");

        //repeat for black
        assert_eq!(true, game.get_possible_moves("A8").is_empty());
        game.make_move("A7","A5");

        // Check moves after we've moved the pawn forwards
        assert_eq!(vec!["A2", "A3"], game.get_possible_moves("A1"));
        game.make_move("A1","A3");

        //repeat for black
        assert_eq!(vec!["A7", "A6"], game.get_possible_moves("A8"));
        game.make_move("A8","A6");

        // Check moves after we've moved the rook forwards
        assert_eq!(vec!["A2", "A1", "B3", "C3", "D3", "E3", "F3", "G3", "H3"], game.get_possible_moves("A3"));
        game.make_move("A3","B3");

        //repeat for black
        assert_eq!(vec!["A7", "A8", "B6", "C6", "D6", "E6", "F6", "G6", "H6"], game.get_possible_moves("A6"));
        game.make_move("A6","B6");

        assert_eq!(game.to_fen(), "1nbqkbnr/1ppppppp/1r6/p7/P7/1R6/1PPPPPPP/1NBQKBNR w Kk - 4 4");
    }

    #[test]
    fn test_rook_castling_queenside() {
        let mut game = Game::new();

        // Move all pieces out of the way
        game.make_move("B1","C3");
        game.make_move("B8","C6");
        game.make_move("B2","B4");
        game.make_move("B7","B5");
        game.make_move("C1","A3");
        game.make_move("C8","A6");
        game.make_move("D1","B1");
        game.make_move("D8","B8");
        game.make_move("B1","B3");
        game.make_move("B8","B6");

        assert_eq!(game.to_fen(), "r3kbnr/p1pppppp/bqn5/1p6/1P6/BQN5/P1PPPPPP/R3KBNR w KQkq - 6 6");

        game.make_move("E1","A1"); // Castling queen white
        assert_eq!(game.to_fen(), "r3kbnr/p1pppppp/bqn5/1p6/1P6/BQN5/P1PPPPPP/2KR1BNR b kq - 7 6");

         game.make_move("E8","A8"); // Castling queen black
        assert_eq!(game.to_fen(), "2kr1bnr/p1pppppp/bqn5/1p6/1P6/BQN5/P1PPPPPP/2KR1BNR w - - 8 7");
    }

    #[test]
    fn test_rook_castling_kingside() {
        let mut game = Game::new();

        // Move all pieces out of the way
        game.make_move("G1","F3");
        game.make_move("G8","F6");

        game.make_move("G2","G4");
        game.make_move("G7","G5");

        game.make_move("F1","H3");
        game.make_move("F8","H6");

        assert_eq!(game.to_fen(), "rnbqk2r/pppppp1p/5n1b/6p1/6P1/5N1B/PPPPPP1P/RNBQK2R w KQkq - 2 4");

        game.make_move("E1","H1"); // Castling king white
        assert_eq!(game.to_fen(), "rnbqk2r/pppppp1p/5n1b/6p1/6P1/5N1B/PPPPPP1P/RNBQ1RK1 b kq - 3 4");

        game.make_move("E8","H8"); // Castling king black
        assert_eq!(game.to_fen(), "rnbq1rk1/pppppp1p/5n1b/6p1/6P1/5N1B/PPPPPP1P/RNBQ1RK1 w - - 4 5");
    }

    #[test]
    fn test_king_movement() {
        let mut game = Game::new();

        // King shouldn't be able to move in the beginning
        assert_eq!(true, game.get_possible_moves("E1").is_empty());
        game.make_move("E2","E4");

        // Repeat for black
        assert_eq!(true, game.get_possible_moves("E8").is_empty());
        game.make_move("E7","E5");

        // Check king movement
        assert_eq!(vec!["E2"], game.get_possible_moves("E1"));
        game.make_move("E1","E2");
        assert_eq!(game.to_fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2"); // Check castling rights

        // Repeat for black
        assert_eq!(vec!["E7"], game.get_possible_moves("E8"));
        game.make_move("E8","E7");

        // Check king movement
        assert_eq!(vec!["D3", "E1", "E3", "F3"], game.get_possible_moves("E2"));
        game.make_move("E2","E3");

        // Repeat for black
        assert_eq!(vec! ["D6", "E6", "E8", "F6"], game.get_possible_moves("E7"));
        game.make_move("E7","E6");

        assert_eq!(game.to_fen(), "rnbq1bnr/pppp1ppp/4k3/4p3/4P3/4K3/PPPP1PPP/RNBQ1BNR w - - 4 4")
    }


    // Checks if it is impossible for the own piece to move in a way that puts the own king into check.
    #[test]
    fn test_anti_check_movement() {
        let mut game = Game::new();
        game.make_move("E2","E4");
        game.make_move("E7","E5");
        game.make_move("D1","H5");
        game.make_move("D8","H4");

        assert_eq!(game.to_fen(), "rnb1kbnr/pppp1ppp/8/4p2Q/4P2q/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3");

        assert_eq!(true, game.get_possible_moves("F2").is_empty()); //Moving the pawn at F2 would put the white king into chess
        game.make_move("A2","A3");


        assert_eq!(true, game.get_possible_moves("F7").is_empty()); //Moving the pawn at F7 would put the black king into chess
    }

    #[test]
    fn test_check() {
        let mut game = Game::new();
        game.make_move("E2","E4");
        game.make_move("E7","E5");
        game.make_move("D1","H5");
        game.make_move("D8","H4");

        assert_eq!(GameState::InProgress, game.get_game_state());

        game.make_move("H5","E5"); // Put black king into check
        assert_eq!(GameState::Check, game.get_game_state());

        assert_eq!(vec!["D8"], game.get_possible_moves("E8")); // Black king should only be able to move into safety
        game.make_move("E8", "D8");
        assert_eq!(GameState::InProgress, game.get_game_state());

        game.make_move("A2","A3"); //Boilerplate move

        //Repeat check test for white king
        game.make_move("H4","E4"); // Put white king into check
        assert_eq!(GameState::Check, game.get_game_state());

        assert_eq!(vec!["D1"], game.get_possible_moves("E1")); // White king should only be able to move into safety
        game.make_move("E1", "D1");
        assert_eq!(GameState::InProgress, game.get_game_state());

        assert_eq!(game.to_fen(), "rnbk1bnr/pppp1ppp/8/4Q3/4q3/P7/1PPP1PPP/RNBK1BNR b - - 1 5")
    }


    //Test game loop
    //#[test]
    fn test_game_functionality_in_console() {
        let mut game = Game::new();
        println!();

        while game.state.eq(&GameState::InProgress) || game.state.eq(&GameState::Check) || game.state.eq(&GameState::Promoting) {


            if game.state.eq(&GameState::Promoting) {
                print!("Select piece type to promote: ");
                let piece_type = get_player_input("Please select a piece type (K, P, R, etc): ");

                loop {
                    if game.make_promotion(&*piece_type).is_some() {
                        break;
                    }
                }

                continue;
            }


            print!("Your turn ");
            println!("{}", get_colour_string(game.turn));

            print_board_and_possible_moves(&game, &vec![]);

            let mut possible_moves: Vec<String> = vec![];
            let mut cord: String;
            loop {
                cord = get_player_input("Please select a piece (pattern E1, A2 etc): ");

                if cord.eq("fen") {
                    println!("{}", game.to_fen());
                    continue;
                }

                else {
                    possible_moves = game.get_possible_moves(cord.as_str());
                }

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


        assert_eq!(true, true);
    }


    ///Gets a two-dimensional coordinate input from the player.
    fn get_player_input(message: &str) -> String {
        loop {
            println!("{}", message);
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

