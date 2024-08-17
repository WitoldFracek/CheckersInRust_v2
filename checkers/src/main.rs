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
    for (x, y) in white_pieces {
        println!("({x}, {y}): {}", controller.can_move(x, y));
    }

    let black_pieces = controller.get_black_pieces_position();
    for (x, y) in black_pieces {
        println!("({x}, {y}): {}", controller.can_move(x, y));
    }

}
