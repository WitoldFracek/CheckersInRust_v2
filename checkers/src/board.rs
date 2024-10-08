use std::fmt::{Display, Formatter};
use crate::controller::{CheckersColor, Figure};
use crate::colors::colors as colors;

macro_rules! set_bit {
    ($board: expr, $field: ident, $x: expr, $y: expr, $value: expr) => {
        assert!($value == 1 || $value == 0);
        let shift = $x / 2 + $y * 4;
        if $value == 1 {
            $board.$field |= 1 << shift;
        } else {
            $board.$field &= !(1 << shift);
        }
    };
}

pub fn alias(x: u8, y: u8) -> String {
    assert!(Board::in_range(x), "");
    assert!(Board::in_range(y));
    let mut ret = String::with_capacity(2);
    let col = (x + b'A') as char;
    let row: char = (y + b'1') as char;
    ret.push(col);
    ret.push(row);
    ret
}

pub fn coords_from_alias(alias: &str) -> (u8, u8) {
    assert_eq!(alias.len(), 2, "invalid alias - unknown board position {alias:?}");
    let letter = alias.chars().next().expect("alias should have length equal to 2").to_ascii_uppercase();
    let index = alias.chars().nth(1).expect("alias should have length equal to 2");
    assert!(('A'..='H').contains(&letter), "invalid alias - unknown letter board position {alias:?}");
    assert!(('1'..='8').contains(&index), "invalid alias - unknown number board position {alias:?}");
    let x = letter as u8 - b'A';
    let y = index as u8 - b'1';
    (x, y)
}

#[derive(Copy, Clone)]
pub struct Square {
    pub figure: Option<Figure>,
    pub was_jumped_over: bool
}

impl Square {
    pub fn new(figure: Option<Figure>, was_jumped_over: bool) -> Self {
        Self { figure, was_jumped_over }
    }

    pub fn has_figure(&self) -> bool {
        self.figure.is_some()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub occupation: u32,
    pub color: u32,
    pub figure: u32,
    pub flags: u32,
}

impl Board {

    pub fn empty() -> Self {
        Self {occupation: 0, color: 0, figure: 0, flags: 0}
    }

    pub fn from_str_repr(repr: &str, empty: char, white_pieces: (char, char), black_pieces: (char, char)) -> Self {
        let mut ret = Self::empty();
        repr.split("\n").enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                let figure = if c == empty {
                    None
                } else if c == white_pieces.0 {
                    Some(Figure::Pawn(CheckersColor::White))
                } else if c == white_pieces.1 {
                    Some(Figure::Queen(CheckersColor::White))
                } else if c == black_pieces.0 {
                    Some(Figure::Pawn(CheckersColor::Black))
                } else {
                    Some(Figure::Queen(CheckersColor::Black))
                };
                ret.set(x as u8, y as u8, figure);
            })
        });
        ret
    }

    pub fn from_positions(positions: &[(u8, u8, Figure)]) -> Self {
        let mut ret = Self::empty();
        for &(x, y, figure) in positions {
           ret.set(x, y, Some(figure));
        }
        ret
    }

    pub fn from_alias_positions(positions: &[(&str, Figure)]) -> Self {
        let mut ret = Self::empty();
        for &(alias, figure) in positions {
           let (x, y) = coords_from_alias(alias);
            ret.set(x, y, Some(figure));
        }
        ret
    }

    pub fn at_alias(&self, alias: &str) -> Option<Figure> {
        assert_eq!(alias.len(), 2, "invalid alias - unknown board position {alias:?}");
        let letter = alias.chars().next()?.to_ascii_uppercase();
        let index = alias.chars().nth(1)?;
        assert!(('A'..='H').contains(&letter), "invalid alias - unknown letter board position {alias:?}");
        assert!(('1'..='8').contains(&index), "invalid alias - unknown number board position {alias:?}");
        let x = letter as u8 - b'A';
        let y = index as u8 - b'1';
        self.at(x, y)
    }

    pub fn calculate_shift(x: u8, y: u8) -> u8 { x / 2 + y * 4 }


    pub fn at(&self, x: u8, y: u8) -> Option<Figure> {
        if x % 2 != y % 2 {
            return None;
        }
        let shift = Self::calculate_shift(x, y);
        let occupation = (self.occupation >> shift) & 1;
        if occupation == 0 {
            return None;
        }
        let figure = (self.figure >> shift) & 1;
        let color = (self.color >> shift) & 1;
        match (figure, color) {
            (0, 0) => Some(Figure::Pawn(CheckersColor::White)),
            (0, 1) => Some(Figure::Pawn(CheckersColor::Black)),
            (1, 0) => Some(Figure::Queen(CheckersColor::White)),
            (1, 1) => Some(Figure::Queen(CheckersColor::Black)),
            _ => None
        }
    }


    fn in_range(value: u8) -> bool {
        (0..=7).contains(&value)
    }

    pub fn set(&mut self, x: u8, y: u8, figure: Option<Figure>) {
        assert!(Self::in_range(x), "x out of bounds. Got {}", x);
        assert!(Self::in_range(y), "x out of bounds. Got {}", y);
        if x % 2 == y % 2 {
            let (occupation, color, figure, _) = match figure {
                Some(figure) => figure.bits(),
                None => (0, 0, 0, 0)
            };
            set_bit!(self, occupation, x, y, occupation);
            set_bit!(self, color, x, y, color);
            set_bit!(self, figure, x, y, figure);
        }
    }

    pub fn set_flag(&mut self, x: u8, y: u8, flag: bool) {
        assert!(Self::in_range(x), "x out of bounds. Got {}", x);
        assert!(Self::in_range(y), "x out of bounds. Got {}", y);
        set_bit!(self, flags, x, y, if flag { 1 } else { 0 });
    }


    fn reset_flags(&mut self) {
        self.flags = 0;
    }

    pub fn num_figures(&self, color: CheckersColor) -> u32 {
        match color {
            CheckersColor::White => self.num_white_figures(),
            CheckersColor::Black => self.num_black_figures(),
        }
    }

    pub fn num_pawns(&self, color: CheckersColor) -> u32 {
        match color {
            CheckersColor::White => self.num_white_pawns(),
            CheckersColor::Black => self.num_black_pawns(),
        }
    }

    pub fn num_queens(&self, color: CheckersColor) -> u32 {
        match color {
            CheckersColor::White => self.num_white_queens(),
            CheckersColor::Black => self.num_black_queens(),
        }
    }

    pub fn num_white_figures(&self) -> u32 {
        (&self.occupation & !&self.color).count_ones()
    }

    pub fn num_white_pawns(&self) -> u32 {
        (&self.occupation & !&self.color & !&self.figure).count_ones()
    }

    pub fn num_white_queens(&self) -> u32 {
        (&self.occupation & !&self.color & &self.figure).count_ones()
    }

    pub fn num_black_figures(&self) -> u32 {
        (&self.occupation & &self.color).count_ones()
    }

    pub fn num_black_pawns(&self) -> u32 {
        (&self.occupation & &self.color & !&self.figure).count_ones()
    }

    pub fn num_black_queens(&self) -> u32 {
        (&self.occupation & &self.color & &self.figure).count_ones()
    }

}

impl Default for Board {
    fn default() -> Self {
        Self {
            occupation: 0b11111111111100000000111111111111,
            color: 0b11111111111100000000000000000000,
            figure: 0,
            flags: 0,
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn cell_repr(cell: Option<Figure>, x: u8, y: u8) -> String {
            let bg = if x % 2 != y % 2 {
                colors::bg::WHITE
            } else {
                colors::bg::BLACK
            };
            // ♣ ◄◎►
            match cell {
                None => colors::colored_text("   ", colors::NONE, &bg, true),
                Some(figure) => match figure {
                    Figure::Pawn(CheckersColor::White) => colors::colored_text(" ● ", &colors::fg::color(255, 255, 255), &bg, true),
                    Figure::Pawn(CheckersColor::Black) => colors::colored_text(" ● ", &colors::fg::VIOLET, &bg, true),
                    Figure::Queen(CheckersColor::White) => colors::colored_text("◄◎►", &colors::fg::color(255, 255, 255), &bg, true),
                    Figure::Queen(CheckersColor::Black) => colors::colored_text("◄◎►", &colors::fg::VIOLET, &bg, true),
                }
            }
        }
        let mut ret = String::from("   A  B  C  D  E  F  G  H \n");
        for y in (0..8).rev() {
            ret = format!("{ret}{} ", y + 1);
            for x in 0..8 {
                let figure = self.at(x, y);
                let r = cell_repr(figure, x, y);
                ret = format!("{ret}{r}");
            }
            ret = format!("{ret} {}\n", y + 1);
        }
        ret = format!("{ret}{}", String::from("   A  B  C  D  E  F  G  H \n"));
        write!(f, "{}", ret)
    }
}