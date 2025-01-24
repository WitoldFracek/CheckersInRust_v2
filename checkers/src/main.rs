
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::board::{Board, alias, coords_from_alias};
use crate::controller::{CheckersColor, CheckersController, Figure, Jump, Move};
use crate::game::{Game};
use crate::game::estimators::{BoardEstimator, CountEstimator, WeightMatrixEstimator};
use crate::game::player::{AlphaBetaBot, DummyBot, HumanPlayer, MinMaxBot};

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

macro_rules! comp {
    [$elem:expr; for $var:ident in $iter:expr] => {{
        let mut res = Vec::new();
        for $var in $iter {
            res.push($elem)
        }
        res
    }};
    [$elem:expr; for $var:ident in $iter:expr; if $cond:expr] => {{
        let mut res = Vec::new();
        for $var in $iter {
            if $cond {
                res.push($elem)
            }
        }
        res
    }};
}


fn main() {
    let xs = comp![x * 2; for x in 0..10; if x % 2 == 0];
    println!("{xs:?}");
    // let board = Board::default();
    // let controller = CheckersController::new(board);
    //
    // let count_estimator = CountEstimator::new(1.0, 3.0);
    // let matrix_estimator = WeightMatrixEstimator::new(
    //     [
    //         [3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0],
    //         [3.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0],
    //         [3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0],
    //         [3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0],
    //         [3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0],
    //         [3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0],
    //         [3.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0],
    //         [3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0]
    //     ], 1.0, 3.0
    // );
    //
    // let human = HumanPlayer::new();
    // let dummy = DummyBot::new();
    // let minmax1 = MinMaxBot::new(count_estimator, 6);
    // let minmax2 = MinMaxBot::new(count_estimator, 8);
    // let alpha_beta1 = AlphaBetaBot::new(count_estimator, 10);
    // let mut game = Game::new(
    //     controller,
    //     alpha_beta1,
    //     minmax1
    // );
    // let winner = game.run();
    // println!("Winner: {winner:?}");

}
