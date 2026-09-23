use crate::{Game};
use crate::moves;
use crate::piece::Colour::{Black, White};
use crate::piece::Type::*;
use crate::position::Position;

/// A struct containing a representation of a chess piece
/// Holds a [`Type`] e.g. [`KNIGHT`]
/// Holds a [`Colour`] e.g. [`White`]
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
    /// Refers to a function based of the [`Type`]
    /// See [`moves`] for more details
    pub(crate) fn get_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        match self.piece_type {
            King => moves::king::king_possible_moves(position, game),
            QUEEN => moves::queen::queen_possible_moves(position, game),
            ROOK => moves::rook::rook_possible_moves(position, game, vec![]),
            BISHOP =>moves::bishop::bishop_possible_moves(position, game, vec![]),
            KNIGHT => moves::knight::knight_possible_moves(position, game),
            PAWN => moves::pawn::pawn_possible_moves(position, game),
        }
    }

    /// Converts a [`Type`] into a symbolic representation
    /// For debug purposes only
    pub fn get_character_representation(&self) -> char {
        if self.piece_color.eq(&Black) {
            match self.piece_type {
                King => '♔',
                QUEEN => '♕',
                ROOK => '♖',
                BISHOP => '♗',
                KNIGHT => '♘',
                PAWN => '♙'
            }
        }

        else {
            match self.piece_type {
                King => '♚',
                QUEEN => '♛',
                ROOK => '♜',
                BISHOP => '♝',
                KNIGHT => '♞',
                PAWN => '♟'
            }
        }
    }

    pub fn get_fen_representation(&self) -> char {
        let mut representation = match self.piece_type {
            King => 'k',
            QUEEN => 'q',
            ROOK => 'r',
            BISHOP => 'b',
            KNIGHT => 'n',
            PAWN => 'p'
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
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
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
