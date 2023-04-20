#![allow(unused)]

pub mod game;
pub mod helpers;
pub mod transposition_table;

use std::time::Duration;

use crate::game::*;

#[tokio::main]
async fn main() {
    let mut game = Game::new();
    let mut game = Box::new(game);

    let mut black_engine = Engine::new(Color::Black, Duration::from_millis(1000));
    let mut white_engine = Engine::new(Color::White, Duration::from_millis(10));

    loop {
        let black_move = black_engine.get_best_move(&mut game).await;
        game.make_move_sequence(&black_move.0.clone().unwrap());
        println!("B ({}) {:?}", black_move.1, black_move.2);

        println!("{}", game.to_console_string());

        if game.is_terminal() {
            println!(
                "Game over! {} won!",
                match game.is_white_win() {
                    true => "White",
                    false => "Black",
                }
            );
            break;
        }

        let white_move = white_engine.get_best_move(&mut game).await;
        game.make_move_sequence(&white_move.0.clone().unwrap());
        println!("W ({}) {:?}", white_move.1, white_move.2);

        println!("{}", game.to_console_string());

        if game.is_terminal() {
            println!(
                "Game over! {} won!",
                match game.is_white_win() {
                    true => "White",
                    false => "Black",
                }
            );
            break;
        }
    }
}
