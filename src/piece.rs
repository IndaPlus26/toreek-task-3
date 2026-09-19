use crate::board::position_to_string;
use crate::Game;

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


    //TODO
    pub(crate) fn get_possible_moves(&self, position: (u8, u8), game: &Game) -> Vec<String> {
        println!("{:?}", position);
        let position = (position.0, position.1 + 1);

        if is_out_of_bounds(position) {
            println!("Out of bounds position: ({}, {})", position.0, position.1);
            vec![]
        }
        else if is_friendly_piece_at(position, &game) {
            println!("Friendly piece: ({}, {})", position.0, position.1);
            vec![]
        }
        else {
            println!("Sucess");
            vec![position_to_string(position)]
        }
    }



    pub fn get_character_representation(&self) -> char {
        if self.piece_color.eq(&Colour::Black) {
            match self.piece_type {
                Type::KING => '♔',
                Type::QUEEN => '♕',
                Type::ROOK => '♖',
                Type::BISHOP => '♗',
                Type::KNIGHT => '♘',
                Type::PAWN => '♙'
            }
        }

        else {
            match self.piece_type {
                Type::KING => '♚',
                Type::QUEEN => '♛',
                Type::ROOK => '♜',
                Type::BISHOP => '♝',
                Type::KNIGHT => '♞',
                Type::PAWN => '♟'
            }
        }
    }
}

fn is_friendly_piece_at(position: (u8, u8), game: &Game) -> bool {
    if let Some(piece) = game.get_piece_at(position) {
        if piece.piece_color == game.turn {
            true
        }
        else {
            false
        }
    }
    else {
        false
    }


}

fn is_out_of_bounds(position: (u8, u8)) -> bool {
    if position.0 < 1 || position.1 < 1 || position.0 > 9 || position.1 > 9 {
        true
    } else {
        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    KING,
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


