// Author: Viola Söderlund
// Modified by: Isak Larsson

mod tests;

/// Contains definitions of a chess piece and related code
pub mod piece;

/// Contains board logic; accessing and moving pieces and initializing the board
pub mod board;

/// Contains definition of a chess position and position logic
pub mod position;

/// Contains logic for how each chess piece should move 
pub mod moves;

use std::fmt;
use crate::board::*;
use crate::GameState::InProgress;
use crate::piece::*;
use crate::position::{Position};

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
            state: InProgress,
            turn: Colour::White,
            board: create_default_board()
        }
    }


    //TODO: CHECK
    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState> {
        let state = InProgress;

        if self.get_possible_moves(from).contains(&to.to_string()) {
            self.move_piece(&Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));
        }
        else {
            return None;
        }

        if state == InProgress {
            self.turn = self.turn.get_opponent_color();
        }

        Option::from(InProgress)
    }

    // or this.
    fn make_promotion(&mut self, piece: &str) -> Option<GameState> {
        None
    }

    fn get_game_state(&self) -> GameState {
        self.state
    }

    fn get_turn(&self) -> Colour {
        self.turn
    }

    fn get_possible_moves(&self, position: &str) -> Vec<String> {
        let position = &Position::from_symbolic_representation(position);

        if let Some(piece) = self.get_piece_at(position) && piece.get_piece_color() == self.turn {
            piece.get_possible_moves(position, self)
        }
        else {
            vec![]
        }
    }

    fn to_fen(&self) -> String {
        todo!()
    }
}



fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(Type::ROOK, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::King, colour)), Some(Piece::new(Type::QUEEN, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::ROOK, colour))]
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
