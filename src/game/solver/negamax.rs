use super::*;

#[derive(Debug)]
pub struct NegamaxSolver {
    game: Game,
    color: Color,
    depth: usize,
    best_move: Option<MoveSequence>,
}

impl NegamaxSolver {
    pub fn new(game: Game, color: Color, depth: usize) -> Self {
        NegamaxSolver {
            game,
            color,
            depth,
            best_move: None,
        }
    }
}

impl NegamaxSolver {
    pub async fn get_best_move(&mut self) -> (Option<MoveSequence>, i32) {
        let best_score = self.negamax(self.depth);
        (self.best_move.clone(), best_score)
    }
}

impl NegamaxSolver {
    fn negamax(&mut self, depth: usize) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }

        let mut best_score = i32::MIN;
        let available_moves = self.game.generate_move_sequences();

        for m in available_moves {
            self.game.make_move_sequence(&m);
            let score = -self.negamax(depth - 1);
            if score > best_score {
                best_score = score;
                if depth == self.depth {
                    self.best_move = Some(m);
                }
            }
            self.game.unmake_move_sequence();
        }

        best_score
    }
}

impl Evaluator for NegamaxSolver {
    fn evaluate(&mut self) -> i32 {
        match self.color {
            Color::White => self.game.white.count() as i32 - self.game.black.count() as i32,
            Color::Black => self.game.black.count() as i32 - self.game.white.count() as i32,
        }
    }
}
