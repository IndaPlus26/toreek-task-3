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
use crate::GameState::*;
use crate::piece::*;
use crate::position::{Position};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Promoting,
    Stalemate
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

#[derive(Copy, Clone)]
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


    /// Checks if the move is legal, see [`self.get_possible_moves`]. If true moves the piece, see [`Game::move_piece`],  else returns [`None`]
    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState> {
         let illegal_move = match self.state {
            InProgress => {
                handle_in_progress(self, from, to)
            }
            Check => {
                handle_in_progress(self, from, to)
            }
            _ => {
                true
            }
        };


        if illegal_move {
            return None;
        }

        self.state = check_check(self);

        if has_opponent_possible_moves(self) {
            if self.state.eq(&Check) {
                self.state = Checkmate;
                println!("Checkmate",);
            }
            else {
                self.state = Stalemate;
                println!("Stalemate",);
            }

        }

        if self.get_game_state().eq(&InProgress) || self.get_game_state().eq(&Check) {
            self.turn = self.turn.get_opponent_color();
        }

        Some(self.state)
    }

    fn make_promotion(&mut self, piece: &str) -> Option<GameState> {
        None
    }

    fn get_game_state(&self) -> GameState {
        self.state
    }

    fn get_turn(&self) -> Colour {
        self.turn
    }


    /// Converts a symbolic representation of a chess position into a [`Position`]. See [`Position::from_symbolic_representation`]
    ///
    /// If the [`Position`] contains a [`Piece`] that has the same [`Colour`] as the current turn, check for possible moves. See [`Piece::get_possible_moves`]
    ///
    /// Returns a [`Vec<String>`] contain the possible moves. Returns an empty [`Vec<String>`] if no possible moves was found.
    fn get_possible_moves(&self, position: &str) -> Vec<String> {
        let position = &Position::from_symbolic_representation(position);

        if let Some(piece) = self.get_piece_at(position) && piece.get_piece_color() == self.turn {
            check_possible_moves_check(position, self, &piece)
        }
        else {
            vec![]
        }
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
