pub mod game;
pub mod helpers;
use crate::game::*;

fn main() {
    let mut game = Game::new();
    println!("{:#?}", game);
}
