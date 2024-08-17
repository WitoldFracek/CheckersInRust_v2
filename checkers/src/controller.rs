use std::iter::once;
use crate::board::Board;

#[derive(Copy, Clone, Debug)]
pub enum CheckersColor{
    White, Black
}

#[derive(Copy, Clone, Debug)]
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
}

pub struct CheckersController {

}

impl CheckersController {
    pub fn get_white_pieces_position(board: &Board) -> Vec<(u8, u8)> {
        let mut ret = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(figure) = board.at(x, y) {
                    if figure.is_white() {
                        ret.push((x, y));
                    }
                }
            }
        }
        ret
    }

    pub fn get_black_pieces_position(board: &Board) -> Vec<(u8, u8)> {
        let mut ret = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(figure) = board.at(x, y) {
                    if !figure.is_white() {
                        ret.push((x, y));
                    }
                }
            }
        }
        ret
    }

    pub fn can_move(board: &Board, x: u8, y: u8) -> bool {
        match board.at(x, y) {
            None => false,
            Some(Figure::Queen(_)) => Self::can_queen_move(board, x as i8, y as i8),
            Some(Figure::Pawn(CheckersColor::White)) => Self::can_white_pawn_move(board, x as i8, y as i8),
            Some(Figure::Pawn(CheckersColor::Black)) => Self::can_black_pawn_move(board, x as i8, y as i8),
        }
    }

    fn can_white_pawn_move(board: &Board, x: i8, y: i8) -> bool {
        let (x1, y1) = (x - 1, y + 1);
        let (x2, y2) = (x + 1, y + 1);
        Self::is_square_free(board, x1, y1) || Self::is_square_free(board, x2, y2)
    }

    fn can_black_pawn_move(board: &Board, x: i8, y: i8) -> bool {
        let (x1, y1) = (x - 1, y - 1);
        let (x2, y2) = (x + 1, y - 1);
        Self::is_square_free(board, x1, y1) || Self::is_square_free(board, x2, y2)
    }

    fn can_queen_move(board: &Board, x: i8, y: i8) -> bool {
        // if the piece is a queen then the direction of move doesn't matter.
        // Both black and white queens have the same moves.
        Self::can_white_pawn_move(board, x, y) || Self::can_black_pawn_move(board, x, y)
    }

    pub fn is_square_free(board: &Board, x: i8, y: i8) -> bool {
        if y > 7 || x > 7 { return false; }
        if y < 0 || x < 0 { return false; }
        match board.at(x as u8, y as u8) {
            None => true,
            _ => false
        }
    }
}
