use crate::Game;

pub struct Position {
    pub x: i8,
    pub y: i8
}

impl Position {
    pub fn new(x: i8, y: i8) -> Position {
        Position { x, y }
    }

    pub fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn add(&self, x_offset: i8, y_offset: i8) -> Position {
        Position::new(self.x.wrapping_add(x_offset), self.y.wrapping_add(y_offset))
    }

    pub fn add_x(&self, x_offset: i8) -> Position {
        Position::new(self.x.wrapping_add(x_offset), self.y)
    }

    pub fn add_y(&self, y_offset: i8) -> Position {
        Position::new(self.x, self.y.wrapping_add(y_offset))
    }
    pub fn is_out_of_bounds(&self) -> bool {
        self.x < 1 || self.y < 1 || self.x > 8 || self.y > 8
    }

    pub fn is_valid_position(&self, game: &Game) -> bool {
        !self.is_out_of_bounds() && !self.is_friendly_piece_at(&game)
    }

    pub fn is_friendly_piece_at(&self, game: &Game) -> bool {
        if let Some(piece) = game.get_piece_at(self) {
            if piece.get_piece_color() == game.turn {
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

    pub(crate) fn is_enemy_piece_at(&self, game: &Game) -> bool {
        if self.is_out_of_bounds() {
            false
        }

        else if let Some(piece) = game.get_piece_at(self) {
            if piece.get_piece_color() != game.turn {
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

    pub fn to_symbolic_representation(&self) -> String {
        get_letter_from_number(self.x).to_string() + self.y.to_string().as_str()
    }

    pub fn from_symbolic_representation(symbolic_representation: &str) -> Position {
        Position::new(get_number_from_letter(symbolic_representation.chars().nth(0).unwrap()), symbolic_representation.chars().nth(1).unwrap().to_digit(10).unwrap() as i8)
    }
}

pub fn get_number_from_letter(letter: char) -> i8 {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
         _  => 1
    }
}

pub fn get_letter_from_number(letter: i8) -> char {
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