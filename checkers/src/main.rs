use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure, Jump, Move};

mod board;
mod controller;
mod colors;


fn main() {
    let board = Board::from_str_repr(
        "w.w.w.w.\n\
              .w.w.w.w\n\
              ........\n\
              ........\n\
              ....w.w.\n\
              .....B..\n\
              B.b.b.b.\n\
              .b.b.b.b",
        '.', ('w', 'W'), ('b', 'B')
    );
    let mut controller = CheckersController::new(board);

    println!("{}", controller.board);

    controller.execute_jump(&Jump::new(5, 5, 4, 4, 3, 3));
    println!("{}", controller.board);

    let mut controller = CheckersController::new(board);
    controller.execute_move(&Move::new(6, 4, 5, 5));
    println!("{}", controller.board);

}
