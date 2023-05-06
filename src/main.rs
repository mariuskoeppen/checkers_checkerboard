#![allow(unused)]

pub mod game;
pub mod helpers;
pub mod transposition_table;

use std::time::Duration;

use crate::game::*;

#[tokio::main]
async fn main() {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    for _ in 0..9 {
        let mut game = Game::new();
        let mut black_engine = Engine::new(Color::Black, Duration::from_millis(50));
        let mut white_engine = Engine::new(Color::White, Duration::from_millis(10));

        // let (best_move, score, pv) = black_engine.find_best_move(&mut game).await;
        // println!("Best move: {:?} <{}> {} ", best_move, score, pv);

        loop {
            let black_move = black_engine.find_best_move(&mut game).await;
            game.make_move_sequence(&black_move.0.clone().unwrap());
            println!(
                "B <{}> ({}) {}",
                black_engine.current_depth - 1,
                Engine::translate_score(black_move.1, Color::Black, game.ply),
                black_move.2
            );

            println!("{}", game.to_console_string());

            if game.is_draw() {
                println!("Game over! Draw!");
                draws += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            } else if game.is_black_win() {
                println!("Game over! Black won!");
                black_wins += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            } else if game.is_white_win() {
                println!("Game over! White won!");
                white_wins += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            }

            let white_move = white_engine.find_best_move(&mut game).await;
            game.make_move_sequence(&white_move.0.clone().unwrap());
            println!(
                "W <{}> ({}) {}",
                white_engine.current_depth - 1,
                Engine::translate_score(white_move.1, Color::White, game.ply),
                white_move.2
            );

            println!("{}", game.to_console_string());

            if game.is_draw() {
                println!("Game over! Draw!");
                draws += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            } else if game.is_black_win() {
                println!("Game over! Black won!");
                black_wins += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            } else if game.is_white_win() {
                println!("Game over! White won!");
                white_wins += 1;
                println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
                break;
            }
        }
    }

    println!("stats: b: {} w: {} d: {}", black_wins, white_wins, draws);
}
