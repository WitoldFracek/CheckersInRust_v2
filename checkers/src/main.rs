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
        "W.......\n\
              .B.b.b..\n\
              ........\n\
              ...w....\n\
              ....b.b.\n\
              .b......\n\
              ....b...\n\
              ........",
        '.', ('w', 'W'), ('b', 'B')
    );
    let board = Board::default();
    let mut controller = CheckersController::new(board);

    println!("{}", controller.board);
    let (jumps, moves) = controller.options(CheckersColor::White);
    for jump in jumps {
        println!("{jump}");
    }
    println!();

    for move_ in moves {
        println!("{move_}");
    }

}
