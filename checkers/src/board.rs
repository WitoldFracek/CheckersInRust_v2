use std::fmt::format;
use crate::controller::{CheckersColor, Figure};
use crate::colors::colors as colors;


#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub occupation: u32,
    pub color: u32,
    pub figure: u32,
    pub flags: u32,
}

impl Board {

    pub fn standard() -> Self {
        Self {
            occupation: 0b11111111000000000000000011111111,
            color: 0b11111111000000000000000000000000,
            figure: 0,
            flags: 0,
        }
    }

    pub fn square(&self, alias: &str) -> Option<Figure> {
        assert_eq!(alias.len(), 2, "invalid alias - unknown board position {alias:?}");
        let letter = alias.chars().next()?.to_ascii_uppercase();
        let index = alias.chars().nth(1)?;
        assert!(('A'..='H').contains(&letter), "invalid alias - unknown letter board position {alias:?}");
        assert!(('1'..='8').contains(&index), "invalid alias - unknown number board position {alias:?}");
        let x = letter as u8 - b'A';
        let y = index as u8 - b'1';
        self.at(x, y)
    }


    pub fn at(&self, x: u8, y: u8) -> Option<Figure> {
        if x % 2 != y % 2 {
            return None;
        }
        let shift = x / 2 + y * 4;
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

    pub fn set(&mut self, x: u8, y: u8, figure: Option<Figure>) {
        let range = 0..=7;
        assert!(range.contains(&x), "x out of bounds. Got {}", x);
        assert!(range.contains(&y), "x out of bounds. Got {}", y);
        let shift = x / 2 + y * 4;
        let (occupation, color, figure, _) = match figure {
            None => (0, 0, 0, 0),
            Some(figure) => figure.bits()
        };
        if occupation == 0 {
            self.occupation &= !(1 << shift);
            return;
        } else {
            self.occupation |= 1 << shift;
        }
        if color == 0 {
            self.color &= !(1 << shift);
        } else {
            self.color |= 1 << shift;
        }
        if figure == 0 {
            self.figure &= !(1 << shift);
        } else {
            self.figure |= 1 << shift;
        }
    }

    fn reset_flags(&mut self) {
        self.flags &= 0;
    }

    pub fn num_pawns(&self, color: CheckersColor) -> u32 {
        match color {
            CheckersColor::White => self.num_white_figures(),
            CheckersColor::Black => self.num_black_figures(),
        }
    }

    pub fn num_white_figures(&self) -> u32 {
        (&self.occupation & !&self.color).count_ones()
    }

    pub fn num_black_figures(&self) -> u32 {
        (&self.occupation & &self.color).count_ones()
    }
}


pub struct BoardPrinter {

}

impl BoardPrinter {
     pub fn repr(board: &Board) -> String {
         let mut ret = String::from("   A  B  C  D  E  F  G  H \n");
         for y in (0..8).rev() {
             ret = format!("{ret}{} ", y + 1);
             for x in 0..8 {
                 let figure = board.at(x, y);
                 let r = Self::cell_repr(figure, x, y);
                 ret = format!("{ret}{r}");
             }
             ret = format!("{ret} {}\n", y + 1);
         }
         ret = format!("{ret}{}", String::from("   A  B  C  D  E  F  G  H \n"));
         ret
    }

    fn cell_repr(cell: Option<Figure>, x: u8, y: u8) -> String {
        let bg = if x % 2 != y % 2 {
            colors::bg::WHITE
        } else {
            colors::bg::BLACK
        };

        match cell {
            None => colors::colored_text("   ", colors::NONE, &bg, true),
            Some(figure) => match figure {
                Figure::Pawn(CheckersColor::White) => colors::colored_text(" ● ", &colors::fg::color(255, 255, 255), &bg, true),
                Figure::Pawn(CheckersColor::Black) => colors::colored_text(" ● ", &colors::fg::color(118, 118, 118), &bg, true),
                Figure::Queen(CheckersColor::White) => colors::colored_text(" ♣ ", &colors::fg::color(255, 255, 255), &bg, true),
                Figure::Queen(CheckersColor::Black) => colors::colored_text(" ♣ ", &colors::fg::color(118, 118, 118), &bg, true),
            }
        }
    }
}