#![allow(unused)]

pub mod game;
pub mod helpers;
pub mod transposition_table;

use std::time::Duration;

use crate::game::*;

#[tokio::main]
async fn main() {
    let mut game = Game::new();
    let mut engine = Engine::new(game, Color::Black, Duration::from_secs(1));

    let start_time = std::time::Instant::now();

    let (best_move, best_score) = engine.get_best_move().await;

    let end_time = std::time::Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
    println!("Best move: {:?}, score: {}", best_move, best_score);
    println!("Searched nodes: {}", engine.searched_nodes);
    println!("Reached depth: {}", engine.current_depth);
}
