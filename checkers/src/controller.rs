use std::fmt::{write, Display, Formatter};
use crate::board::{Board, alias};

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

#[derive(Copy, Clone, Debug)]
pub struct Move {
    x_start: u8,
    y_start: u8,
    x_end: u8,
    y_end: u8
}

impl Move {
    pub fn new(x_start: u8, y_start: u8, x_end: u8, y_end: u8) -> Self {
        Self {x_start, y_start, x_end, y_end}
    }
}

impl CheckersAction for Move {
    fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = format!("{} -> {}", alias(self.x_start, self.y_start), alias(self.x_end, self.y_end));
        write!(f, "{}", repr)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Jump {
    x_start: u8,
    y_start: u8,
    x_over: u8,
    y_over: u8,
    x_end: u8,
    y_end: u8
}

impl Jump {
    pub fn new(x_start: u8, y_start: u8, x_over: u8, y_over: u8, x_end: u8, y_end: u8) -> Self {
        Self {x_start, y_start, x_over, y_over, x_end, y_end}
    }

    pub fn over_position(&self) -> (u8, u8) {
        (self.x_over, self.y_over)
    }
}

impl CheckersAction for Jump {
    fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = format!(
            "{} --|{}|-> {}",
            alias(self.x_start, self.y_start),
            alias(self.x_over, self.y_over),
            alias(self.x_end, self.y_end)
        );
        write!(f, "{}", repr)
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

    // moves

    pub fn can_move(&self, x: u8, y: u8) -> bool {
        match self.board.at(x, y) {
            None => false,
            Some(Figure::Queen(_)) => self.can_queen_move(x, y),
            Some(Figure::Pawn(CheckersColor::White)) => self.can_white_pawn_move(x, y),
            Some(Figure::Pawn(CheckersColor::Black)) => self.can_black_pawn_move(x, y),
        }
    }

    pub fn get_moves_at(&self, x: u8, y: u8) -> Vec<Move> {
        let figure = self.board.at(x, y);
        if figure.is_none() { return Vec::new(); }
        match figure.unwrap() {
            Figure::Queen(_) => self.queen_moves_at(x, y),
            Figure::Pawn(CheckersColor::White) => self.white_pawn_moves_at(x, y),
            Figure::Pawn(CheckersColor::Black) => self.black_pawn_moves_at(x, y),
        }
    }

    fn white_pawn_moves_at(&self, x: u8, y: u8) -> Vec<Move> {
        let mut ret = Vec::with_capacity(2);
        let (x0, y0) = (x as i8 - 1, y as i8 + 1);
        let (x1, y1) = (x as i8 + 1, y as i8 + 1);
        if Self::in_bounds(x0, y0) && self.is_square_free(x0 as u8, y0 as u8) {
            ret.push(Move::new(
                x, y, x0 as u8, y0 as u8
            ));
        }
        if Self::in_bounds(x1, y1) && self.is_square_free(x1 as u8, y1 as u8) {
            ret.push(Move::new(
                x, y, x1 as u8, y1 as u8
            ));
        }
        ret
    }

    fn black_pawn_moves_at(&self, x: u8, y: u8) -> Vec<Move> {
        let mut ret = Vec::with_capacity(2);
        let (x0, y0) = (x as i8 - 1, y as i8 - 1);
        let (x1, y1) = (x as i8 + 1, y as i8 - 1);
        if Self::in_bounds(x0, y0) && self.is_square_free(x0 as u8, y0 as u8) {
            ret.push(Move::new(
                x, y, x0 as u8, y0 as u8
            ));
        }
        if Self::in_bounds(x1, y1) && self.is_square_free(x1 as u8, y1 as u8) {
            ret.push(Move::new(
                x, y, x1 as u8, y1 as u8
            ));
        }
        ret
    }

    fn queen_moves_at(&self, x: u8, y: u8) -> Vec<Move> {
        let mut ret = Vec::new();
        let (d1, d2, d3, d4) = Self::diagonals(x as i8, y as i8);
        for diagonal in [d1, d2, d3, d4] {
            for (x_end, y_end) in diagonal {
                if Self::in_bounds(x_end as i8, y_end as i8) && self.is_square_free(x_end, y_end) {
                    ret.push(Move::new(x, y, x_end, y_end));
                } else if !self.is_square_free(x_end, y_end) {
                    break
                }
            }
        }
        ret
    }

    fn can_white_pawn_move(&self, x: u8, y: u8) -> bool {
        let (x1, y1) = (x as i8 - 1, y + 1);
        let (x2, y2) = (x + 1, y + 1);
        (Self::in_bounds(x1, y1 as i8) && self.is_square_free(x1 as u8, y1))
            || (Self::in_bounds(x2 as i8, y2 as i8) && self.is_square_free(x2, y2))
    }

    fn can_black_pawn_move(&self, x: u8, y: u8) -> bool {
        let (x1, y1) = (x as i8 - 1, y as i8 - 1);
        let (x2, y2) = (x + 1, y as i8 - 1);
        (Self::in_bounds(x1, y1) && self.is_square_free(x1 as u8, y1 as u8))
            || (Self::in_bounds(x2 as i8, y2) && self.is_square_free(x2, y2 as u8))
    }

    fn can_queen_move(&self, x: u8, y: u8) -> bool {
        // if the piece is a queen then the direction of move doesn't matter.
        // Both black and white queens have the same moves.
        self.can_white_pawn_move(x, y) || self.can_black_pawn_move(x, y)
    }

    pub fn is_square_free(&self, x: u8, y: u8) -> bool {
        if !Self::in_bounds(x as i8, y as i8) { return false; }
        self.board.at(x, y).is_none()
    }

    pub fn is_enemy_on_square(&self, x: u8, y: u8, enemy_color: CheckersColor) -> bool {
        let figure = self.board.at(x, y);
        if figure.is_none() { return false; }
        figure.unwrap().color() == enemy_color
    }

    pub fn was_jumped_over(&self, x: u8, y: u8) -> bool {
        let figure = self.board.at(x, y);
        if figure.is_none() { return false; }
        let shift = Board::calculate_shift(x, y);
        ((self.board.flags >> shift) & 1) == 1
    }

    pub fn get_all_moves(&self, color: CheckersColor) -> Vec<Move> {
        let mut ret = Vec::new();
        let positions = match color {
            CheckersColor::White => self.get_white_pieces_position(),
            CheckersColor::Black => self.get_black_pieces_position(),
        };
        for (x, y) in positions {
           let mut figure_moves = self.get_moves_at(x, y);
            ret.append(&mut figure_moves);
        }
        ret
    }

    // captures

    pub fn get_all_captures(&self, color: CheckersColor) -> Vec<Vec<Jump>> {
        let mut ret = Vec::new();
        let positions = match color {
            CheckersColor::White => self.get_white_pieces_position(),
            CheckersColor::Black => self.get_black_pieces_position(),
        };
        for (x, y) in positions {
            let mut captures = self.captures_at(x, y);
            ret.append(&mut captures);
        }
        ret
    }

    pub fn captures_at(&self, x: u8, y: u8) -> Vec<Jump> {
        let mut ret = Vec::new();
        let mut controller = Self::new(self.board);
        let captures = match controller.board.at(x, y) {
            None => vec![],
            Some(Figure::Pawn(_)) => controller.pawn_captures_beginnings_at(x, y),
            Some(Figure::Queen(_)) => controller.queen_captures_at(x, y),
        };
    }

    pub fn tuple_pawn_captures_at(&self, coords: (u8, u8)) -> Vec<Jump> {
        self.pawn_captures_beginnings_at(coords.0, coords.1)
    }

    pub fn pawn_captures_beginnings_at(&self, x: u8, y: u8) -> Vec<Jump> {
        let pawn = self.board.at(x, y);
        if pawn.is_none() { return Vec::new(); }
        let pawn = pawn.unwrap();
        if pawn.is_queen() { return Vec::new(); }
        let (d1, d2) = match pawn.color() {
            CheckersColor::White => (Self::diagonal(x as i8, y as i8, 1, 1), Self::diagonal(x as i8, y as i8, -1, 1)),
            CheckersColor::Black => (Self::diagonal(x as i8, y as i8, 1, -1), Self::diagonal(x as i8, y as i8, -1, -1)),
        };
        let mut ret = Vec::new();
        for diagonal in [d1, d2] {
            if diagonal.len() >= 2 {
                let over = diagonal[0];
                let end = diagonal[1];
                if self.is_square_free(end.0, end.1)
                    && self.is_enemy_on_square(over.0, over.1, pawn.enemy_color())
                    && !self.was_jumped_over(over.0, over.1) {
                    ret.push(Jump::new(x, y, over.0, over.1, end.0, end.1))
                }
            }
        }
        ret
    }

    pub fn tuple_queen_captures_at(&self, coords: (u8, u8)) -> Vec<Jump> {
        self.queen_captures_at(coords.0, coords.1)
    }

    pub fn queen_captures_at(&self, x: u8, y: u8) -> Vec<Jump> {
        let queen = self.board.at(x, y);
        if queen.is_none() { return Vec::new(); }
        let queen = queen.unwrap();
        if !queen.is_queen() { return Vec::new(); }
        let (d1, d2, d3, d4) = Self::diagonals(x as i8, y as i8);
        let mut ret = Vec::new();
        for diagonal in [d1, d2, d3, d4] {
            let mut enemy_index = 8;
            for (i, &(x_over, y_over)) in diagonal.iter().enumerate() {
                if !self.is_square_free(x_over, y_over)
                    && self.is_enemy_on_square(x_over, y_over, queen.enemy_color())
                    && !self.was_jumped_over(x_over, y_over) {
                    enemy_index = i;
                    break;
                }
            }
            if enemy_index != 8 {
                let (x_over, y_over) = diagonal[enemy_index];
                for &(x_end, y_end) in diagonal.iter().skip(enemy_index) {
                    if self.is_square_free(x_end, y_end) {
                        ret.push(Jump::new(x, y, x_over, y_over, x_end, y_end))
                    }
                }
            }

        }
        ret
    }

    pub fn execute_jump(&mut self, jump: &Jump) {
        let (x_over, y_over) = jump.over_position();
        let ((x_start, y_start), (x_end, y_end)) = jump.start_end();
        let piece = self.board.at(x_start, y_start).unwrap();
        self.board.set(x_start, y_start, None);
        self.board.set(x_over, y_over, None);
        self.board.set_flag(x_over, y_over, true);
        self.board.set(x_end, y_end, Some(piece));
    }

    pub fn execute_move(&mut self, move_: &Move) {
        let ((x_start, y_start), (x_end, y_end)) = move_.start_end();
        let piece = self.board.at(x_start, y_start).unwrap();
        self.board.set(x_start, y_start, None);
        self.board.set(x_end, y_end, Some(piece));
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
