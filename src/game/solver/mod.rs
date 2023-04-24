// pub mod dumb;
pub mod engine;
pub mod negamax;

// pub use dumb::DumbSolver;
// pub use negamax::NegamaxSolver;
pub use engine::*;

use crate::game::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Score {
    /* Might want to add a "win in x plies" to Win and Loss */
    Win,
    Loss,
    Draw,
    Numeric(i32),
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Score::Win, Score::Win) => Some(std::cmp::Ordering::Equal),
            (Score::Win, _) => Some(std::cmp::Ordering::Greater),
            (_, Score::Win) => Some(std::cmp::Ordering::Less),

            (Score::Loss, Score::Loss) => Some(std::cmp::Ordering::Equal),
            (Score::Loss, _) => Some(std::cmp::Ordering::Less),
            (_, Score::Loss) => Some(std::cmp::Ordering::Greater),

            (Score::Draw, Score::Draw) => Some(std::cmp::Ordering::Equal),
            (Score::Draw, Score::Numeric(n)) => n.partial_cmp(&0),
            (Score::Numeric(n), Score::Draw) => n.partial_cmp(&0),

            (Score::Numeric(a), Score::Numeric(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("there is no ordering for these scores")
    }
}

impl std::ops::Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Score::Win => Score::Loss,
            Score::Loss => Score::Win,
            Score::Draw => Score::Draw,
            Score::Numeric(n) => Score::Numeric(-n),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Win => write!(f, "+Infinity"),
            Score::Loss => write!(f, "-Infinity"),
            Score::Draw => write!(f, "Draw"),
            Score::Numeric(n) => write!(f, "{}", n),
        }
    }
}

pub trait Solver {
    /// Returns the best move and score for the current player.
    fn get_best_move(&mut self) -> (MoveSequence, i32);
}

pub trait Evaluator {
    fn evaluate(&mut self) -> i32 {
        0
    }
}
