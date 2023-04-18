pub mod bitboard;
pub mod game;
pub mod move_sequence;

pub use bitboard::Bitboard;
pub use game::Game;
pub use move_sequence::{Move, MoveSequence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn switch(&mut self) {
        *self = match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}