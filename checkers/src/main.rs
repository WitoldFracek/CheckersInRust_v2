use crate::board::{Board, BoardPrinter};
use crate::controller::{CheckersColor, CheckersController, Figure};

mod board;
mod controller;
mod colors;

fn main() {
    let mut board = Board::standard();
    println!("{}", BoardPrinter::repr(&board));
    let white_pieces = CheckersController::get_white_pieces_position(&board);
    for (x, y) in white_pieces {
        println!("({x}, {y}): {}", CheckersController::can_move(&board, x, y));
    }

    let black_pieces = CheckersController::get_black_pieces_position(&board);
    for (x, y) in black_pieces {
        println!("({x}, {y}): {}", CheckersController::can_move(&board, x, y));
    }

}
