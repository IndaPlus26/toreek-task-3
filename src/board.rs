use crate::GameState::{Check, Checkmate, InProgress, Promoting, Stalemate};
use crate::piece::Colour::{Black, White};
use crate::piece::Type::{King, BISHOP, PAWN, QUEEN, ROOK, KNIGHT};
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
        if self.get_piece_at(original_position).unwrap().get_piece_type().eq(&PAWN) {
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
        [Some(Piece::new(PAWN, White)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece::new(PAWN, Black)); 8],
        create_first_layer_piece_row(Black),
    ]
}

fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(ROOK, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(QUEEN, colour)), Some(Piece::new(King, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::ROOK, colour))]
}
/// Checks various thing right after a turn
///
/// Checks if any [`PAWN`] can promote. See [`check_for_promotion`]
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


/// Checks the board for a [`PAWN`] than can promote
pub(crate) fn check_for_promotion(game: &Game) -> Option<Position> {
    let side = if game.get_turn().eq(&White) {8} else {1};

    for x in 1..9 {
        if let Some(piece) = game.get_piece_at(&Position::new(x, side)) && piece.get_piece_type().eq(&PAWN) {
            return Some(Position::new(x, side));
        }
    }
    None
}

/// Gets the piece [`Type`] from a String. Returns [`None`] if the str does not contain a valid promotion type.
pub(crate) fn get_promotion_type(piece: &str) -> Option<Type> {
    match piece.to_lowercase().as_str() {
        "q" => Some(QUEEN),
        "r" => Some(ROOK),
        "b" => Some(BISHOP),
        "n" => Some(KNIGHT),

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

/// Checks if the move is legal, see [`Game::get_possible_moves`]. If true moves the piece, see [`Game::move_piece`] and returns true, else returns false
pub(crate) fn handle_in_progress(game: &mut Game, from: &str, to: &str) -> bool {
    if game.get_possible_moves(from).contains(&to.to_string()) {
        game.move_piece(&Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));
        false
    }
    else {
        true
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

/// Checks if a [`Piece`] has any legal moves on a specific [`Position`]
///
/// Gets all moves a [`Piece`] can do, see [`Piece::get_possible_moves`] for per [`Type`] logic
///
/// Checks if any moves puts the own [`King`] into check. See [`check_for_check`]
/// Moves that puts the own [`King`] into check are considered illegal and are not added to the final [`Vec<String>`] of allowed_moves
/// Returns a [`Vec<String>`] of all legal moves a [`Piece`] can do
pub(crate) fn check_possible_moves(position: &Position, game: &Game, piece: &Piece) -> Vec<String> {
    let mut allowed_moves: Vec<String> = vec![];

    for moves in piece.get_possible_moves(position, game) {

        let mut game_test = game.clone();

        game_test.turn = game.get_turn().get_opponent_color();
        game_test.move_piece(position, &Position::from_symbolic_representation(&*moves));

        if !check_for_check(&game_test) {

            if !allowed_moves.contains(&moves) {
               allowed_moves.push(moves);
            }
        }
    }

    allowed_moves
}

