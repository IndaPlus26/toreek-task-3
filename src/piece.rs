use crate::board::position_to_string;
use crate::Game;
use crate::piece::Colour::{Black, White};

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
        match self.piece_type {
            Type::PAWN => self.pawn_possible_moves(position, game),
            _ => todo!(),
        }
    }

    pub(crate) fn pawn_possible_moves(&self, position: (u8, u8), game: &Game) -> Vec<String> {
        let mut valid_positions: Vec<String> = vec![];

        //White pawns can only move up and black pawn can only move down
        let color_multiplier: i8 = if game.turn == White {1} else {-1};

        if !is_enemy_piece_at((position.0, (position.1 as i8 + color_multiplier) as u8), game) {
            add_position_if_valid((position.0, (position.1 as i8 + color_multiplier) as u8), game, &mut valid_positions);
        }

        //Check if pawn can be moved two pieces ahead
        if (game.turn == White && position.1 == 2) || (game.turn == Black && position.1== 7) {
            if !is_enemy_piece_at((position.0, (position.1 as i8 + color_multiplier * 2) as u8), game) {
                add_position_if_valid((position.0, (position.1 as i8 + color_multiplier * 2) as u8), game, &mut valid_positions);
            }
        }

        if is_enemy_piece_at((position.0 + 1, (position.1 as i8 + color_multiplier) as u8), game) {
            add_position_if_valid((position.0 + 1, (position.1 as i8 + color_multiplier) as u8), game, &mut valid_positions);
        }

        if is_enemy_piece_at((position.0 - 1, (position.1 as i8 + color_multiplier) as u8), game) {
            add_position_if_valid((position.0 - 1, (position.1 as i8 + color_multiplier) as u8), game, &mut valid_positions);
        }


        valid_positions
    }



    pub fn get_character_representation(&self) -> char {
        if self.piece_color.eq(&Black) {
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

fn add_position_if_valid(position: (u8, u8), game: &Game, positions: &mut Vec<String>) -> bool {
    if is_valid_position(position, game) {
            positions.push(position_to_string(position));
        true
    }
    else {
        false
    }

}

fn is_valid_position(position: (u8, u8), game: &Game) -> bool {
    !is_friendly_piece_at(position, &game) && !is_out_of_bounds(position)
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

fn is_enemy_piece_at(position: (u8, u8), game: &Game) -> bool {
    if is_out_of_bounds(position) {
         false
    }

    else if let Some(piece) = game.get_piece_at(position) {
        if piece.piece_color != game.turn {
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
