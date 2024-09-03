use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure, Jump, Move};
use crate::game::{Game};
use crate::game::ai::{BoardEstimator, CountEstimator};
use crate::game::player::{DummyBot, HumanPlayer, MinMaxBot};

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


fn main() { let board = Board::default();
    let controller = CheckersController::new(board);
    let human = HumanPlayer::new();
    let dummy = DummyBot::new();
    let minmax1 = MinMaxBot::new(CountEstimator::new(1.0, 3.0), 8);
    let minmax2 = MinMaxBot::new(CountEstimator::new(1.0, 3.0), 3);
    let mut game = Game::new(
        controller,
        minmax1,
        minmax2
    );
    let winner = game.run();
    println!("Winner: {winner:?}");
}
