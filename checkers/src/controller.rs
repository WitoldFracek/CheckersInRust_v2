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

impl CheckersColor {
    pub fn is_white(&self) -> bool {
        match self {
            CheckersColor::White => true,
            _ => false
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

pub trait CheckersAct: Into<CheckersAction> {
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

impl Into<CheckersAction> for Move {
    fn into(self) -> CheckersAction {
        CheckersAction::Move(self)
    }
}

impl CheckersAct for Move {
    fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }
}

impl Move {
    pub fn new(x_start: u8, y_start: u8, x_end: u8, y_end: u8) -> Self {
        Self {x_start, y_start, x_end, y_end}
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

    pub fn start_position(&self) -> (u8, u8) {
        (self.x_start, self.y_start)
    }

    pub fn end_position(&self) -> (u8, u8) {
        (self.x_end, self.y_end)
    }

    pub fn start_end(&self) -> ((u8, u8), (u8, u8)) {
        (self.start_position(), self.end_position())
    }
}

// impl CheckersAct for Jump {
//     fn start_position(&self) -> (u8, u8) {
//         (self.x_start, self.y_start)
//     }
//
//     fn end_position(&self) -> (u8, u8) {
//         (self.x_end, self.y_end)
//     }
// }

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

#[derive(Debug)]
pub struct JumpChain(Vec<Jump>);

impl JumpChain {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn start_position(&self) -> (u8, u8) {
        self.0.first().unwrap().start_position()
    }

    pub fn end_position(&self) -> (u8, u8) {
        self.0.last().unwrap().end_position()
    }
}

impl Display for JumpChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() { return write!(f, "[]") };
        let (x_start, y_start) = self.0.first().unwrap().start_position();
        let mut ret = format!("{} -> ", alias(x_start, y_start));
        let (mut x_end, mut y_end) = self.0.first().unwrap().end_position();
        for jump in self.0.iter().skip(1) {
            let (x_start, y_start) = jump.start_position();
            let al = alias(x_start, y_start);
            ret = format!("{ret}{al} -> ");
            (x_end, y_end) = jump.end_position();
        };
        ret = format!("{ret}{}", alias(x_end, y_end));
        write!(f, "{}", ret)
    }
}

// impl CheckersAct for JumpChain {
//     fn start_position(&self) -> (u8, u8) {
//         self.0[0].start_position()
//     }
//
//     fn end_position(&self) -> (u8, u8) {
//         self.0.last().unwrap().end_position()
//     }
// }

pub enum CheckersAction {
    Jump(Jump), Move(Move), JumpChain(JumpChain)
}

impl CheckersAction {
    pub fn start_position(&self) -> (u8, u8) {
        match self {
            CheckersAction::JumpChain(jump_chain) => jump_chain.start_position(),
            CheckersAction::Jump(jump) => jump.start_position(),
            CheckersAction::Move(move_) => move_.start_position()
        }
    }

    pub fn end_position(&self) -> (u8, u8) {
        match self {
            CheckersAction::JumpChain(jump_chain) => jump_chain.end_position(),
            CheckersAction::Jump(jump) => jump.end_position(),
            CheckersAction::Move(move_) => move_.end_position()
        }
    }
}

pub struct CheckersController {
    pub board: Board,
    white_queen_idle_moves: u8,
    black_queen_idle_moves: u8,
}

impl CheckersController {

    pub fn new(board: Board) -> Self {
        Self {
            board,
            white_queen_idle_moves: 0,
            black_queen_idle_moves: 0
        }
    }

    pub fn with_idle_moves(board: Board, wqim: u8, bqim: u8) -> Self {
        Self {
            board,
            white_queen_idle_moves: wqim,
            black_queen_idle_moves: bqim
        }
    }

    pub fn get_white_queen_idle_moves(&self) -> u8 { self.white_queen_idle_moves }

    pub fn get_black_queen_idle_moves(&self) -> u8 { self.black_queen_idle_moves }

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

    pub fn options(&self, color: CheckersColor) -> (Vec<JumpChain>, Vec<Move>) {
        let positions = match color {
            CheckersColor::White => self.get_white_pieces_position(),
            CheckersColor::Black => self.get_black_pieces_position()
        };
        let mut jumps = Vec::new();
        let mut moves = Vec::new();
        if positions.iter().any(|&(x, y)| self.can_capture(x, y)) {
            let captures = self.all_captures(color);
            let max_len = captures.iter().map(|jc| jc.len()).max().unwrap_or(0);
            jumps = self.all_captures(color)
                .into_iter()
                .filter(|jc| jc.len() == max_len)
                .collect();
        }
        if positions.iter().any(|&(x, y)| self.can_move(x, y)) {
            moves = self.all_moves(color)
                .into_iter()
                .collect();
        }
        (jumps, moves)
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

    pub fn moves_at(&self, x: u8, y: u8) -> Vec<Move> {
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
        let shift = Board::calculate_shift(x, y);
        ((self.board.flags >> shift) & 1) == 1
    }

    pub fn all_moves(&self, color: CheckersColor) -> Vec<Move> {
        let mut ret = Vec::new();
        let positions = match color {
            CheckersColor::White => self.get_white_pieces_position(),
            CheckersColor::Black => self.get_black_pieces_position(),
        };
        for (x, y) in positions {
           let mut figure_moves = self.moves_at(x, y);
            ret.append(&mut figure_moves);
        }
        ret
    }

    // captures

    pub fn all_captures(&self, color: CheckersColor) -> Vec<JumpChain> {
        let mut ret = Vec::new();
        let positions = match color {
            CheckersColor::White => self.get_white_pieces_position(),
            CheckersColor::Black => self.get_black_pieces_position(),
        };
        for (x, y) in positions {
            let mut chains = self.captures_at(x, y);
            ret.append(&mut chains);
        }
        ret
    }

    pub fn can_capture(&self, x: u8, y: u8) -> bool {
        match self.board.at(x, y) {
            None => false,
            Some(Figure::Pawn(_)) => !self.possible_pawn_jumps_at(x, y).is_empty(),
            Some(Figure::Queen(_)) => !self.possible_queen_jumps_at(x, y).is_empty()
        }
    }
    pub fn captures_at(&self, x: u8, y: u8) -> Vec<JumpChain> {
        let mut ret = Vec::new();
        self.capture_path(x, y, &mut Vec::new(), &mut ret);
        let max_len = ret.iter().map(|v| v.len()).max().unwrap_or(0);
        ret.into_iter()
            .filter(|v| v.len() == max_len)
            .map(|v| JumpChain(v))
            .collect()
    }

    fn capture_path(&self, x: u8, y: u8, path: &mut Vec<Jump>, all_paths: &mut Vec<Vec<Jump>>) {
        let possible_jumps = self.possible_captures_at(x, y);
        if possible_jumps.is_empty() {
            if !path.is_empty() {
                all_paths.push(path.clone());
            }
            return;
        }
        for jump in possible_jumps {
            path.push(jump);
            let mut controller = Self::new(self.board);
            controller.execute_jump(&jump);
            let (x_end, y_end) = jump.end_position();
            controller.capture_path(x_end, y_end, path, all_paths);
            path.pop();
        }
    }

    pub fn possible_captures_at(&self, x: u8, y: u8) -> Vec<Jump> {
        match self.board.at(x, y) {
            None => Vec::new(),
            Some(Figure::Pawn(_)) => self.possible_pawn_jumps_at(x, y),
            Some(Figure::Queen(_)) => self.possible_queen_jumps_at(x, y),
        }
    }

    pub fn tuple_pawn_captures_at(&self, coords: (u8, u8)) -> Vec<Jump> {
        self.possible_pawn_jumps_at(coords.0, coords.1)
    }

    pub fn possible_pawn_jumps_at(&self, x: u8, y: u8) -> Vec<Jump> {
        let pawn = self.board.at(x, y);
        if pawn.is_none() { return Vec::new(); }
        let pawn = pawn.unwrap();
        if pawn.is_queen() { return Vec::new(); }
        let (d1, d2, d3, d4) = Self::diagonals(x as i8, y as i8);
        let mut ret = Vec::new();
        for diagonal in [d1, d2, d3, d4] {
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
        self.possible_queen_jumps_at(coords.0, coords.1)
    }

    pub fn possible_queen_jumps_at(&self, x: u8, y: u8) -> Vec<Jump> {
        let queen = self.board.at(x, y);
        if queen.is_none() { return Vec::new(); }
        let queen = queen.unwrap();
        if !queen.is_queen() { return Vec::new(); }
        let (d1, d2, d3, d4) = Self::diagonals(x as i8, y as i8);
        let mut ret = Vec::new();
        for diagonal in [d1, d2, d3, d4] {
            let mut enemy_index = 8;
            for (i, &(x_over, y_over)) in diagonal.iter().enumerate() {
                if self.was_jumped_over(x_over, y_over) {
                    break;
                } else if !self.is_square_free(x_over, y_over) {
                    if self.is_enemy_on_square(x_over, y_over, queen.enemy_color()) {
                        enemy_index = i;
                    }
                    break;
                }
            }
            if enemy_index != 8 {
                let (x_over, y_over) = diagonal[enemy_index];
                for &(x_end, y_end) in diagonal.iter().skip(enemy_index + 1) {
                    if self.is_square_free(x_end, y_end) {
                        ret.push(Jump::new(x, y, x_over, y_over, x_end, y_end))
                    } else {
                        break;
                    }
                }
            }

        }
        ret
    }

    pub fn execute_capture(&mut self, jump_chain: &JumpChain) {
        for jump in &jump_chain.0 {
            self.execute_jump(jump);
        }
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
        match piece {
            Figure::Queen(CheckersColor::White) => self.white_queen_idle_moves += 1,
            Figure::Queen(CheckersColor::Black) => self.black_queen_idle_moves += 1,
            Figure::Pawn(CheckersColor::White) => self.white_queen_idle_moves = 0,
            Figure::Pawn(CheckersColor::Black) => self.black_queen_idle_moves = 0,
        }
        self.board.set(x_start, y_start, None);
        self.board.set(x_end, y_end, Some(piece));
    }

    pub fn execute_action(&mut self, action: &CheckersAction) {
        match action {
            CheckersAction::Move(move_) => self.execute_move(move_),
            CheckersAction::Jump(jump) => self.execute_jump(jump),
            CheckersAction::JumpChain(jump_chain) => self.execute_capture(jump_chain)
        }
    }

    fn in_bounds(x: i8, y: i8) -> bool {
        if y > 7 || x > 7 { return false; }
        if y < 0 || x < 0 { return false; }
        true
    }

    pub fn diagonals(x: i8, y: i8) -> (Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>) {
        // right up, left up, right down, left down
        let right_up = Self::diagonal(x, y, 1, 1);
        let left_up = Self::diagonal(x, y, -1, 1);
        let right_down = Self::diagonal(x, y, 1, -1);
        let left_down = Self::diagonal(x, y, -1, -1);
        (right_up, left_up, right_down, left_down)
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

    pub fn promote(&mut self) {
        for (x, y) in [(1, 7), (3, 7), (5, 7), (7, 7)] {
            let figure = self.board.at(x, y);
            if let Some(Figure::Pawn(CheckersColor::White)) = figure {
                self.board.set(x, y, Some(Figure::Queen(CheckersColor::White)))
            }
        }
        for (x, y) in [(0, 0), (2, 0), (4, 0), (6, 0)] {
            let figure = self.board.at(x, y);
            if let Some(Figure::Pawn(CheckersColor::Black)) = figure {
                self.board.set(x, y, Some(Figure::Queen(CheckersColor::Black)))
            }
        }
    }
}


