pub mod game;
pub mod helpers;
use crate::game::*;

fn main() {
    let mut game = Game::new();
    println!("{:#?}", game);

    // // Make move Black 9-14
    // game.make_move_sequence(&MoveSequence::new(vec![Move::new(
    //     Color::Black,
    //     9,
    //     14,
    //     None,
    //     false,
    //     false,
    //     false,
    // )]));

    // game.make_move_external("9-14");
    // game.make_move_external("24-20");
    // game.make_move_external("5-9");
    // game.make_move_external("22-18");
    // game.make_move_external("14-17");
    // game.make_move_external("21x14x5");

    // println!("ply {}", game.ply);
    // println!("white {}", game.white);
    // println!("black {}", game.black);
    // println!("white kings {}", game.white_kings);
    // println!("black kings {}", game.black_kings);

    game.divide(1);
}
