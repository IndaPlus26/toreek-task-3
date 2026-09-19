
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pice {
    pice_type: Type,
    pice_color: Colour
}

impl Pice {
    pub(crate) fn new(pice_type: Type, pice_color: Colour) -> Pice {
        Pice { pice_type, pice_color }
    }
    pub fn get_pice_type(&self) -> Type {
        self.pice_type
    }

    pub fn get_pice_color(&self) -> Colour {
        self.pice_color
    }

    pub fn get_character_representation(&self) -> char {
        if self.pice_color.eq(&Colour::White) {
            match self.pice_type {
                Type::KING => '♔',
                Type::QUEEN => '♕',
                Type::ROOK => '♖',
                Type::BISHOP => '♗',
                Type::KNIGHT => '♘',
                Type::PAWN => '♙'
            }
        }

        else {
            match self.pice_type {
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


