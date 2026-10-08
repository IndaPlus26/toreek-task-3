// Author: Viola Söderlund
// Modified by: Isak Larsson

mod tests;

/// Contains definitions of a chess piece and related code.
pub mod piece;

/// Contains board logic; accessing and moving pieces and initializing the board.
pub mod board;

/// Contains definition of a chess position and position logic.
pub mod position;

/// Contains logic for how each chess piece should move.
pub mod moves;

use crate::GameState::*;
use crate::board::*;
use crate::piece::Colour::White;
use crate::piece::*;
use crate::position::Position;
use std::fmt;

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

    en_passant_position: Option<Position>,
    castling_queen_white: bool,
    castling_queen_black: bool,
    castling_king_black: bool,
    castling_king_white: bool
}

impl GameTraits for Game {
    fn new() -> Game {
        Game {
            state: InProgress,
            turn: White,
            board: create_default_board(),
            halfmove_clock: 0,
            fullmove_counter: 1,

            en_passant_position: None,
            castling_queen_white: true,
            castling_queen_black: true,
            castling_king_black: true,
            castling_king_white: true
        }
    }


    /// Entry point for moving a piece.
    /// Checks if the [`GameState`] allows for moves to be made. Moves are only allowed during [`Check`] and [`InProgress`].
    ///
    /// See [`handle_in_progress`] for piece movement logic.
    ///
    /// Returns [`None`] if the move was illegal or movement wasn't allowed.
    ///
    /// Calls [`post_turn_check`] to determinate the next [`GameState`].
    fn make_move(&mut self, from: &str, to: &str) -> Option<GameState> {
         let illegal_move = match self.state {
            InProgress => {
                !handle_in_progress(self, from, to)
            }
            Check => {
                !handle_in_progress(self, from, to)
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

    /// Entry point for making a promotion.
    /// Checks if the [`GameState`] allows for/equals [`Promoting`].
    /// Checks if the input is valid. See [`get_promotion_type`].
    ///
    /// If the input is valid, promote the piece and do the usual [`post_turn_check`].
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


    /// Converts a symbolic representation of a chess position into a [`Position`]. See [`Position::from_symbolic_representation`].
    ///
    /// If the [`Position`] contains a [`Piece`] that has the same [`Colour`] as the current turn, check for possible moves. See [`check_possible_moves`].
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

    /// Converts the game into a fen string.
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

        //Turn
        fen.push(' ');
        fen.push(if self.get_turn().eq(&White) {'w'} else {'b'});

        //Castling
        fen.push(' ');
        if self.castling_queen_black || self.castling_king_black || self.castling_queen_white || self.castling_king_white {
            if self.castling_king_white {
                fen.push('K')
            }
            if self.castling_queen_white {
                fen.push('Q')
            }
            if self.castling_king_black {
                fen.push('k')
            }
            if self.castling_queen_black {
                fen.push('q')
            }

        }
        else {
            fen.push('-');
        }
        
        //En passant
        fen.push(' ');
        if let Some(position) = &self.en_passant_position {
            fen.push_str(&*position.to_symbolic_representation().to_ascii_lowercase());
        }
        else {
            fen.push('-');
        }

        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        fen.push(' ');
        fen.push_str(&self.fullmove_counter.to_string());

        fen
    }
}

pub fn from_fen(fen: &str) -> Game {
    let mut game = Game::new();

    let mut split = fen.split_whitespace();

    let board_string = split.next().unwrap();
    let mut tmp_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

    let mut y = 7;
    for  str in board_string.split('/') {
        let mut pos = 0;

        for character in str.chars() {
            if character.is_numeric() {
                pos += character.to_digit(10).unwrap();
            }
            else {
                tmp_board[y][pos as usize] = char_to_fen_representation(character);

                pos += 1;
            }

        }
        y = y.wrapping_sub(1);
    }
    game.board = tmp_board;

    let tmp_color = split.next().unwrap();
    game.turn = if tmp_color.eq("w") {White} else {Colour::Black};

    let tmp_castling = split.next().unwrap();
    if tmp_castling.contains('K') {
        game.castling_king_white = true;
    }
    else {
        game.castling_king_white = false;
    }
    if tmp_castling.contains('Q') {
        game.castling_queen_white = true;
    }
    else {
        game.castling_queen_white = false;
    }
    if tmp_castling.contains('k') {
        game.castling_king_black = true;
    }
    else {
        game.castling_king_black = false;
    }
    if tmp_castling.contains('Q') {
        game.castling_queen_black = true;
    }
    else {
        game.castling_queen_black = false;
    }

    let tmp_en_passant = split.next().unwrap();
    if !tmp_en_passant.eq("-") {
        game.en_passant_position = Some(Position::from_symbolic_representation(tmp_en_passant));
    }

    let tmp_half_move_clock = split.next().unwrap();
    game.halfmove_clock = tmp_half_move_clock.parse::<u8>().unwrap();

    let full_move_clock = split.next().unwrap();
    game.fullmove_counter = full_move_clock.parse::<u32>().unwrap();
    
    game
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
