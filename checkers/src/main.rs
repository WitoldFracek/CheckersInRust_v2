use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure, Jump, Move};

mod board;
mod controller;
mod colors;

macro_rules! mov {
    ($from: ident -> $to: ident) => {{
        let (x_start, y_start) = coords_from_alias(stringify!($from));
        let (x_end, y_end) = coords_from_alias(stringify!($to));
        Move::new(x_start, y_start, x_end, y_end)
    }};
}

macro_rules! jump {
    ($from: ident -- $over: ident -> $to: ident) => {{
        let (x_start, y_start) = coords_from_alias(stringify!($from));
        let (x_end, y_end) = coords_from_alias(stringify!($to));
        let (x_over, y_over) = coords_from_alias(stringify!($over));
        Jump::new(x_start, y_start, x_over, y_over, x_end, y_end)
    }};
}


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

    controller.execute_jump(&jump!(F6 -- E5 -> C3));
    println!("{}", controller.board);

    let mut controller = CheckersController::new(board);
    controller.execute_move(&mov!(G5 -> F6));
    println!("{}", controller.board);

}
