use crate::{Game, GameTraits};
use crate::piece::Colour::*;
use crate::piece::Type::*;
use crate::position::Position;

/// Contains the logic behind the move pattern for type [`King`]
/// See [`king_possible_moves`] for entry point
pub(crate) mod king {

    use super::*;

    /// Entry point for [`King`] move pattern logic
    ///
    /// "Illegal" moves are considered moves that puts the king into a threatening position. See [`king_illegal_moves`]
    /// Subtracts the [`king_illegal_moves`] from the kings default move pattern.
    /// For default move pattern, see [`king_moves`]
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`King`]
    pub(crate) fn king_possible_moves(position: &Position, game: &Game) -> Vec<String> {
        let mut moves = king_moves(position, game);

        for illegal_move in king_illegal_moves(game) {
            moves.retain(|x| !x.eq(&illegal_move));
        }

        moves
    }

    /// Returns a [`Vec<String>`] of every possible move the opposite player can do, that can threaten another piece.
    /// Observe that a [`PAWN`], unlike other [`Piece`]s can move forwards, but can only threaten pieces diagonally
    ///
    /// Loops every [`Position`] of [`crate::Game::board`]
    /// Checks for [`Piece`] of opposite [`Colour`]
    ///
    /// If the [`Piece`] is a [`King`] check the positions around the king. See [`king_moves`]. We can't reuse [`Piece::get_possible_moves`] as it would refer to this metod, resulting in a loop.
    /// If the [`Piece`] is a [`PAWN`] check the positions diagonally in front of the [`PAWN`].  We can't reuse [`Piece::get_possible_moves`] as the default move pattern doesn't neccessarely threaten other pieces.
    /// If the [`Piece`] is any other piece, get their possible moves from [`Piece::get_possible_moves`]
    fn king_illegal_moves(game: &Game) -> Vec<String> {
        let mut illegal_moves: Vec<String> = vec![];

        for x in 1..9 {
            for y in 1..9 {
                let test_position = Position::new(x, y);

                if let Some(piece) = game.get_piece_at(&test_position) && piece.get_piece_color().eq(&game.get_turn().get_opponent_color()) {
                    if piece.get_piece_type().eq(&King) {
                        illegal_moves.extend(king_moves(&test_position, game));
                    } else if piece.get_piece_type().eq(&PAWN) {
                        let color_multiplier: i8 = if game.turn.get_opponent_color() == White { 1 } else { -1 };
                        add_position_if_valid(&test_position.add(1, color_multiplier), game, &mut illegal_moves);
                        add_position_if_valid(&test_position.add(-1, color_multiplier), game, &mut illegal_moves);
                    } else {
                        illegal_moves.extend(piece.get_possible_moves(&test_position, game));
                    }
                }
            }
        }

        illegal_moves
    }

    /// Returns a 3x3 grid around the [`Position`] of the [`King`], excluding the position in the middle.
    /// Excludes invalid positions, see [`add_position_if_valid`] for details.
    fn king_moves(position: &Position, game: &Game) -> Vec<String> {
        let mut positions: Vec<String> = vec![];

        for x in -1..2 {
            for y in -1..2 {
                if x != 0 || y != 0 {
                    add_position_if_valid(&position.add(x, y), game, &mut positions);
                }
            }
        }

        positions
    }
}

pub(crate) mod queen {
    use super::*;

    /// Entry point for [`QUEEN`] move pattern logic
    ///
    /// The [`QUEEN`] can both move according to the move pattern logic of [`ROOK`] and [`BISHOP`]
    /// See [`rook::rook_possible_moves`] and [`bishop::bishop_possible_moves`]
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`QUEEN`]
    pub(crate) fn queen_possible_moves(position: &Position, game: &Game) -> Vec<String> {
        rook::rook_possible_moves(position, game, bishop::bishop_possible_moves(position, game, vec![]))
    }
}

pub(crate) mod rook {
    use super::*;

    /// Entry point for [`ROOK`] move pattern logic
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`ROOK`]
    pub(crate) fn rook_possible_moves(position: &Position, game: &Game, mut valid_positions: Vec<String>) -> Vec<String> {
        for direction_multiplier in (-1..2).step_by(2) { //Loops for -1 and 1
            for step_multiplier in 0..2 { //Loops for 0 and 1
                for step in 1..9 {
                    let test_position = &position.add(direction_multiplier * step_multiplier * step, direction_multiplier * (1 - step_multiplier) * step);
                    if !add_position_if_valid(test_position, game, &mut valid_positions) || test_position.is_enemy_piece_at(game) {
                        break;
                    }
                }
            }
        }

        valid_positions
    }
}


pub(crate) mod bishop {
    use super::*;

    /// Entry point for [`BISHOP`] move pattern logic
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`BISHOP`]
    pub(crate) fn bishop_possible_moves(position: &Position, game: &Game, mut valid_positions: Vec<String>) -> Vec<String> {
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
}
pub(crate) mod knight {
    use super::*;

    /// Entry point for [`KNIGHT`] move pattern logic
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`KNIGHT`]
    pub(crate) fn knight_possible_moves(position: &Position, game: &Game) -> Vec<String> {
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
}

pub(crate) mod pawn {
    use super::*;

    /// Entry point for [`PAWN`] move pattern logic
    ///
    /// The [`PAWN`] can only move towards the other player. The `color_multiplier` variable is used for this purpose.
    ///
    /// Returns a [`Vec<String>`] of the possible moves for type [`PAWN`]
    pub(crate) fn pawn_possible_moves(position: &Position, game: &Game) -> Vec<String> {
        let mut valid_positions: Vec<String> = vec![];

        //White pawns can only move up and black pawn can only move down
        let color_multiplier: i8 = if game.turn == White {1} else {-1};

        pawn_move_straight(position, game, color_multiplier, &mut valid_positions);

        //Check if pawn can be moved two pieces ahead
        if (game.turn == White && position.y == 2) || (game.turn == Black && position.y == 7) {
            pawn_move_straight(position, game, color_multiplier * 2, &mut valid_positions);

        }

        pawn_move_side(position, game, 1, color_multiplier, &mut valid_positions);
        pawn_move_side(position, game, -1, color_multiplier, &mut valid_positions);


        valid_positions
    }

    /// Checks if a [`PAWN`] can move straight.
    fn pawn_move_straight(position: &Position, game: &Game, offset: i8, valid_positions: &mut Vec<String>) {

        let test_pos = position.add_y(offset);
        if !test_pos.is_enemy_piece_at(game) {
            add_position_if_valid(&test_pos, game, valid_positions);
        }
    }

    /// Checks if a [`PAWN`] can move diagonally.
    fn pawn_move_side(position: &Position, game: &Game, offset_x: i8, offset_y: i8, valid_positions: &mut Vec<String>) {

        let test_pos = position.add(offset_x, offset_y);
        if test_pos.is_enemy_piece_at(game) {
            add_position_if_valid(&test_pos, game, valid_positions);
        }
    }
}


/// Checks if a position is valid and adds the position to a [`Vec<String>`] if true
/// See [`Position::is_valid_position`] for details.
/// Returns `true` if the position is valid
pub(crate) fn add_position_if_valid(position: &Position, game: &Game, positions: &mut Vec<String>) -> bool {
    if position.is_valid_position(game) {
        positions.push(position.to_symbolic_representation());
        true
    } else {
        false
    }
}
