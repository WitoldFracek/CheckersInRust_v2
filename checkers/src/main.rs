use crate::board::{Board};
use crate::controller::{CheckersColor, CheckersController, Figure};

mod board;
mod controller;
mod colors;

fn main() {
    let mut board = Board::from_str_repr(
        "wewewewe\n\
              ewewewew\n\
              eeeeeeee\n\
              eeeeeeee\n\
              eeeeeeee\n\
              eweeeeee\n\
              Bebebebe\n\
              ebebebeb",
        'e', ('w', 'W'), ('b', 'B')
    );
    // let mut board = Board::default();
    // board.set(0, 0, Some(Figure::Pawn(CheckersColor::White)));
    println!("{}", board);
    board.set(0, 0, Some(Figure::Pawn(CheckersColor::White)));
    let controller = CheckersController::new(board);
    println!("{:?}", controller.queen_captures(0, 6));

    println!("{}", Board::alias(0, 7));
}
