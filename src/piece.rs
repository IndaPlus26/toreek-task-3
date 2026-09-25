use crate::moves;
use crate::piece::Colour::{Black, White};
use crate::piece::Type::*;
use crate::position::Position;
use crate::Game;

/// A struct containing a representation of a chess piece.
/// Holds a [`Type`] e.g. [`Knight`].
/// Holds a [`Colour`] e.g. [`White`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    piece_type: Type,
    piece_color: Colour
}

impl Piece {
    pub(crate) fn new(piece_type: Type, piece_color: Colour) -> Piece {
        Piece { piece_type, piece_color }
    }
    pub(crate) fn get_piece_type(&self) -> Type {
        self.piece_type
    }

    pub(crate) fn get_piece_color(&self) -> Colour {
        self.piece_color
    }

    /// Returns a [`Vec<String>`] of each possible move a piece can do.
    /// Refers to a function based of the [`Type`].
    /// See [`moves`] for more details.
    pub(crate) fn get_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        match self.piece_type {
            King => moves::king::king_possible_moves(position, game),
            Queen => moves::queen::queen_possible_moves(position, game),
            Rook => moves::rook::rook_possible_moves(position, game, vec![]),
            Bishop =>moves::bishop::bishop_possible_moves(position, game, vec![]),
            Knight => moves::knight::knight_possible_moves(position, game),
            Pawn => moves::pawn::pawn_possible_moves(position, game),
        }
    }

    /// Converts a [`Type`] into a symbolic representation.
    /// For debug purposes only.
    pub fn get_character_representation(&self) -> char {
        if self.piece_color.eq(&Black) {
            match self.piece_type {
                King => '♔',
                Queen => '♕',
                Rook => '♖',
                Bishop => '♗',
                Knight => '♘',
                Pawn => '♙'
            }
        }

        else {
            match self.piece_type {
                King => '♚',
                Queen => '♛',
                Rook => '♜',
                Bishop => '♝',
                Knight => '♞',
                Pawn => '♟'
            }
        }
    }

    pub fn get_fen_representation(&self) -> char {
        let mut representation = match self.piece_type {
            King => 'k',
            Queen => 'q',
            Rook => 'r',
            Bishop => 'b',
            Knight => 'n',
            Pawn => 'p'
        };

        if self.piece_color.eq(&White) {
            representation = representation.to_ascii_uppercase()
        }

        representation
    }
}




#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn get_opponent_color(&self) -> Colour {
        if *self == White {
            Black
        }
        else {
            White
        }
    }

}
