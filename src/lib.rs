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
use crate::piece::Colour::White;
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
    board: [[Option<Piece>; 8]; 8],
    halfmove_clock: u8,
    fullmove_counter: u32,
}

impl GameTraits for Game {
    fn new() -> Game {
        Game {
            state: InProgress,
            turn: White,
            board: create_default_board(),
            halfmove_clock: 0,
            fullmove_counter: 1
        }
    }


    /// Entry point for moving a piece
    /// Checks if the [`GameState`] allows for moves to be made. Moves are only allowed during [`Check`] and [`InProgress`]
    ///
    /// See [`handle_in_progress`] for piece movement logic
    ///
    /// Returns [`None`] if the move was illegal or movement wasn't allowed
    ///
    /// Calls [`post_turn_check`] to determinate the next [`GameState`]
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

        post_turn_check(self);
        Some(self.state)
    }

    /// Entry point for making a promotion
    /// Checks if the [`GameState`] allows for/equals [`Promoting`]
    /// Checks if the input is valid. See [`get_promotion_type`]
    ///
    /// If the input is valid, promote the piece and do the usual [`post_turn_check`]
    fn make_promotion(&mut self, piece: &str) -> Option<GameState> {
        if self.state.eq(&Promoting)  {
           if let Some(piece_type) = get_promotion_type(piece) {
               let position = check_for_promotion(self).unwrap();
               self. set_piece_at(&position, Some(Piece::new(piece_type, self.get_turn().clone())));

               post_turn_check(self);
               Some(self.state)

           }
           else {
               None
           }

        }
        else {
            None
        }

    }

    fn get_game_state(&self) -> GameState {
        self.state
    }

    fn get_turn(&self) -> Colour {
        self.turn
    }


    /// Converts a symbolic representation of a chess position into a [`Position`]. See [`Position::from_symbolic_representation`]
    ///
    /// If the [`Position`] contains a [`Piece`] that has the same [`Colour`] as the current turn, check for possible moves. See [`check_possible_moves`]
    ///
    /// Returns a [`Vec<String>`] contain the possible moves. Returns an empty [`Vec<String>`] if no possible moves was found.
    fn get_possible_moves(&self, position: &str) -> Vec<String> {
        let position = &Position::from_symbolic_representation(position);

        if let Some(piece) = self.get_piece_at(position) && piece.get_piece_color() == self.turn {
            check_possible_moves(position, self, &piece)
        }
        else {
            vec![]
        }
    }

    /// Converts the game into a fen string
    fn to_fen(&self) -> String {
        let mut fen = String::new();

        for y in (1..9).rev() {

            let mut counter = 0;
            for x in 1..9 {
                if let Some(piece) = self.get_piece_at(&Position::new(x, y)) {

                    if counter != 0 {
                        fen.push_str(&*counter.to_string());
                        counter = 0;
                    }

                    fen.push(piece.get_fen_representation())
                }
                else {
                    counter += 1
                }
            }

            if counter != 0 {
                fen.push_str(&*counter.to_string());
            }

            if y != 1 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(if self.get_turn().eq(&White) {'w'} else {'b'});
        fen.push_str(" - - ");
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_counter.to_string());

        fen
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
