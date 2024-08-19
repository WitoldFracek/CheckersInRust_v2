use crate::board::{Board};
use crate::controller::{CheckersColor, CheckersController, Figure};

mod board;
mod controller;
mod colors;

fn main() {
    let mut board = Board::default();
    println!("{}", board);
    let controller = CheckersController::new(board);
    let white_pieces = controller.get_white_pieces_position();

    let (d1, d2, d3, d4) = CheckersController::diagonals(2, 2);
    for d in d1 {
        println!("{:?}", d);
    }
}
