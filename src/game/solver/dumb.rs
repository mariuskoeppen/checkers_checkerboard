use super::*;

#[derive(Debug)]
pub struct DumbSolver {
    pub game: Game,
    pub side: Color,
}

impl DumbSolver {
    pub fn new(game: Game, side: Color) -> Self {
        DumbSolver { game, side }
    }
}

impl Solver for DumbSolver {
    fn get_best_move(&mut self) -> (MoveSequence, i32) {
        let mut best_score = 0;

        let available_moves = self.game.generate_move_sequences();
        let mut best_move = available_moves[0].clone();

        (best_move, best_score)
    }
}