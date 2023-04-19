#![allow(unused)]

use super::*;

#[derive(Debug)]
pub struct Engine {
    game: Box<Game>,
    pub color: Color,
    pub max_depth: usize,
    pub best_move: Option<MoveSequence>,
    pub best_score: i32,
    pub searched_nodes: usize,
}

impl Engine {
    pub fn new(game: Game, color: Color, max_depth: usize) -> Self {
        Engine {
            game: Box::new(game),
            color,
            max_depth,
            best_move: None,
            best_score: 0,
            searched_nodes: 0,
        }
    }
}

impl Engine {
    pub async fn get_best_move(&mut self) -> (Option<MoveSequence>, i32) {
        self.searched_nodes = 0;

        self.search(self.max_depth);

        (self.best_move.clone(), self.best_score)
    }
}

impl Engine {
    fn search(&mut self, depth: usize) -> i32 {
        self.searched_nodes += 1;

        if self.game.is_black_win() {
            match self.color {
                Color::White => return i32::MIN,
                Color::Black => return i32::MAX,
            }
        } else if self.game.is_white_win() {
            match self.color {
                Color::White => return i32::MAX,
                Color::Black => return i32::MIN,
            }
        } else if self.game.is_draw() {
            return 0;
        }

        if depth == 0 {
            return self.evaluate();
        }

        let mut best_score = i32::MIN;
        let available_moves = self.game.generate_move_sequences();

        for m in available_moves {
            self.game.make_move_sequence(&m);
            let score = -self.search(depth - 1);
            if score > best_score {
                best_score = score;

                // Only update the best move if we are at the top level of the search tree.
                if depth == self.max_depth {
                    self.best_move = Some(m);
                    self.best_score = score;
                }
            }
            self.game.unmake_move_sequence();
        }

        best_score
    }
}

impl Engine {
    /// Evaluate the current position from the perspective of the current player.
    /// White has generally speaking a positive score while black has a negative score.
    /// This takes into account that the engine might be playing as either color.
    /// Returns a positive score if the current player is winning, and a negative score if the current player is losing.
    fn evaluate(&mut self) -> i32 {
        // Score increases as white is winning, and decreases as black is winning.
        let mut score = 0;

        // Material
        score += self.game.white.count() as i32 - self.game.black.count() as i32;

        match self.color {
            Color::White => score,
            Color::Black => -score,
        }
    }
}
