use crate::board::Board;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CheckersColor{
    White, Black
}

impl CheckersColor {
    pub fn opposite(&self) -> Self {
        match self {
            CheckersColor::White => CheckersColor::Black,
            CheckersColor::Black => CheckersColor::White
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Figure {
    Pawn(CheckersColor), Queen(CheckersColor)
}

impl Figure {
    pub fn bits(&self) -> (u32, u32, u32, u32) {
        // (occupation, color, figure, flag)
        match self {
            Figure::Pawn(CheckersColor::White) => (1, 0, 0, 0),
            Figure::Pawn(CheckersColor::Black) => (1, 1, 0, 0),
            Figure::Queen(CheckersColor::White) => (1, 0, 1, 0),
            Figure::Queen(CheckersColor::Black) => (1, 1, 1, 0)
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            Figure::Pawn(CheckersColor::White) | Figure::Queen(CheckersColor::White) => true,
            _ => false,
        }
    }

    pub fn is_queen(&self) -> bool {
        match self {
            Figure::Queen(CheckersColor::Black) | Figure::Queen(CheckersColor::White) => true,
            _ => false,
        }
    }

    pub fn color(&self) -> CheckersColor {
        match self {
            Figure::Pawn(color) | Figure::Queen(color) => *color
        }
    }

    pub fn enemy_color(&self) -> CheckersColor {
        self.color().opposite()
    }
}

pub trait CheckersAction {
    fn start_position(&self) -> (u8, u8);

    fn end_position(&self) -> (u8, u8);
    
    fn start_end(&self) -> ((u8, u8), (u8, u8)) {
        (self.start_position(), self.end_position())
    }
}

pub struct Move {
    x_start: u8,
    y_start: u8,
    x_end: u8,
    y_end: u8
}

impl CheckersAction for Move {
    fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }
}

pub struct Jump {
    x_start: u8,
    y_start: u8,
    x_over: u8,
    y_over: u8,
    x_end: u8,
    y_end: u8
}

impl CheckersAction for Jump {
    fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }
}

pub struct CheckersController {
    pub board: Board
}

impl CheckersController {

    pub fn new(board: Board) -> Self {
        Self {
            board
        }
    }
    pub fn get_white_pieces_position(&self) -> Vec<(u8, u8)> {
        let mut ret = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(figure) = self.board.at(x, y) {
                    if figure.is_white() {
                        ret.push((x, y));
                    }
                }
            }
        }
        ret
    }

    pub fn get_black_pieces_position(&self) -> Vec<(u8, u8)> {
        let mut ret = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(figure) = self.board.at(x, y) {
                    if !figure.is_white() {
                        ret.push((x, y));
                    }
                }
            }
        }
        ret
    }

    pub fn can_move(&self, x: u8, y: u8) -> bool {
        match self.board.at(x, y) {
            None => false,
            Some(Figure::Queen(_)) => self.can_queen_move(x as i8, y as i8),
            Some(Figure::Pawn(CheckersColor::White)) => self.can_white_pawn_move(x as i8, y as i8),
            Some(Figure::Pawn(CheckersColor::Black)) => self.can_black_pawn_move(x as i8, y as i8),
        }
    }

    fn can_white_pawn_move(&self, x: i8, y: i8) -> bool {
        let (x1, y1) = (x - 1, y + 1);
        let (x2, y2) = (x + 1, y + 1);
        self.is_square_free(x1, y1) || self.is_square_free(x2, y2)
    }

    fn can_black_pawn_move(&self, x: i8, y: i8) -> bool {
        let (x1, y1) = (x - 1, y - 1);
        let (x2, y2) = (x + 1, y - 1);
        self.is_square_free(x1, y1) || self.is_square_free(x2, y2)
    }

    fn can_queen_move(&self, x: i8, y: i8) -> bool {
        // if the piece is a queen then the direction of move doesn't matter.
        // Both black and white queens have the same moves.
        self.can_white_pawn_move(x, y) || self.can_black_pawn_move(x, y)
    }

    pub fn is_square_free(&self, x: i8, y: i8) -> bool {
        if !Self::in_bounds(x, y) { return false; }
        match self.board.at(x as u8, y as u8) {
            None => true,
            _ => false
        }
    }

    pub fn is_enemy_on_square(&self, x: i8, y: i8, enemy_color: CheckersColor) -> bool {
        let figure = self.board.at(x as u8, y as u8);
        if figure.is_none() { return false; }
        figure.unwrap().color() == enemy_color
    }

    pub fn can_pawn_capture(&self, x: u8, y: u8) -> bool {
        todo!()
    }

    fn in_bounds(x: i8, y: i8) -> bool {
        if y > 7 || x > 7 { return false; }
        if y < 0 || x < 0 { return false; }
        true
    }

    pub fn diagonals(x: i8, y: i8) -> (Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>) {
        // right up, left up, right down, left down
        let d1 = Self::diagonal(x, y, 1, 1);
        let d2 = Self::diagonal(x, y, -1, 1);
        let d3 = Self::diagonal(x, y, 1, -1);
        let d4 = Self::diagonal(x, y, -1, -1);
        (d1, d2, d3, d4)
    }

    fn diagonal(x: i8, y: i8, x_step: i8, y_step: i8) -> Vec<(u8, u8)> {
        let mut ret = vec![];
        let mut x = x + x_step;
        let mut y = y + y_step;
        while Self::in_bounds(x, y) {
            ret.push((x as u8, y as u8));
            x += x_step;
            y += y_step;
        }
        ret
    }
}
