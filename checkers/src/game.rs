use std::fmt::Display;
use std::thread;
use std::time::Duration;
use crate::controller::{CheckersController, CheckersColor, CheckersAction, Figure};
use rand::seq::SliceRandom;
use crate::game::player::Player;

pub mod player {
    use std::cmp::max;
    use std::io;
    use std::sync::{Arc, Mutex};
    use rand::seq::SliceRandom;
    use rayon::prelude::*;
    use crate::board::Board;
    use crate::controller::{CheckersColor, CheckersController, JumpChain, Move};
    use crate::game::estimators::BoardEstimator;

    pub trait Player {
        fn choose_move<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move;
        fn choose_capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain;
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
        fn choose_move<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            let m = moves.choose(&mut rand::thread_rng()).unwrap();
            println!("{m}");
            m
        }

        fn choose_capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
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
        fn choose_move<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            for (i, move_) in moves.iter().enumerate() {
                println!("{i}. {move_}");
            }
            let index = self.get_index(moves.len());
            &moves[index]
        }

        fn choose_capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
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
                if input == "" {
                    return 0;
                }
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
        color: CheckersColor,
        nodes_visited: Arc<Mutex<usize>>
    }

    impl <T> MinMaxBot<T> {

        pub const MIN_SCORE: f64 = -1e10;
        pub const MAX_SCORE: f64 = 1e10;

        pub fn new(estimator: T, depth: usize) -> Self { Self{estimator, depth, color: CheckersColor::White, nodes_visited: Arc::new(Mutex::new(0))} }
    }

    impl <T: BoardEstimator> MinMaxBot<T> {
        fn minmax(&self, controller: &CheckersController, depth: usize, current_color: CheckersColor) -> f64 {
            if depth == 0 {
                {
                    let mut nodes_visited = self.nodes_visited.lock().unwrap();
                    *nodes_visited += 1;
                }
                return self.estimator.score(&controller.board);
            }
            let idle_moves = match self.color {
                CheckersColor::White => controller.get_white_queen_idle_moves(),
                CheckersColor::Black => controller.get_black_queen_idle_moves(),
            };
            if idle_moves > 8 {
                return if current_color.is_white() { Self::MIN_SCORE } else { Self::MAX_SCORE };
            }
            let (jumps, moves) = controller.options(current_color);
            if !jumps.is_empty() {
                return self.minmax_jumps(controller, &jumps, depth, current_color);
            }
            if !moves.is_empty() {
                return self.minmax_moves(controller, &moves, depth, current_color);
            }
            if current_color.is_white() { Self::MIN_SCORE } else { Self::MAX_SCORE }
        }

        fn minmax_jumps(&self, controller: &CheckersController, captures: &[JumpChain], depth: usize, current_color: CheckersColor) -> f64 {
            let mut current = if current_color.is_white() { f64::MIN } else { f64::MAX };
            for capture in captures {
                let mut controller = CheckersController::with_idle_moves(
                    controller.board,
                    controller.get_white_queen_idle_moves(),
                    controller.get_black_queen_idle_moves()
                );
                controller.execute_capture(capture);
                let est = self.minmax(&controller, depth - 1, current_color.opposite());
                if current_color.is_white() {
                    if est > current {
                        current = est;
                    }
                } else {
                    if est < current {
                        current = est;
                    }
                }
            }
            current
        }

        fn minmax_moves(&self, controller: &CheckersController, moves: &[Move], depth: usize, current_color: CheckersColor) -> f64 {
            let mut current = if current_color.is_white() { f64::MIN } else { f64::MAX };
            for move_ in moves {
                let mut controller = CheckersController::with_idle_moves(
                    controller.board,
                    controller.get_white_queen_idle_moves(),
                    controller.get_black_queen_idle_moves(),
                );
                controller.execute_move(move_);
                let est = self.minmax(&controller, depth - 1, current_color.opposite());
                if current_color.is_white() {
                    if est > current {
                        current = est;
                    }
                } else {
                    if est < current {
                        current = est;
                    }
                }
            }
            current
        }

        fn get_best_eval(&self, evals: &Vec<(usize, f64)>) -> f64 {
            if self.color.is_white() {
                evals
                    .iter()
                    .map(|(_, eval)| eval)
                    .cloned()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(f64::MIN)
            } else {
                evals
                    .iter()
                    .map(|(_, eval)| eval)
                    .cloned()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(f64::MAX)
            }
        }

        fn get_best_indices(&self, evals: &Vec<(usize, f64)>, best: f64) -> Vec<usize> {
            evals
                .iter()
                .filter(|(i, eval)| (eval - best).abs() < f64::EPSILON)
                .map(|(i, _)| *i)
                .collect()
        }

        fn get_at_indices<'a, U>(&'a self, indices: &[usize], values: &'a [U]) -> Vec<&U> {
            let mut ret = Vec::with_capacity(indices.len());
            for &i in indices {
                ret.push(&values[i])
            }
            ret
        }
    }

    impl <T: BoardEstimator + Sync> Player for MinMaxBot<T> {
        fn choose_move<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                *nodes_visited = 0;
            }
            if moves.len() == 1 {
                return moves.first().unwrap()
            }

            let moves_eval: Vec<(usize, f64)> = moves
                .par_iter()
                .enumerate()
                .map(|(i, move_)| {
                    let mut controller = CheckersController::new(board);
                    controller.execute_move(move_);
                    let eval = self.minmax(&controller, self.depth -1, self.get_color().opposite());
                    (i, eval)
                }).collect();
            let best_eval = self.get_best_eval(&moves_eval);
            let indices = self.get_best_indices(&moves_eval, best_eval);
            let best_moves = self.get_at_indices(&indices, moves);

            println!("{:?} MinMaxBot best: {}", self.color, best_eval * if self.color.is_white() {1.0} else {-1.0});
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                println!("Universes visited: {}", *nodes_visited);
            }
            best_moves.choose(&mut rand::thread_rng()).unwrap()
        }

        fn choose_capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                *nodes_visited = 0;
            }

            if captures.len() == 1 {
                return captures.first().unwrap()
            }

            let captures_eval: Vec<(usize, f64)> = captures
                .par_iter()
                .enumerate()
                .map(|(i, capture)| {
                    let mut controller = CheckersController::new(board);
                    controller.execute_capture(capture);
                    let eval = self.minmax(&controller, self.depth - 1, self.get_color().opposite());
                    (i, eval)
                })
                .collect();

            let best_eval = self.get_best_eval(&captures_eval);
            let indices = self.get_best_indices(&captures_eval, best_eval);
            let best_captures = self.get_at_indices(&indices, captures);

            println!("{:?} MinMaxBot best: {}",self.color , best_eval * if self.color.is_white() {1.0} else {-1.0});
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                println!("Universes visited: {}", *nodes_visited);
            }
            best_captures.choose(&mut rand::thread_rng()).unwrap()
        }

        fn get_color(&self) -> CheckersColor {
            self.color
        }

        fn set_color(&mut self, color: CheckersColor) {
            self.color = color;
        }
    }

    pub struct AlphaBetaBot<T> {
        estimator: T,
        depth: usize,
        color: CheckersColor,
        nodes_visited: Arc<Mutex<usize>>
    }

    impl <T> AlphaBetaBot<T> {
        pub const MIN_SCORE: f64 = -1e10;
        pub const MAX_SCORE: f64 = 1e10;

        pub fn new(estimator: T, depth: usize) -> Self {
            Self{estimator, depth, color: CheckersColor::White, nodes_visited: Arc::new(Mutex::new(0))}
        }

        fn get_best_eval(&self, evals: &Vec<(usize, f64)>) -> f64 {
            if self.color.is_white() {
                evals
                    .iter()
                    .map(|(_, eval)| eval)
                    .cloned()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(f64::MIN)
            } else {
                evals
                    .iter()
                    .map(|(_, eval)| eval)
                    .cloned()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(f64::MAX)
            }
        }

        fn get_best_indices(&self, evals: &Vec<(usize, f64)>, best: f64) -> Vec<usize> {
            evals
                .iter()
                .filter(|(i, eval)| (eval - best).abs() < f64::EPSILON)
                .map(|(i, _)| *i)
                .collect()
        }

        fn get_at_indices<'a, U>(&'a self, indices: &[usize], values: &'a [U]) -> Vec<&U> {
            let mut ret = Vec::with_capacity(indices.len());
            for &i in indices {
                ret.push(&values[i])
            }
            ret
        }
    }

    impl <T: BoardEstimator> AlphaBetaBot<T> {
        fn minmax(
            &self,
            controller: &CheckersController,
            depth: usize,
            current_color: CheckersColor,
            alpha: f64,
            beta: f64
        ) -> f64 {
            if depth == 0 {
                {
                    let mut nodes_visited = self.nodes_visited.lock().unwrap();
                    *nodes_visited += 1;
                }
                return self.estimator.score(&controller.board);
            }
            let idle_moves = match self.color {
                CheckersColor::White => controller.get_white_queen_idle_moves(),
                CheckersColor::Black => controller.get_black_queen_idle_moves(),
            };
            if idle_moves > 8 {
                return if current_color.is_white() { Self::MIN_SCORE } else { Self::MAX_SCORE };
            }
            let (jumps, moves) = controller.options(current_color);
            if !jumps.is_empty() {
                return self.minmax_jumps(controller, &jumps, depth, current_color, alpha, beta);
            }
            if !moves.is_empty() {
                return self.minmax_moves(controller, &moves, depth, current_color, alpha, beta);
            }
            if current_color.is_white() { Self::MIN_SCORE } else { Self::MAX_SCORE }
        }

        fn minmax_jumps(
            &self,
            controller: &CheckersController,
            captures: &[JumpChain],
            depth: usize,
            current_color: CheckersColor,
            alpha: f64,
            beta: f64
        ) -> f64 {
            let mut current = if current_color.is_white() { f64::MIN } else { f64::MAX };
            let mut new_cut = current;
            for capture in captures {
                let mut controller = CheckersController::with_idle_moves(
                    controller.board,
                    controller.get_white_queen_idle_moves(),
                    controller.get_black_queen_idle_moves()
                );
                controller.execute_capture(capture);
                let est = self.minmax(
                    &controller,
                    depth - 1,
                    current_color.opposite(),
                    if current_color.is_white() { new_cut } else {alpha},
                    if current_color.is_white() {beta} else {new_cut}
                );
                if current_color.is_white() {
                    if est > current {
                        current = est;
                    }
                    if est > new_cut {
                        new_cut = est;
                    }
                    if beta <= new_cut {
                        break
                    }
                } else {
                    if est < current {
                        current = est;
                    }
                    if est < new_cut {
                        new_cut = est;
                    }
                    if new_cut < alpha {
                        break;
                    }
                }
            }
            current
        }

        fn minmax_moves(
            &self,
            controller: &CheckersController,
            moves: &[Move],
            depth: usize,
            current_color: CheckersColor,
            alpha: f64,
            beta: f64
        ) -> f64 {
            let mut current = if current_color.is_white() { f64::MIN } else { f64::MAX };
            let mut new_cut = current;
            for move_ in moves {
                let mut controller = CheckersController::with_idle_moves(
                    controller.board,
                    controller.get_white_queen_idle_moves(),
                    controller.get_black_queen_idle_moves(),
                );
                controller.execute_move(move_);
                let est = self.minmax(
                    &controller,
                    depth - 1,
                    current_color.opposite(),
                    if current_color.is_white() { new_cut } else {alpha},
                    if current_color.is_white() {beta} else {new_cut}
                );
                if current_color.is_white() {
                    if est > current {
                        current = est;
                    }
                    if est > new_cut {
                        new_cut = est;
                    }
                    if beta <= new_cut {
                        break
                    }
                } else {
                    if est < current {
                        current = est;
                    }
                    if est < new_cut {
                        new_cut = est;
                    }
                    if new_cut < alpha {
                        break;
                    }
                }
            }
            current
        }
    }

    impl <T: BoardEstimator + Sync> Player for AlphaBetaBot<T> {
        fn choose_move<'a>(&'a self, moves: &'a [Move], board: Board) -> &Move {
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                *nodes_visited = 0;
            }
            if moves.len() == 1 {
                return moves.first().unwrap()
            }

            let moves_eval: Vec<(usize, f64)> = moves
                .par_iter()
                .enumerate()
                .map(|(i, move_)| {
                    let mut controller = CheckersController::new(board);
                    controller.execute_move(move_);
                    let eval = self.minmax(&controller, self.depth - 1, self.get_color().opposite(), Self::MIN_SCORE - 1.0, Self::MAX_SCORE + 1.0);
                    (i, eval)
                }).collect();
            let best_eval = self.get_best_eval(&moves_eval);
            let indices = self.get_best_indices(&moves_eval, best_eval);
            let best_moves = self.get_at_indices(&indices, moves);

            println!("{:?} AlphaBetaBot best: {}", self.color, best_eval * if self.color.is_white() {1.0} else {-1.0});
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                println!("Universes visited: {}", *nodes_visited);
            }
            best_moves.choose(&mut rand::thread_rng()).unwrap()
        }

        fn choose_capture<'a>(&'a self, captures: &'a [JumpChain], board: Board) -> &JumpChain {
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                *nodes_visited = 0;
            }

            if captures.len() == 1 {
                return captures.first().unwrap()
            }

            let captures_eval: Vec<(usize, f64)> = captures
                .par_iter()
                .enumerate()
                .map(|(i, capture)| {
                    let mut controller = CheckersController::new(board);
                    controller.execute_capture(capture);
                    let eval = self.minmax(&controller, self.depth - 1, self.get_color().opposite(), Self::MIN_SCORE - 1.0, Self::MAX_SCORE + 1.0);
                    (i, eval)
                })
                .collect();

            let best_eval = self.get_best_eval(&captures_eval);
            let indices = self.get_best_indices(&captures_eval, best_eval);
            let best_captures = self.get_at_indices(&indices, captures);

            println!("{:?} AlphaBetaBot best: {}",self.color , best_eval * if self.color.is_white() {1.0} else {-1.0});
            {
                let mut nodes_visited = self.nodes_visited.lock().unwrap();
                println!("Universes visited: {}", *nodes_visited);
            }
            best_captures.choose(&mut rand::thread_rng()).unwrap()
        }

        fn get_color(&self) -> CheckersColor {
            self.color
        }

        fn set_color(&mut self, color: CheckersColor) {
            self.color = color;
        }
    }
}

pub mod estimators {
    use crate::board::Board;
    use crate::controller::{CheckersColor, CheckersController};

    pub trait BoardEstimator {
        fn score(&self, board: &Board) -> f64;
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
        fn score(&self, board: &Board) -> f64 {
            let pawns = board.num_pawns(CheckersColor::White);
            let queens = board.num_queens(CheckersColor::White);
            let enemy_pawns = board.num_pawns(CheckersColor::Black);
            let enemy_queens = board.num_queens(CheckersColor::Black);
            let positive = pawns as f64 * self.pawn_weight + queens as f64 * self.queen_weight;
            let negative = enemy_pawns as f64 * self.pawn_weight + enemy_queens as f64 * self.queen_weight;
            positive - negative
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
        fn score(&self, board: &Board) -> f64 {
            let controller = CheckersController::new(*board);
            let white_positions = controller.get_white_pieces_position();
            let black_positions = controller.get_black_pieces_position();

            let mut white_score = 0.0;
            for (x, y) in white_positions {
                let figure = board.at(x, y);
                if let Some(figure) = figure {
                    if figure.is_queen() {
                        white_score += self.board_weights[x as usize][y as usize] * self.queen_weight;
                    } else {
                        white_score += self.board_weights[x as usize][y as usize] * self.pawn_weight;
                    }
                }
            }

            let mut black_score = 0.0;
            for (x, y) in black_positions {
                let figure = board.at(x, y);
                if let Some(figure) = figure {
                    if figure.is_queen() {
                        black_score += self.board_weights[x as usize][y as usize] * self.queen_weight;
                    } else {
                        black_score += self.board_weights[x as usize][y as usize] * self.pawn_weight;
                    }
                }
            }
            white_score - black_score
        }
    }
}


pub struct Game<WP, BP> {
    controller: CheckersController,
    white_player : WP,
    black_player: BP,
    current_player: CheckersColor,
}


impl <WP: Player, BP: Player> Game<WP, BP> {
    pub fn new(controller: CheckersController, mut white_player: WP, mut black_player: BP) -> Self {
        white_player.set_color(CheckersColor::White);
        black_player.set_color(CheckersColor::Black);
        Self {
            controller,
            white_player,
            black_player,
            current_player: CheckersColor::White
        }
    }

    pub fn run(&mut self) -> CheckersColor {
        println!("{}", self.controller.board);
        while let Some(_) = self.step() {
            self.controller.promote();
            println!("{}", self.controller.board);
            if self.controller.board.num_white_figures() == 0 {
                return CheckersColor::Black;
            }
            if self.controller.board.num_black_figures() == 0 {
                return CheckersColor::White;
            }
        }
        self.current_player.opposite()
    }

    pub fn step(&mut self) -> Option<()> {
        let idle_moves = match self.current_player {
            CheckersColor::White => self.controller.get_white_queen_idle_moves(),
            CheckersColor::Black => self.controller.get_black_queen_idle_moves(),
        };
        if idle_moves > 8 {
            return None
        }
        let (captures, moves) = self.controller.options(self.current_player);
        if !captures.is_empty() {
            let capture = match self.current_player {
                CheckersColor::White => self.white_player.choose_capture(&captures, self.controller.board),
                CheckersColor::Black => self.black_player.choose_capture(&captures, self.controller.board),
            };
            self.controller.execute_capture(capture);
            self.controller.board.flags = 0;
            self.current_player = self.current_player.opposite();
            return Some(());
        }
        if !moves.is_empty() {
            let move_ = match self.current_player {
                CheckersColor::White => self.white_player.choose_move(&moves, self.controller.board),
                CheckersColor::Black => self.black_player.choose_move(&moves, self.controller.board),
            };
            self.controller.execute_move(move_);
            self.current_player = self.current_player.opposite();
            return Some(());
        }
        None
    }
}