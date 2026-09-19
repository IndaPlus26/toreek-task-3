// Author: Viola Söderlund
// Modified by: Isak Larsson

mod tests;
mod piece;

use std::fmt;
use crate::piece::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    Promoting,
    GameOver
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
    state: GameState,
    turn: Colour,
    board: [[Option<Piece>; 8]; 8]
}

impl GameTraits for Game {
    fn new() -> Game {
        Game {
            state: GameState::InProgress,
            turn: Colour::White,
            board: create_default_board()
        }
    }


    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState> {
        todo!()
    }

    // or this.
    fn make_promotion(&mut self, piece: &str) -> Option<GameState> {
        return None;
    }

    fn get_game_state(&self) -> GameState {
        self.state
    }

    fn get_turn(&self) -> Colour {
        self.turn
    }

    fn get_possible_moves(&self, position: &str) -> Vec<String> {
        todo!()
    }

    fn to_fen(&self) -> String {
        todo!()
    }
}

impl Game {
    fn print_board(&self) {
        println!();

        for row in self.board.iter() {
            for board_position in row.iter() {
                if let Some(piece) = board_position {
                    print!("{}", piece.get_character_representation());
                }

                print!(" ");
            }
            println!();
        }

    }
}

fn create_default_board() -> [[Option<Piece>; 8]; 8] {
    [
        create_first_layer_piece_row(Colour::Black),
        [Some(Piece::new(Type::PAWN, Colour::Black)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece::new(Type::PAWN, Colour::White)); 8],
        create_first_layer_piece_row(Colour::White),
    ]
}

fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(Type::ROOK, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KING, colour)), Some(Piece::new(Type::QUEEN, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::ROOK, colour))]
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
