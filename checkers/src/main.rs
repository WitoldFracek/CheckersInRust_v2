use crate::board::{Board, BoardPrinter};
use crate::controller::{CheckersColor, CheckersController, Figure};

mod board;
mod controller;
mod colors;

fn main() {
    let mut board = Board::standard();
    println!("{}", BoardPrinter::repr(&board));
    let controller = CheckersController::new(board);
    let white_pieces = controller.get_white_pieces_position();

    let (d1, d2, d3, d4) = CheckersController::diagonals(2, 2);
    for d in diagonals {
        println!("{:?}", d);
    }
}
