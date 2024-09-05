use std::thread;
use std::time::Duration;
use crate::controller::{CheckersController, CheckersColor};
use rand::seq::SliceRandom;
use crate::game::player::Player;

pub mod player {
    use std::cmp::max;
    use std::io;
    use rand::seq::SliceRandom;
    use crate::board::Board;
    use crate::controller::{CheckersColor, CheckersController, JumpChain, Move};
    use crate::game::ai::BoardEstimator;

    pub trait Player {
        fn move_piece<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move;
        fn capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain;
        fn get_color(&self) -> CheckersColor;
        fn set_color(&mut self, color: CheckersColor);
    }

    pub struct DummyBot {
        color: CheckersColor
    }

    impl DummyBot {
        pub fn new() -> Self { Self { color: CheckersColor::White } }

    }

    impl Player for DummyBot {
        fn move_piece<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            let m = moves.choose(&mut rand::thread_rng()).unwrap();
            println!("{m}");
            m
        }

        fn capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
            let c = captures.choose(&mut rand::thread_rng()).unwrap();
            println!("{c}");
            c
        }

        fn set_color(&mut self, color: CheckersColor) {
            self.color = color;
        }

        fn get_color(&self) -> CheckersColor {
            self.color
        }


    }

    pub struct HumanPlayer {
        color: CheckersColor
    }

    impl HumanPlayer {
        pub fn new() -> Self { Self{ color: CheckersColor::White } }
    }

    impl Player for HumanPlayer {
        fn move_piece<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            for (i, move_) in moves.iter().enumerate() {
                println!("{i}. {move_}");
            }
            let index = self.get_index(moves.len());
            &moves[index]
        }

        fn capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
            for (i, jump_chain) in captures.iter().enumerate() {
                println!("{i}. {jump_chain}");
            }
            let index = self.get_index(captures.len());
            &captures[index]
        }

        fn get_color(&self) -> CheckersColor {
            self.color
        }

        fn set_color(&mut self, color: CheckersColor) {
            self.color = color
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
        estimator: T,
        depth: usize,
        color: CheckersColor
    }

    impl <T> MinMaxBot<T> {
        pub fn new(estimator: T, depth: usize) -> Self { Self{estimator, depth, color: CheckersColor::White} }
    }

    impl <T: BoardEstimator> MinMaxBot<T> {
        fn minmax(&self, controller: &CheckersController, depth: usize, current_color: CheckersColor, maximising: bool) -> f64 {
            if depth == 0 { return self.estimator.score(&controller.board, self.color); }// * if maximising {1.0} else {-1.0}; }
            let (jumps, moves) = controller.options(current_color);
            if !jumps.is_empty() {
                return self.minmax_jumps(controller.board, &jumps, depth, current_color, maximising);
            }
            if !moves.is_empty() {
                return self.minmax_moves(controller.board, &moves, depth, current_color, maximising);
            }
            if maximising {
                f64::MIN
            } else {
                f64::MAX
            }
        }

        fn minmax_jumps(&self, board: Board, captures: &[JumpChain], depth: usize, current_color: CheckersColor, maximizing: bool) -> f64 {
            let mut current = if maximizing {f64::MIN} else {f64::MAX};
            for capture in captures {
                let mut controller = CheckersController::new(board);
                controller.execute_capture(capture);
                let est = self.minmax(&controller, depth - 1, current_color.opposite(), maximizing);
                if maximizing && est > current {
                    current = est;
                } else if !maximizing && est < current {
                    current = est;
                }
            }
            current
        }

        fn minmax_moves(&self, board: Board, moves: &[Move], depth: usize, current_color: CheckersColor, maximizing: bool) -> f64 {
            let mut current = if maximizing {f64::MIN} else {f64::MAX};
            for move_ in moves {
                let mut controller = CheckersController::new(board);
                controller.execute_move(move_);
                let est = self.minmax(&controller, depth - 1, current_color.opposite(), maximizing);
                if maximizing && est > current {
                    current = est;
                } else if !maximizing && est < current {
                    current = est;
                }
            }
            current
        }
    }

    impl <T: BoardEstimator> Player for MinMaxBot<T> {
        fn move_piece<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            if moves.len() == 1 {
                return moves.first().unwrap()
            }
            let mut best_moves = Vec::new();
            let mut best_eval = f64::MIN;
            for (i, move_) in moves.iter().enumerate() {
                let mut controller = CheckersController::new(board);
                controller.execute_move(move_);
                let eval = self.minmax(&controller, self.depth - 1, self.get_color().opposite(), true);
                if eval > best_eval {
                    best_eval = eval;
                    best_moves.clear();
                    best_moves.push(i);
                } else if (best_eval - eval).abs() < f64::EPSILON {
                    best_moves.push(i);
                }
            }
            let index = *best_moves.choose(&mut rand::thread_rng()).unwrap();
            &moves[index]
        }

        fn capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
            if captures.len() == 1 {
                return captures.first().unwrap()
            }
            let mut best_captures = Vec::new();
            let mut best_eval = f64::MIN;
            for (i, capture) in captures.iter().enumerate() {
                let mut controller = CheckersController::new(board);
                controller.execute_capture(capture);
                let eval = self.minmax(&controller, self.depth - 1, self.get_color().opposite(), true);
                if eval > best_eval {
                    best_eval = eval;
                    best_captures.clear();
                    best_captures.push(i);
                } else if (best_eval - eval).abs() < f64::EPSILON {
                    best_captures.push(i);
                }
            }
            let index = *best_captures.choose(&mut rand::thread_rng()).unwrap();
            &captures[index]
        }

        fn get_color(&self) -> CheckersColor {
            self.color
        }

        fn set_color(&mut self, color: CheckersColor) {
            self.color = color;
        }
    }
}

pub mod ai {
    use crate::board::Board;
    use crate::controller::{CheckersColor, CheckersController};

    pub trait BoardEstimator {
        fn score(&self, board: &Board, color: CheckersColor) -> f64;
    }

    #[derive(Copy, Clone)]
    pub struct CountEstimator {
        pawn_weight: f64,
        queen_weight: f64
    }

    impl CountEstimator {
        pub fn new(pawn_weight: f64, queen_weight: f64) -> Self {
            Self{pawn_weight, queen_weight} }
    }

    impl BoardEstimator for CountEstimator {
        fn score(&self, board: &Board, maximizing_color: CheckersColor) -> f64 {
            let (pawns, queens) = match maximizing_color {
                CheckersColor::White => (board.num_white_paws(), board.num_white_queens()),
                CheckersColor::Black => (board.num_black_pawns(), board.num_black_queens())
            };
            pawns as f64 * self.pawn_weight + queens as f64 * self.queen_weight
        }
    }

    pub struct WeightMatrixEstimator {
        board_weights: [[f64; 8]; 8],
        pawn_weight: f64,
        queen_weight: f64
    }

    impl WeightMatrixEstimator {
        pub fn new(board_weights: [[f64; 8]; 8], pawn_weight: f64, queen_weight: f64) -> Self {
            Self { board_weights, pawn_weight, queen_weight }
        }
    }

    impl BoardEstimator for WeightMatrixEstimator {
        fn score(&self, board: &Board, color: CheckersColor) -> f64 {
            let controller = CheckersController::new(*board);
            let positions = match color {
                CheckersColor::White => controller.get_white_pieces_position(),
                CheckersColor::Black => controller.get_black_pieces_position(),
            };
            let mut score = 0.0;
            for (x, y) in positions {
                let figure = board.at(x, y);
                if let Some(figure) = figure {
                    if figure.is_queen() {
                        score += self.board_weights[x as usize][y as usize] * self.queen_weight;
                    } else {
                        score += self.board_weights[x as usize][y as usize] * self.pawn_weight;
                    }
                }
            }
            score
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
    pub fn new(controller: CheckersController, mut white_player: WP, mut black_player: BP) -> Self {
        white_player.set_color(CheckersColor::White);
        black_player.set_color(CheckersColor::Black);
        Self {
            controller, white_player, black_player, current_player: CheckersColor::White
        }
    }

    pub fn run(&mut self) -> CheckersColor {
        println!("{}", self.controller.board);
        while let Ok(_) = self.step() {
            self.controller.promote();
            println!("{}", self.controller.board);
            thread::sleep(Duration::from_millis(100));
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
                CheckersColor::White => self.white_player.capture(&captures, self.controller.board),
                CheckersColor::Black => self.black_player.capture(&captures, self.controller.board),
            };
            self.controller.execute_capture(capture);
            self.controller.board.flags = 0;
            self.current_player = self.current_player.opposite();
            return Ok(());
        }
        if !moves.is_empty() {
            let move_ = match self.current_player {
                CheckersColor::White => self.white_player.move_piece(&moves, self.controller.board),
                CheckersColor::Black => self.black_player.move_piece(&moves, self.controller.board),
            };
            self.controller.execute_move(move_);
            self.current_player = self.current_player.opposite();
            return Ok(());
        }
        Err(())
    }
}