pub mod game;
pub mod helpers;
use crate::game::*;

#[tokio::main]
async fn main() {
    let mut game = Game::new();
    let mut engine = Engine::new(game, Color::Black, 10);
    let (best_move, best_score) = engine.get_best_move().await;
    println!("Best move: {:?}, score: {}", best_move, best_score);
    println!("Searched nodes: {}", engine.searched_nodes);
}
