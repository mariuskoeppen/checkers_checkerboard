// pub mod dumb;
pub mod endgame_table;
pub mod engine;
pub mod negamax;

// pub use dumb::DumbSolver;
// pub use negamax::NegamaxSolver;
pub use endgame_table::*;
pub use engine::*;

use crate::game::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Score;

impl Score {
    pub const INFINITY: i32 = 2_000_000;
    pub const WIN: i32 = 1_000_000;
    pub const DB_WIN: i32 = 400_000;
    pub const DB_MOSTLY_WIN_BONUS: i32 = 5_000;
    pub const DRAW: i32 = 0;
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ScoreOld {
//     /* Might want to add a "win in x plies" to Win and Loss */
//     /// The maximizing player (white) is winning.
//     /// The minimizing player (black) is losing.
//     Win,
//     /// The minimizing player (black) is winning.
//     /// The maximizing player (white) is losing.
//     Loss,
//     /// The game is a (literal) draw.
//     /// Literal meaning that the game is over,
//     /// and not just a positional balance.
//     /// Might also mean the search has exhausted its time.
//     Draw,
//     /// Score is a number returned by the evaluation function.
//     Numeric(i32),
// }

// impl PartialOrd for ScoreOld {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (ScoreOld::Win, ScoreOld::Win) => Some(std::cmp::Ordering::Equal),
//             (ScoreOld::Win, _) => Some(std::cmp::Ordering::Greater),
//             (_, ScoreOld::Win) => Some(std::cmp::Ordering::Less),

//             (ScoreOld::Loss, ScoreOld::Loss) => Some(std::cmp::Ordering::Equal),
//             (ScoreOld::Loss, _) => Some(std::cmp::Ordering::Less),
//             (_, ScoreOld::Loss) => Some(std::cmp::Ordering::Greater),

//             (ScoreOld::Draw, ScoreOld::Draw) => Some(std::cmp::Ordering::Equal),
//             (ScoreOld::Draw, ScoreOld::Numeric(n)) => n.partial_cmp(&0),
//             (ScoreOld::Numeric(n), ScoreOld::Draw) => n.partial_cmp(&0),

//             (ScoreOld::Numeric(a), ScoreOld::Numeric(b)) => a.partial_cmp(b),
//         }
//     }
// }

// impl Ord for ScoreOld {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other)
//             .expect("there is no ordering for these scores")
//     }
// }

// impl std::ops::Neg for ScoreOld {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         match self {
//             ScoreOld::Win => ScoreOld::Loss,
//             ScoreOld::Loss => ScoreOld::Win,
//             ScoreOld::Draw => ScoreOld::Draw,
//             ScoreOld::Numeric(n) => ScoreOld::Numeric(-n),
//         }
//     }
// }

// impl std::fmt::Display for ScoreOld {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ScoreOld::Win => write!(f, "+Infinity"),
//             ScoreOld::Loss => write!(f, "-Infinity"),
//             ScoreOld::Draw => write!(f, "Draw"),
//             ScoreOld::Numeric(n) => write!(f, "{}", n),
//         }
//     }
// }

pub trait Solver {
    /// Returns the best move and score for the current player.
    fn get_best_move(&mut self) -> (MoveSequence, i32);
}

pub trait Evaluator {
    fn evaluate(&mut self) -> i32 {
        0
    }
}
