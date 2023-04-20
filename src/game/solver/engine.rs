#![allow(unused)]

use std::time::Duration;

use super::*;

#[derive(Debug)]
pub struct Engine {
    game: Box<Game>,
    pub color: Color,
    pub current_depth: usize,
    pub max_time: Duration,
    start_time: std::time::Instant,
    stopped_searching: bool,
    pub best_move: Option<MoveSequence>,
    pub best_score: i32,
    pub searched_nodes: usize,
}

impl Engine {
    pub fn new(game: Game, color: Color, max_time: Duration) -> Self {
        Engine {
            game: Box::new(game),
            color,
            current_depth: 0,
            max_time,
            start_time: std::time::Instant::now(),
            stopped_searching: false,
            best_move: None,
            best_score: 0,
            searched_nodes: 0,
        }
    }
}

impl Engine {
    pub async fn get_best_move(&mut self) -> (Option<MoveSequence>, i32) {
        if self.game.side_to_move != self.color {
            panic!("Engine is not playing as the side to move.");
        }

        // Iterative deepening.
        self.start_time = std::time::Instant::now();
        self.searched_nodes = 0;
        self.current_depth = 0;
        self.stopped_searching = false;

        loop {
            self.current_depth += 1;

            self.search(self.current_depth, -i32::MAX, i32::MAX);

            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            if elapsed_time >= self.max_time || self.current_depth >= 32 || self.stopped_searching {
                break;
            }
        }

        (self.best_move.clone(), self.best_score)
    }
}

impl Engine {
    fn search(&mut self, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
        self.searched_nodes += 1;

        if self.searched_nodes % 10_000 == 0 {
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            if elapsed_time >= self.max_time {
                self.stopped_searching = true;
                return 0;
            }
        }

        if depth == 0 {
            return self.quiescence_search(alpha, beta);
        }

        if self.game.is_black_win() {
            match self.color {
                Color::White => return -i32::MAX,
                Color::Black => return i32::MAX,
            }
        } else if self.game.is_white_win() {
            match self.color {
                Color::White => return i32::MAX,
                Color::Black => return -i32::MAX,
            }
        } else if self.game.is_draw() {
            return 0;
        }

        let mut best_score = -i32::MAX;
        let mut best_move = None;
        let mut available_moves = self.game.generate_move_sequences();
        Engine::order_moves(&mut available_moves);

        for m in available_moves {
            self.game.make_move_sequence(&m);
            let score = -self.search(depth - 1, -beta, -alpha);
            self.game.unmake_move_sequence();

            if self.stopped_searching {
                return 0;
            }

            if score >= beta {
                return score;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(m);

                if score > alpha {
                    alpha = score;
                }
            }
        }

        // Only update the best move if we are at the top level of the search tree.
        if depth == self.current_depth && !self.stopped_searching {
            self.best_move = best_move;
            self.best_score = best_score;
        }

        best_score
    }

    fn quiescence_search(&mut self, mut alpha: i32, mut beta: i32) -> i32 {
        self.searched_nodes += 1;

        if self.searched_nodes % 10_000 == 0 {
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            if elapsed_time >= self.max_time {
                self.stopped_searching = true;
                return 0;
            }
        }

        if self.game.is_black_win() {
            match self.color {
                Color::White => return -i32::MAX,
                Color::Black => return i32::MAX,
            }
        } else if self.game.is_white_win() {
            match self.color {
                Color::White => return i32::MAX,
                Color::Black => return -i32::MAX,
            }
        } else if self.game.is_draw() {
            return 0;
        }

        let standing_pat = self.evaluate();
        if standing_pat >= beta {
            return beta;
        }

        if alpha < standing_pat {
            alpha = standing_pat;
        }

        let mut best_score = -i32::MAX;
        let mut available_moves = self.game.generate_capture_move_sequences();
        Engine::order_moves(&mut available_moves);

        for m in available_moves {
            self.game.make_move_sequence(&m);
            let score = -self.quiescence_search(-beta, -alpha);
            self.game.unmake_move_sequence();

            if self.stopped_searching {
                return 0;
            }

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}

impl Engine {
    /// Evaluate the current position from the perspective of the current player.
    /// White has generally speaking a positive score while black has a negative score.
    /// This takes into account that the engine might be playing as either color.
    /// Returns a positive score if the current player is winning, and a negative score if the current player is losing.
    ///
    /// The score is calculated as follows:
    /// - Material: 1000 The number of pieces on the board.
    /// - Kings: 1450 The number of kings on the board.
    fn evaluate(&mut self) -> i32 {
        // Score increases as white is winning, and decreases as black is winning.
        let mut score = 0;

        // Material
        score += 1000 * (self.game.white.count() as i32 - self.game.black.count() as i32);

        // Kings
        score +=
            450 * (self.game.white_kings.count() as i32 - self.game.black_kings.count() as i32);

        match self.color {
            Color::White => score,
            Color::Black => -score,
        }
    }
}

impl Engine {
    fn order_moves(moves: &mut Vec<MoveSequence>) {
        moves.sort_by(|a, b| {
            let a_score = a.score();
            let b_score = b.score();
            b_score.cmp(&a_score)
        });
    }
}
