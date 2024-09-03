use crate::controller::{CheckersController, Move, JumpChain, CheckersColor};
use rand::seq::SliceRandom;
use std::{io, thread};
use std::time::Duration;

pub trait Player {
    fn get_move<'a>(&'a self, moves: &'a [Move]) -> &Move;
    fn get_capture<'a>(&'a self, captures: &'a [JumpChain]) -> &JumpChain;
}

pub struct DummyBot;

impl DummyBot {
    pub fn new() -> Self { Self {} }

}

impl Player for DummyBot {
    fn get_move<'a>(&'a self, moves: &'a [Move]) -> &Move {
        let m = moves.choose(&mut rand::thread_rng()).unwrap();
        println!("{m}");
        m
    }

    fn get_capture<'a>(&'a self, captures: &'a [JumpChain]) -> &JumpChain {
        let c = captures.choose(&mut rand::thread_rng()).unwrap();
        println!("{c}");
        c
    }
}

pub struct HumanPlayer;

impl HumanPlayer {
    pub fn new() -> Self { Self{} }
}

impl Player for HumanPlayer {
    fn get_move<'a>(&'a self, moves: &'a [Move]) -> &Move {
        for (i, move_) in moves.iter().enumerate() {
            println!("{i}. {move_}");
        }
        let index = self.get_index(moves.len());
        &moves[index]
    }

    fn get_capture<'a>(&'a self, captures: &'a [JumpChain]) -> &JumpChain {
        for (i, jump_chain) in captures.iter().enumerate() {
            println!("{i}. {jump_chain}");
        }
        let index = self.get_index(captures.len());
        &captures[index]
    }
}

impl HumanPlayer {
    fn get_index(&self, len: usize) -> usize {
        loop {
            let mut input = String::new();
            let res = io::stdin().read_line(&mut input);
            if res.is_err() {
                println!("Failed to read line");
                continue;
            }
            let input = input.trim();
            let index = input.parse::<usize>();
            if index.is_err() {
                println!("unrecognised option \"{input}\"");
                continue;
            }
            let index = index.unwrap();
            if index >= len {
                println!("no move with index {index}");
                continue;
            }
            break index
        }
    }
}

pub struct Game<WP: Player, BP: Player> {
    controller: CheckersController,
    white_player : WP,
    black_player: BP,
    current_player: CheckersColor
}


impl <WP: Player, BP: Player> Game<WP, BP> {
    pub fn new(controller: CheckersController, white_player: WP, black_player: BP) -> Self {
        Self {
            controller, white_player, black_player, current_player: CheckersColor::White
        }
    }

    pub fn run(&mut self) -> CheckersColor {
        println!("{}", self.controller.board);
        while let Ok(_) = self.step() {
            self.controller.promote();
            println!("{}", self.controller.board);
            // thread::sleep(Duration::from_millis(1000));
            if self.controller.board.num_white_figures() == 0 {
                return CheckersColor::Black;
            }
            if self.controller.board.num_black_figures() == 0 {
                return CheckersColor::White;
            }
        }
        panic!()
    }

    pub fn step(&mut self) -> Result<(), ()> {
        let (captures, moves) = self.controller.options(self.current_player);
        if !captures.is_empty() {
            let capture = match self.current_player {
                CheckersColor::White => self.white_player.get_capture(&captures),
                CheckersColor::Black => self.black_player.get_capture(&captures),
            };
            self.controller.execute_capture(capture);
            self.controller.board.flags = 0;
            self.current_player = self.current_player.opposite();
            return Ok(());
        }
        if !moves.is_empty() {
            let move_ = match self.current_player {
                CheckersColor::White => self.white_player.get_move(&moves),
                CheckersColor::Black => self.black_player.get_move(&moves),
            };
            self.controller.execute_move(move_);
            self.current_player = self.current_player.opposite();
            return Ok(());
        }
        Err(())
    }
}