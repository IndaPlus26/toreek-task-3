use crate::GameState::{Check, Checkmate, InProgress, Promoting, Stalemate};
use crate::piece::Colour::{Black, White};
use crate::piece::Type::{King, Bishop, Pawn, Queen, Rook, Knight};
use crate::piece::{Colour, Piece, Type};
use crate::position::Position;
use crate::{Game, GameTraits, moves};

impl Game {
    pub(crate) fn get_piece_at(&self, position: &Position) -> Option<Piece> {
        self.board[(position.y - 1) as usize][(position.x - 1) as usize]
    }

    pub(crate) fn set_piece_at(&mut self, position: &Position, piece: Option<Piece> )  {
        self.board[(position.y - 1) as usize][(position.x - 1) as usize] = piece;
    }

    /// Moves a piece without restrictions. Also resets or increment the [`Game.halfmove_clock`]
    pub(crate) fn move_piece(&mut self, original_position: &Position, new_position: &Position) {
        if self.get_piece_at(original_position).unwrap().get_piece_type().eq(&Pawn) {
            self.halfmove_clock = 0;
        }
        else if self.get_piece_at(new_position).is_some() {
            self.halfmove_clock = 0;
        }
        else {
            self.halfmove_clock += 1;
        }

        let piece = self.get_piece_at(original_position);

        self.set_piece_at(new_position, piece);
        self.set_piece_at(original_position, None);
    }
}

pub(crate) fn create_default_board() -> [[Option<Piece>; 8]; 8] {
    [
        create_first_layer_piece_row(White),
        [Some(Piece::new(Pawn, White)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece::new(Pawn, Black)); 8],
        create_first_layer_piece_row(Black),
    ]
}

fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(Rook, colour)), Some(Piece::new(Type::Knight, colour)), Some(Piece::new(Type::Bishop, colour)), Some(Piece::new(Queen, colour)), Some(Piece::new(King, colour)), Some(Piece::new(Type::Bishop, colour)), Some(Piece::new(Type::Knight, colour)), Some(Piece::new(Type::Rook, colour))]
}
/// Checks various thing right after a turn
///
/// Checks if any [`Pawn`] can promote. See [`check_for_promotion`]
///
/// Checks if the opponent is in [`Check`]. See [`check_for_check`]
/// Checks if the opponent can do any legal moves. See [`has_opponent_possible_moves`]
///
/// If the opponent is in [`Check`] and does not have any legal moves. Set [`game.state`] to [`Checkmate`]
/// If the opponent isn't in [`Check`] and does not have any legal moves. Set [`game.state`] to [`Stalemate`]
///
/// If the opponent can do any legal move, continues the game and changes turn. See [`Colour::get_opponent_color`]
/// Also increment the [`Game::fullmove_clock`] if applicable
pub(crate) fn post_turn_check(game: &mut Game) {
    if check_for_promotion(game).is_some() {
        game.state = Promoting;
        return;
    }

    game.state = if check_for_check(game) {Check} else {InProgress};
    if !has_opponent_possible_moves(game) {
        if game.state.eq(&Check) {
            game.state = Checkmate;
        }
        else {
            game.state = Stalemate;
        }

    }

    if game.halfmove_clock >= 100 {
        game.state = Stalemate;
    }

    if game.get_game_state().eq(&InProgress) || game.get_game_state().eq(&Check) {
        if game.get_turn().eq(&Black) {
            game.fullmove_counter += 1;
        }

        game.turn = game.turn.get_opponent_color();
    }
}


/// Checks the board for a [`Pawn`] than can promote
pub(crate) fn check_for_promotion(game: &Game) -> Option<Position> {
    let side = if game.get_turn().eq(&White) {8} else {1};

    for x in 1..9 {
        if let Some(piece) = game.get_piece_at(&Position::new(x, side)) && piece.get_piece_type().eq(&Pawn) {
            return Some(Position::new(x, side));
        }
    }
    None
}

/// Gets the piece [`Type`] from a String. Returns [`None`] if the str does not contain a valid promotion type.
pub(crate) fn get_promotion_type(piece: &str) -> Option<Type> {
    match piece.to_lowercase().as_str() {
        "q" => Some(Queen),
        "r" => Some(Rook),
        "b" => Some(Bishop),
        "n" => Some(Knight),

        _ => None,
    }

}

/// Checks if the opponent is in [`Check`]
/// See [`moves::king::king_illegal_moves`]
///
/// Returns true if the opponent [`King`] is in check
pub(crate) fn check_for_check(game: &Game) -> bool {
    let mut king_piece_pos: String = "".to_string();

    'outer: for x in 1..9 {
        for y in 1..9 {
            if let Some(piece) = game.get_piece_at(&Position::new(x, y)) && piece.get_piece_type().eq(&King) && piece.get_piece_color().eq(&game.get_turn().get_opponent_color()) {
                king_piece_pos = Position::new(x,y).to_symbolic_representation();
                break 'outer;
            }

        }
    }

    if moves::king::king_illegal_moves(&game, &game.get_turn()).contains(&king_piece_pos) {
        true
    }
    else {
        false
    }
}

/// Checks if the move is legal, see [`Game::get_possible_moves`]. If true moves the piece, see [`handle_move`] and returns true, else returns false.
pub(crate) fn handle_in_progress(game: &mut Game, from: &str, to: &str) -> bool {
    if game.get_possible_moves(from).contains(&to.to_string()) {
        handle_move(game, &Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));

        true
    }
    else {
        false
    }
}

/// Checks if an en passant was performed. See [`handle_en_passant`]. This move only removes an additional piece, so we can still move the piece normally.
/// Checks if a castling was performed. See [`handle_castling`]. If true, we don't want to move the piece normally, as castling provides a different movement pattern.
///
/// Else moves a piece normally. See [`Game::move_piece`].
pub(crate) fn handle_move(game: &mut Game, from: &Position, to: &Position) {
    handle_en_passant(game, &from, &to);
    if !handle_castling(game, &from, &to) {
        game.move_piece(&from, &to);
    }

}

/// Checks if a move is an en passant, or allows for an en passant next turn.
pub(crate) fn handle_en_passant(game: &mut Game, from: &Position, to: &Position) {
    if let Some(piece) = game.get_piece_at(from) && piece.get_piece_type().eq(&Pawn) {

        //En passant check
        let color_offset: i8 = if game.get_turn().eq(&White) { -1 } else { 1 };
        if let Some(position) = game.en_passant_position && to.eq(&position) {
            game.set_piece_at(&position.add_y(color_offset), None);
            game.en_passant_position = None;
        }
        else if (from.y - to.y).abs() == 2 { //Checks if a pawn has moved two steps
            game.en_passant_position = Some(to.add_y(color_offset));
        }
    }
    else {
        game.en_passant_position = None;
    }
}

/// Checks if castling still is allowed. See [`check_castling`]. Also handles a castling move
/// Returns true if the function handles castling, as it overrides the default movement behavior
///
/// The function checks for the position of the rook. A castling can both happen from selecting the king or a rook, so we don't know which position contains the rook.
/// See [`moves::rook::rook_castling`] and [`moves::king::king_castling`]
pub(crate) fn handle_castling(game: &mut Game, from: &Position, to: &Position) -> bool {
    let mut king_check = false;
    let mut rook_position: Option<Position> = None;

    if let Some(from_king) = game.get_piece_at(from) && from_king.get_piece_type().eq(&King) && from_king.get_piece_color().eq(&game.get_turn()) {
        king_check = true;
    }
    else if let Some(to_king) = game.get_piece_at(to) && to_king.get_piece_type().eq(&King) && to_king.get_piece_color().eq(&game.get_turn()){
        king_check = true;
    }

    if let Some(from_rook) = game.get_piece_at(from) && from_rook.get_piece_type().eq(&Rook) && from_rook.get_piece_color().eq(&game.get_turn()) {
        rook_position = Some(*from);
    }
    else if let Some(to_rook) = game.get_piece_at(to) && to_rook.get_piece_type().eq(&Rook) && to_rook.get_piece_color().eq(&game.get_turn()) {
        rook_position = Some(*to);
    }

    if let Some(position) = rook_position && king_check {
        if position.eq(&Position::new(1, 1)) {
            game.move_piece(&position, &Position::new(4, 1)); //Move rook
            game.move_piece(&Position::new(5, 1), &Position::new(3, 1)); //Move kin

            game.castling_king_white = false;
            game.castling_queen_white = false;
        }
        else if position.eq(&Position::new(8, 1)) {
            game.move_piece(&position, &Position::new(6, 1)); //Move rook
            game.move_piece(&Position::new(5, 1), &Position::new(7, 1)); //Move king

            game.castling_king_white = false;
            game.castling_queen_white = false;
        }

        else if position.eq(&Position::new(1, 8)) {
            game.move_piece(&position, &Position::new(4, 8)); //Move rook
            game.move_piece(&Position::new(5, 8), &Position::new(3, 8)); //Move king

            game.castling_king_black = false;
            game.castling_queen_black = false;
        }
        else if position.eq(&Position::new(8, 8)) {
            game.move_piece(&position, &Position::new(6, 8)); //Move rook
            game.move_piece(&Position::new(5, 8), &Position::new(7, 8)); //Move king

            game.castling_king_black = false;
            game.castling_queen_black = false;
        }

        game.halfmove_clock -=1; //We move two pieces.
        true
    }
    else {
        check_castling(game, from);
        false
    }
}

/// Checks if castling still is allowed. Checks if a [`Rook`] has moved, and from what [`Position`], and if a king has moved and from what [`Position`]
pub(crate) fn check_castling(game: &mut Game, from: &Position) {
    if let Some(piece) = game.get_piece_at(from) {
        if piece.get_piece_type().eq(&Rook) {
            if from.eq(&Position::new(1, 1)) {
                game.castling_queen_white = false;
            } else if from.eq(&Position::new(8, 1)) {
                game.castling_king_white = false;

            }
            if from.eq(&Position::new(1, 8)) {
                game.castling_queen_black = false;

            } else if from.eq(&Position::new(8, 8)) {
                game.castling_king_black = false;
            }
        } else if piece.get_piece_type().eq(&King) {
            if from.eq(&Position::new(5, 1)) {

                game.castling_king_white = false;
                game.castling_queen_white = false;
            }
            else if from.eq(&Position::new(5, 8)) {

                game.castling_king_black = false;
                game.castling_queen_black = false;
            }
        }
    }
}


/// Recursively goes through every possible move for every [`Piece`] on the board for the opposite player
/// Checks if the opponent can do any legal moves. Used to determinate [`Checkmate`] or [`Stalemate`]
pub(crate) fn has_opponent_possible_moves(game: &mut Game) -> bool {
    let mut game_test = game.clone();
    game_test.turn = game.get_turn().get_opponent_color();

    for x in 1..9 {
        for y in 1..9 {
            if let Some(piece) = game_test.get_piece_at(&Position::new(x, y)) && piece.get_piece_color().eq(&game_test.get_turn()) {
                if !check_possible_moves(&Position::new(x, y), &game_test, &piece).is_empty() {
                    return true
                }
            }
        }
    }
    false
}

/// Checks if a [`Piece`] has any legal moves on a specific [`Position`].
///
/// Gets all moves a [`Piece`] can do, see [`Piece::get_possible_moves`] for per [`Type`] logic.
///
/// Checks if any moves puts the own [`King`] into check. See [`handle_move`] and [`check_for_check`].
/// Moves that puts the own [`King`] into check are considered illegal and are not added to the final [`Vec<String>`] of allowed_moves.
/// Returns a [`Vec<String>`] of all legal moves a [`Piece`] can do.
pub(crate) fn check_possible_moves(position: &Position, game: &Game, piece: &Piece) -> Vec<String> {
    let mut allowed_moves: Vec<String> = vec![];

    for moves in piece.get_possible_moves(position, game) {

        let mut game_test = game.clone();

        handle_move(&mut game_test, position, &Position::from_symbolic_representation(&*moves));

        game_test.turn = game.get_turn().get_opponent_color();
        if !check_for_check(&game_test) {

            if !allowed_moves.contains(&moves) {
               allowed_moves.push(moves);
            }
        }
    }

    allowed_moves
}

