use crate::Game;
use crate::piece::Colour::{Black, White};
use crate::position::Position;

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
    pub(crate) fn get_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        match self.piece_type {
            Type::KING => todo!(),
            Type::QUEEN => self.queen_possible_moves(position, game),
            Type::ROOK => self.rook_possible_moves(position, game, vec![]),
            Type::BISHOP => self.bishop_possible_moves(position, game, vec![]),
            Type::KNIGHT => self.knight_possible_moves(position, game),
            Type::PAWN => self.pawn_possible_moves(position, game),
        }
    }

    pub(crate) fn queen_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        self.rook_possible_moves(position, game, self.bishop_possible_moves(position, game, vec![]))
    }

    pub(crate) fn rook_possible_moves(&self, position: &Position, game: &Game, mut valid_positions: Vec<String>) -> Vec<String> {
        for direction_multiplier in (-1..2).step_by(2) { //Loops for -1 and 1
            for step_multiplier in 0..2 { //Loops for 0 and 1
                for step in 1..9 {

                    let test_position = &position.add(direction_multiplier * step_multiplier * step , direction_multiplier * (1 - step_multiplier) * step);
                    if !add_position_if_valid(test_position, game, &mut valid_positions) || test_position.is_enemy_piece_at(game) {
                        break;
                    }
                }
            }
        }

        valid_positions
    }

    pub(crate) fn bishop_possible_moves(&self, position: &Position, game: &Game, mut valid_positions: Vec<String>) -> Vec<String> {
        for x_multiplier in (-1..2).step_by(2) { //Loops for -1 and 1
            for y_multiplier in (-1..2).step_by(2) {
                for step in 1..9 {

                    let test_position = &position.add(step * x_multiplier, step * y_multiplier);
                    if !add_position_if_valid(test_position, game, &mut valid_positions) || test_position.is_enemy_piece_at(game) {
                        break;
                    }
                }
            }
        }

        valid_positions
    }

    pub(crate) fn knight_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        let mut valid_positions: Vec<String> = vec![];

        add_position_if_valid(&position.add(1, 2), game, &mut valid_positions);
        add_position_if_valid(&position.add(-1, 2), game, &mut valid_positions);
        add_position_if_valid(&position.add(1, -2), game, &mut valid_positions);
        add_position_if_valid(&position.add(-1, -2), game, &mut valid_positions);

        add_position_if_valid(&position.add(2, 1), game, &mut valid_positions);
        add_position_if_valid(&position.add(-2, 1), game, &mut valid_positions);
        add_position_if_valid(&position.add(2, -1), game, &mut valid_positions);
        add_position_if_valid(&position.add(-2, -1), game, &mut valid_positions);

        valid_positions
    }

    pub(crate) fn pawn_possible_moves(&self, position: &Position, game: &Game) -> Vec<String> {
        let mut valid_positions: Vec<String> = vec![];

        //White pawns can only move up and black pawn can only move down
        let color_multiplier: i8 = if game.turn == White {1} else {-1};

        Self::pawn_move_straight(position, game, color_multiplier, &mut valid_positions);

        //Check if pawn can be moved two pieces ahead
        if (game.turn == White && position.y == 2) || (game.turn == Black && position.y == 7) {
            Self::pawn_move_straight(position, game, color_multiplier * 2, &mut valid_positions);

        }

        Self::pawn_move_side(position, game, 1, color_multiplier, &mut valid_positions);
        Self::pawn_move_side(position, game, -1, color_multiplier, &mut valid_positions);


        valid_positions
    }

    fn pawn_move_straight(position: &Position, game: &Game, offset: i8, valid_positions: &mut Vec<String>) {

        let test_pos = position.add_y(offset);
        if !test_pos.is_enemy_piece_at(game) {
            add_position_if_valid(&test_pos, game, valid_positions);
        }
    }

    fn pawn_move_side(position: &Position, game: &Game, offset_x: i8, offset_y: i8, valid_positions: &mut Vec<String>) {

        let test_pos = position.add(offset_x, offset_y);
        if test_pos.is_enemy_piece_at(game) {
            add_position_if_valid(&test_pos, game, valid_positions);
        }
    }


    /*
    DEBUG
    */
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

fn add_position_if_valid(position: &Position, game: &Game, positions: &mut Vec<String>) -> bool {
    if position.is_valid_position(game) {
            positions.push(position.to_symbolic_representation());
        true
    }
    else {
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
