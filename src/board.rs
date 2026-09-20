use crate::{Game};
use crate::piece::{Colour, Piece, Type};
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
        create_first_layer_piece_row(Colour::White),
        [Some(Piece::new(Type::PAWN, Colour::White)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece::new(Type::PAWN, Colour::Black)); 8],
        create_first_layer_piece_row(Colour::Black),
    ]
}

fn create_first_layer_piece_row(colour: Colour) -> [Option<Piece>; 8] {
    [Some(Piece::new(Type::ROOK, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::King, colour)), Some(Piece::new(Type::QUEEN, colour)), Some(Piece::new(Type::BISHOP, colour)), Some(Piece::new(Type::KNIGHT, colour)), Some(Piece::new(Type::ROOK, colour))]
}


