use crate::{create_first_layer_piece_row, Game, GameTraits};
use crate::piece::{Colour, Piece, Type};

impl Game {
    pub(crate) fn get_piece_at(&self, position: (u8, u8)) -> Option<Piece> {

        self.board[(position.1 - 1) as usize][(position.0 - 1) as usize]
    }

    pub(crate) fn set_piece_at(&mut self, position: (u8, u8), piece: Option<Piece> )  {

        self.board[(position.1 - 1) as usize][(position.0 - 1) as usize] = piece;
    }

    pub(crate) fn move_piece(&mut self, original_position: (u8, u8), new_position: (u8, u8)) {
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

pub(crate) fn position_from_string(position: &str) -> (u8, u8) {
    (get_number_from_letter(position.chars().nth(0).unwrap()), position.chars().nth(1).unwrap().to_digit(10).unwrap() as u8)
}

pub(crate) fn position_to_string(position: (u8, u8)) -> String {
    get_letter_from_number(position.0).to_string() + position.1.to_string().as_str()
}

fn get_number_from_letter(letter: char) -> u8 {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        _ => 1
    }
}

fn get_letter_from_number(letter: u8) -> char {
    match letter {
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        5 => 'E',
        6 => 'F',
        7 => 'G',
        8 => 'H',
        _ => 'A'
    }
}