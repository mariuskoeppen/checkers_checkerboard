// pub mod dumb;
pub mod negamax;
pub mod engine;

// pub use dumb::DumbSolver;
// pub use negamax::NegamaxSolver;
pub use engine::*;

use crate::game::*;

pub trait Solver {
    /// Returns the best move and score for the current player.
    fn get_best_move(&mut self) -> (MoveSequence, i32);
}

pub trait Evaluator {
    fn evaluate(&mut self) -> i32 {
        0
    }
}