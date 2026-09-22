use crate::{moves, Game, GameTraits, GameState};
use crate::piece::{Colour, Piece, Type};
use crate::piece::Colour::White;
use crate::piece::Type::{King, PAWN};
use crate::position::Position;

impl Game {
    pub(crate) fn get_piece_at(&self, position: &Position) -> Option<Piece> {
        self.board[(position.y - 1) as usize][(position.x - 1) as usize]
    }

    pub(crate) fn set_piece_at(&mut self, position: &Position, piece: Option<Piece> )  {
        self.board[(position.y - 1) as usize][(position.x - 1) as usize] = piece;
    }

    pub(crate) fn move_piece(&mut self, original_position: &Position, new_position: &Position) {
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
        [Some(Piece::new(PAWN, Colour::Black)); 8],
        create_first_layer_piece_row(Colour::Black),
    ]
}

fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(Type::ROOK, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(King, colour)), Some(Piece::new(Type::QUEEN, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::ROOK, colour))]
}

pub(crate) fn handle_in_progress(game: &mut Game, from: &str, to: &str) -> bool {
    if game.get_possible_moves(from).contains(&to.to_string()) {
        game.move_piece(&Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));
        false
    }
    else {
        true
    }
}

pub(crate) fn handle_check(game: &mut Game, from: &str, to: &str) -> bool {
    let mut game_check = game.clone();
    if game_check.get_possible_moves(from).contains(&to.to_string()) {
        game_check.move_piece(&Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));
    }
    else {
        return true;
    }

    if check_check(&game_check, &game_check.get_turn().get_opponent_color()).eq(&GameState::InProgress) {
        game.move_piece(&Position::from_symbolic_representation(from), &Position::from_symbolic_representation(to));
        false
    }
    else {
        true
    }

}

pub(crate) fn check_check(game: &Game, colour: &Colour) -> GameState {
    let mut king_piece_pos: String = "".to_string();

    'outer: for x in 1..9 {
        for y in 1..9 {
            if let Some(piece) = game.get_piece_at(&Position::new(x, y)) && piece.get_piece_type().eq(&King) && piece.get_piece_color().eq(&colour.get_opponent_color()) {
                king_piece_pos = Position::new(x,y).to_symbolic_representation();
                break 'outer;
            }

        }
    }

    if !moves::king::king_illegal_moves(&game, colour).contains(&king_piece_pos) {
         GameState::InProgress
    }
    else {
        GameState::Check
    }
}

pub(crate) fn check_check_mate(game: &mut Game) -> bool {
    for x in 1..9 {
        for y in 1..9 {
            if let Some(piece) = game.get_piece_at(&Position::new(x, y)) && piece.get_piece_color().eq(&game.get_turn()) {
                if !check_possible_moves_check(&Position::new(x, y), game, &piece, &game.get_turn().get_opponent_color()).is_empty() {
                    return false
                }
            }

        }
    }

    true
}

pub(crate) fn check_possible_moves_check(position: &Position, game: &Game, piece: &Piece, colour: &Colour) -> Vec<String> {
    let mut allowed_moves: Vec<String> = vec![];

    for moves in piece.get_possible_moves(position, game) {

        let mut game_test = game.clone();

        game_test.move_piece(position, &Position::from_symbolic_representation(&*moves));

        if check_check(&game_test, colour).eq(&GameState::InProgress) {

            if !allowed_moves.contains(&moves) {
               allowed_moves.push(moves);
            }
        }
    }

    allowed_moves
}

