use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure};

mod board;
mod controller;
mod colors;


fn main() {
    let mut board = Board::from_str_repr(
        "w.w.w.w.\n\
              .w.w.w.w\n\
              ........\n\
              ........\n\
              ..w.w.w.\n\
              .....B..\n\
              B.b.b.b.\n\
              .b.b.b.b",
        '.', ('w', 'W'), ('b', 'B')
    );
    // let mut board = Board::default();
    // board.set(0, 0, Some(Figure::Pawn(CheckersColor::White)));
    println!("{}", board);
    board.set(0, 0, Some(Figure::Pawn(CheckersColor::White)));
    let controller = CheckersController::new(board);
    for capture in controller.tuple_queen_captures(coords_from_alias("F6")) {
        println!("{capture}");
    }

    let (x, y) = coords_from_alias("C7");
    for capture in controller.pawn_captures(x, y) {
        println!("{capture}");
    }
}
