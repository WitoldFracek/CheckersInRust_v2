use crate::board::{Board, BoardPrinter};
use crate::controller::{CheckersColor, Figure};

mod board;
mod controller;
mod colors;

fn main() {
    let mut board = Board::standard();

    let piece = board.at(1, 1);
    println!("{:?}", piece);

    println!("{}", BoardPrinter::repr(&board));
    println!("{:?}", board.square("A1"));
    println!("{:?}", board.square("A2"));
    println!("{:?}", board.square("B1"));
    println!("{:?}", board.square("A7"));
    println!("{:?}", board.square("F8"));
    println!("{:?}", board.square("H7"));

    // board.set(0, 0, None);
    // println!("{}", BoardPrinter::repr(&board));
}
