use crate::controller::{CheckersController, CheckersColor};
use rand::seq::SliceRandom;
use crate::game::player::Player;

pub mod player {
    use std::io;
    use rand::seq::SliceRandom;
    use crate::board::Board;
    use crate::controller::{CheckersColor, CheckersController, JumpChain, Move};
    use crate::game::ai::BoardEstimator;

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

    pub struct MinMaxBot<T> {
        estimator: T
    }

    impl <T> MinMaxBot<T> {
        pub fn new(estimator: T) -> Self { Self{estimator}}
    }

    // impl <T: BoardEstimator> MinMaxBot<T> {
    //     fn minmax(&self, controller: &CheckersController, depth: usize, current_color: CheckersColor, maximising: bool) -> f64 {
    //         if depth == 0 { return self.estimator.score(&controller.board); }
    //         let (jumps, moves) = controller.options(current_color);
    //         if !jumps.is_empty() {
    //
    //         }
    //     }
    //
    //     fn minmax_jumps(&self, )
    // }

    impl <T: BoardEstimator> Player for MinMaxBot<T> {
        fn get_move<'a>(&'a self, moves: &'a [Move]) -> &Move {
            todo!()
        }

        fn get_capture<'a>(&'a self, captures: &'a [JumpChain]) -> &JumpChain {
            todo!()
        }
    }
}

pub mod ai {
    use crate::board::Board;
    use crate::controller::CheckersColor;

    pub trait BoardEstimator {
        fn score(&self, board: &Board) -> f64;
    }

    pub struct CountEstimator {
        maximising_color: CheckersColor,
        pawn_weight: f64,
        queen_weight: f64
    }

    impl CountEstimator {
        pub fn new(maximising_color: CheckersColor, pawn_weight: f64, queen_weight: f64) -> Self {
            Self{maximising_color, pawn_weight, queen_weight} }
    }

    impl BoardEstimator for CountEstimator {
        fn score(&self, board: &Board) -> f64 {
            let (pawns, queens) = match self.maximising_color {
                CheckersColor::White => (board.num_white_paws(), board.num_white_queens()),
                CheckersColor::Black => (board.num_black_pawns(), board.num_black_queens())
            };
            pawns as f64 * self.pawn_weight + queens as f64 * self.queen_weight
        }
    }
}


pub struct Game<WP, BP> {
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