// Author: Viola Söderlund
// Modified by: Isak Larsson

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    Promoting,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

pub trait GameTraits {
    /// Initialises a new board with pieces.
    fn new() -> Self;

    /// If the current game state is [`GameState::InProgress`] and the move is legal,
    /// move a piece from `from` to `to` and return the resulting state of the game.
    ///
    /// Otherwise, return [`None`].
    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState>;

    /// If the current game state is [`GameState::Promoting`], promote the peasant that can be promoted to `piece`.
    ///
    /// Otherwise, return [`None`].
    fn make_promotion(&mut self, piece: &str) -> Option<GameState>;

    /// Get the current state of the game.
    fn get_game_state(&self) -> GameState;

    /// Get the color of the side that is currently playing.
    fn get_turn(&self) -> Colour;

    /// If a piece is standing on the tile at `position`, return all possible new positions of that piece.
    ///
    /// Don't forget the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    fn get_possible_moves(&self, position: &str) -> Vec<String>;

    /// Return the current game board as a FEN string for easy test assertions.
    fn to_fen(&self) -> String;
}

/*
    WRITE YOUR IMPLEMENTATION OF "GameTraits" BELOW
*/

pub struct Game {
    /* board representation, active colour, ... */
    // suggestion, can be removed.
    state: GameState,
    turn: Colour,
}

impl GameTraits for Game {
    // Like this example.
    fn new() -> Game {
        return Game {
            state: GameState::InProgress,
            turn: Colour::White,
        };
    }

    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState> {
        todo!()
    }

    // or this.
    fn make_promotion(&mut self, piece: &str) -> Option<GameState> {
        return None;
    }

    fn get_game_state(&self) -> GameState {
        todo!()
    }

    fn get_turn(&self) -> Colour {
        todo!()
    }

    fn get_possible_moves(&self, position: &str) -> Vec<String> {
        todo!()
    }

    fn to_fen(&self) -> String {
        todo!()
    }


}



/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}



// --------------------------
// ######### TESTS ##########
// --------------------------
#[cfg(test)]
mod tests {
    use super::GameTraits;
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}