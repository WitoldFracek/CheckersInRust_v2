use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure, Jump, Move};
use crate::game::{DummyBot, Game, HumanPlayer};

mod board;
mod controller;
mod colors;
mod game;

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

macro_rules! pos {
    ($from: ident, WP) => { (stringify!($from), Figure::Pawn(CheckersColor::White)) };
    ($from: ident, WQ) => { (stringify!($from), Figure::Queen(CheckersColor::White)) };
    ($from: ident, BP) => { (stringify!($from), Figure::Pawn(CheckersColor::Black)) };
    ($from: ident, BQ) => { (stringify!($from), Figure::Queen(CheckersColor::Black)) };
}


fn main() {
    let board = Board::from_str_repr(
        "........\n\
              ........\n\
              ...b....\n\
              ......b.\n\
              .b.....w\n\
              w.......\n\
              ........\n\
              ........",
        '.', ('w', 'W'), ('b', 'B')
    );
    let board = Board::from_alias_positions(&vec![
        pos!(B8, BP),
        pos!(D8, BP),
        pos!(F8, BP),
        pos!(H8, BP),
        pos!(A7, BP),
        pos!(C7, BP),
        pos!(E7, BP),
        pos!(G7, BP),
        pos!(B6, BP),
        pos!(H6, BP),
        pos!(G5, BP),

        pos!(A1, WP),
        pos!(C1, WP),
        pos!(E1, WP),
        pos!(G1, WP),
        pos!(B2, WP),
        pos!(D2, WP),
        pos!(F2, WP),
        pos!(H2, WP),
        pos!(G3, WP),
        pos!(C3, WP),
        pos!(C5, WP),
    ]);
    println!("{board}");
    let mut controller = CheckersController::new(board);
    let (cap, mov) = controller.options(CheckersColor::Black);
    for jc in cap {
        println!("{jc}");
    }


    let board = Board::default();
    let controller = CheckersController::new(board);
    let mut game = Game::new(
        controller,
        HumanPlayer::new(),
        DummyBot::new()
    );
    let winner = game.run();
    println!("Winner: {winner:?}");


}
