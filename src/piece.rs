
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    piece_type: Type,
    piece_color: Colour
}

impl Piece {
    pub(crate) fn new(piece_type: Type, piece_color: Colour) -> Piece {
        Piece { piece_type, piece_color }
    }
    pub fn get_piece_type(&self) -> Type {
        self.piece_type
    }

    pub fn get_piece_color(&self) -> Colour {
        self.piece_color
    }

    pub fn get_character_representation(&self) -> char {
        if self.piece_color.eq(&Colour::White) {
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


