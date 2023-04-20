#![allow(unused)]

use std::time::Duration;

use crate::{game::*, helpers::ImprovedPositionMapper};

#[derive(Debug)]
pub struct Game {
    /// White pieces.
    pub white: Bitboard,
    /// Black pieces.
    pub black: Bitboard,
    /// White kings.
    // Maybe white kings and black kings coulbe be combined into one variable
    pub white_kings: Bitboard,
    /// Black kings.
    pub black_kings: Bitboard,
    /// The side to move.
    pub side_to_move: Color,
    /// The move history.
    pub move_history: Vec<MoveSequence>,
    /// The current ply. One ply = one side's turn (half-move).
    pub ply: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let white =
            Bitboard::default().set_multiple(&(Bitboard::ONE | Bitboard::TWO | Bitboard::THREE));
        let black =
            Bitboard::default().set_multiple(&(Bitboard::SIX | Bitboard::SEVEN | Bitboard::EIGHT));

        // let white = Bitboard::from_u64(825439027200);
        // let black = Bitboard::from_u64(16777216);

        Self {
            white,
            black,
            white_kings: Bitboard::EMPTY,
            black_kings: Bitboard::EMPTY,
            side_to_move: Color::Black,
            move_history: Vec::new(),
            ply: 0,
        }
    }

    pub fn not_occupied(&self) -> Bitboard {
        Bitboard::ALL & !(&self.white | &self.black)
    }

    pub fn is_terminal(&mut self) -> bool {
        // Also need to check if its a draw
        self.is_white_win() || self.is_black_win() || self.is_draw()
    }

    pub fn is_black_win(&mut self) -> bool {
        // We might want to optimize this by not neccecarily generating all the moves
        self.white.is_empty() || self.generate_white_move_sequences().is_empty()
    }

    pub fn is_white_win(&mut self) -> bool {
        self.black.is_empty() || self.generate_black_move_sequences().is_empty()
    }

    pub fn is_draw(&mut self) -> bool {
        false
    }
}

/// External api implementation
impl Game {
    pub fn make_move_external(&mut self, m: &str) -> Result<(), String> {
        let type_of_sequence = match m.contains('x') {
            true => MoveSequenceType::Jump,
            false => MoveSequenceType::Slide,
        };

        let positions = match type_of_sequence {
            MoveSequenceType::Jump => m.split('x'),
            MoveSequenceType::Slide => m.split('-'),
        }
        .map(|s| s.parse::<usize>().map_err(|_| "Invalid move".to_string()))
        .collect::<Result<Vec<usize>, String>>()?;

        let available_move_sequences = self.generate_move_sequences();

        for ms in available_move_sequences {
            if ms.move_sequence_type() == type_of_sequence && ms.positions() == positions {
                self.make_move_sequence(&ms);
                return Ok(());
            }
        }

        Err("No valid move sequences found".to_string())
    }

    pub fn to_console_string(&self) -> String {
        let mut s = String::new();

        s += "\n------------------------------------------------------------\n";

        s += &format!("{}: ", &self.ply.to_string());
        s += match self.side_to_move {
            Color::White => "White moves next ",
            Color::Black => "Black moves next ",
        };
        let mut e = Engine::new(self.side_to_move.clone(), Duration::from_secs(1));
        s += &format!("<{}>", e.evaluate(&self));
        s += "\n\n";

        for row in 0..8 {
            s += "  ";
            s += &(8 - row).to_string();
            s += "  |";
            for column in 0..8 {
                match (row % 2 == 0 && column % 2 == 0) || (row % 2 == 1 && column % 2 == 1) {
                    true => {
                        s += " |";
                        continue;
                    }
                    false => {}
                };

                let position: usize = row * 4 + column / 2 + 1;
                let index = ImprovedPositionMapper::position_to_index[position];

                s.push_str(if self.white.get(index) {
                    if self.white_kings.get(index) {
                        "W"
                    } else {
                        "w"
                    }
                } else if self.black.get(index) {
                    if self.black_kings.get(index) {
                        "B"
                    } else {
                        "b"
                    }
                } else {
                    " "
                });

                s += "|";
            }

            s += "\t\t|";
            for column in 0..8 {
                match (row % 2 == 0 && column % 2 == 0) || (row % 2 == 1 && column % 2 == 1) {
                    true => {
                        s += "  |";
                        continue;
                    }
                    false => {}
                };
                let position: usize = row * 4 + column / 2 + 1;

                if position < 10 {
                    s += " ";
                }
                s += &position.to_string();
                s += "|";
            }

            s += "\n";
        }

        s += "\n    a b c d e f g h";
        s += "\t\t  a  b  c  d  e  f  g  h";

        s += "\n------------------------------------------------------------\n";

        s
    }
}

/// Make move implementation
impl Game {
    pub fn make_move_sequence(&mut self, moves_sequence: &MoveSequence) {
        for mov in moves_sequence.clone().into_iter() {
            self.make_move(&mov);
        }

        self.ply += 1;
        self.side_to_move.switch();
        self.move_history.push(moves_sequence.clone());
    }

    pub fn unmake_move_sequence(&mut self) {
        let move_sequence = self.move_history.pop().expect("No moves to unmake");
        for mov in move_sequence.into_iter().rev() {
            self.unmake_move(&mov);
        }

        self.ply -= 1;
        self.side_to_move.switch();
    }

    pub fn make_move(&mut self, mov: &Move) {
        match mov.side_to_move {
            Color::Black => {
                // Move black mover
                self.black.unset(mov.from);
                self.black.set(mov.to);

                // If capture, remove captured piece
                if let Some(capture_index) = mov.capture {
                    self.white.unset(capture_index);
                    // Remove king if captured piece was king
                    if mov.is_king_capture {
                        self.white_kings.unset(capture_index);
                    }
                }

                // If promotion, make piece king. If king move, update king position
                if mov.is_king_move {
                    self.black_kings.unset(mov.from);
                    self.black_kings.set(mov.to);
                } else if mov.is_promotion {
                    self.black_kings.set(mov.to);
                }
            }
            Color::White => {
                // Move white mover
                self.white.unset(mov.from);
                self.white.set(mov.to);

                // If capture, remove captured piece
                if let Some(capture_index) = mov.capture {
                    self.black.unset(capture_index);
                    // Remove king if captured piece was king
                    if mov.is_king_capture {
                        self.black_kings.unset(capture_index);
                    }
                }

                // If promotion, make piece king. If king move, update king position
                if mov.is_king_move {
                    self.white_kings.unset(mov.from);
                    self.white_kings.set(mov.to);
                } else if mov.is_promotion {
                    self.white_kings.set(mov.to);
                }
            }
        }
    }

    pub fn unmake_move(&mut self, mov: &Move) {
        match mov.side_to_move {
            Color::Black => {
                // Placer pieces into their original positions
                self.black.unset(mov.to);
                self.black.set(mov.from);

                // If capture, place captured piece back
                if let Some(capture_index) = mov.capture {
                    self.white.set(capture_index);
                    // If captured piece was king, place king back
                    if mov.is_king_capture {
                        self.white_kings.set(capture_index);
                    }
                }

                // If promotion, remove king. If king move, update king position
                if mov.is_king_move {
                    self.black_kings.unset(mov.to);
                    self.black_kings.set(mov.from);
                } else if mov.is_promotion {
                    self.black_kings.unset(mov.to);
                }
            }
            Color::White => {
                // Placer pieces into their original positions
                self.white.unset(mov.to);
                self.white.set(mov.from);

                // If capture, place captured piece back
                if let Some(capture_index) = mov.capture {
                    self.black.set(capture_index);
                    // If captured piece was king, place king back
                    if mov.is_king_capture {
                        self.black_kings.set(capture_index);
                    }
                }

                // If promotion, remove king. If king move, update king position
                if mov.is_king_move {
                    self.white_kings.unset(mov.to);
                    self.white_kings.set(mov.from);
                } else if mov.is_promotion {
                    self.white_kings.unset(mov.to);
                }
            }
        }
    }
}

/// Testing and validation
impl Game {
    /// Perft (performance test) is a function that counts the number of legal moves
    pub fn perft(&mut self, depth: usize) -> usize {
        if depth <= 0 {
            return 1;
        }

        let mut nodes = 0;
        let move_sequences = self.generate_move_sequences();

        for ms in move_sequences.into_iter() {
            self.make_move_sequence(&ms);
            nodes += self.perft(depth - 1);
            self.unmake_move_sequence();
        }

        nodes
    }

    pub fn divide(&mut self, depth: usize) {
        let move_sequences = self.generate_move_sequences();
        let mut total_nodes = 0;

        for ms in move_sequences.into_iter() {
            self.make_move_sequence(&ms);
            let nodes = self.perft(depth - 1);
            self.unmake_move_sequence();

            total_nodes += nodes;
            println!("{}: {}", ms.to_string(), nodes);
        }

        println!("Total: {}", total_nodes);
    }
}

/// Move generation implementation
impl Game {
    pub fn generate_move_sequences(&mut self) -> Vec<MoveSequence> {
        match self.side_to_move {
            Color::Black => self.generate_black_move_sequences(),
            Color::White => self.generate_white_move_sequences(),
        }
    }

    pub fn generate_capture_move_sequences(&mut self) -> Vec<MoveSequence> {
        match self.side_to_move {
            Color::Black => self.generate_black_capture_sequences(&Bitboard::ALL),
            Color::White => self.generate_white_capture_sequences(&Bitboard::ALL),
        }
    }

    pub fn generate_black_move_sequences(&mut self) -> Vec<MoveSequence> {
        let capture_moves_sequences = self.generate_black_capture_sequences(&Bitboard::ALL);
        if capture_moves_sequences.len() > 0 {
            return capture_moves_sequences;
        }

        self.generate_black_sliding_moves()
    }

    pub fn generate_black_capture_sequences(&mut self, mask: &Bitboard) -> Vec<MoveSequence> {
        let (left_forward, right_forward, left_backward, right_backward) =
            self.generate_black_jumps(mask);
        let mut move_sequences = Vec::new();

        for from in left_forward.into_iter() {
            let to = from + 10;
            let capture = from + 5;
            let is_king_move = self.black_kings.get(from);
            let is_king_capture = self.white_kings.get(capture);
            let is_promotion = to >= 41 && !is_king_move;

            let mov = Move::new(
                Color::Black,
                from,
                to,
                Some(capture),
                is_king_move,
                is_king_capture,
                is_promotion,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_black_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in right_forward.into_iter() {
            let to = from + 8;
            let capture = from + 4;
            let is_king_move = self.black_kings.get(from);
            let is_king_capture = self.white_kings.get(capture);
            let is_promotion = to >= 41 && !is_king_move;

            let mov = Move::new(
                Color::Black,
                from,
                to,
                Some(capture),
                is_king_move,
                is_king_capture,
                is_promotion,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_black_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in left_backward.into_iter() {
            let to = from - 10;
            let capture = from - 5;
            let is_king_capture = self.white_kings.get(capture);

            let mov = Move::new(
                Color::Black,
                from,
                to,
                Some(capture),
                true,
                is_king_capture,
                false,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_black_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in right_backward.into_iter() {
            let to = from - 8;
            let capture = from - 4;
            let is_king_capture = self.white_kings.get(capture);

            let mov = Move::new(
                Color::Black,
                from,
                to,
                Some(capture),
                true,
                is_king_capture,
                false,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_black_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        move_sequences
    }

    pub fn generate_black_jumps(
        &self,
        mask: &Bitboard,
    ) -> (Bitboard, Bitboard, Bitboard, Bitboard) {
        let not_occupied = self.not_occupied();
        let black = self.black & *mask;
        let white = self.white;
        let black_kings = self.black_kings & *mask;

        let mut left_forward = (((not_occupied >> 5) & white) >> 5) & black;
        let mut right_forward = (((not_occupied >> 4) & white) >> 4) & black;
        let mut left_backward = (((not_occupied << 5) & white) << 5) & black_kings;
        let mut right_backward = (((not_occupied << 4) & white) << 4) & black_kings;

        (left_forward, right_forward, left_backward, right_backward)
    }

    pub fn generate_black_sliding_moves(&self) -> Vec<MoveSequence> {
        let mut move_sequences = Vec::new();

        let (left_forward, right_forward, left_backward, right_backward) =
            self.generate_black_slides();

        for from in left_forward.into_iter() {
            let to = from + 5;
            let is_king_move = self.black_kings.get(from);
            let is_promotion = to >= 41 && !is_king_move;

            let mov = Move::new(
                Color::Black,
                from,
                to,
                None,
                is_king_move,
                false,
                is_promotion,
            );
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in right_forward.into_iter() {
            let to = from + 4;
            let is_king_move = self.black_kings.get(from);
            let is_promotion = to >= 41 && !is_king_move;

            let mov = Move::new(
                Color::Black,
                from,
                to,
                None,
                is_king_move,
                false,
                is_promotion,
            );
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in left_backward.into_iter() {
            let to = from - 5;

            let mov = Move::new(Color::Black, from, to, None, true, false, false);
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in right_backward.into_iter() {
            let to = from - 4;

            let mov = Move::new(Color::Black, from, to, None, true, false, false);
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        move_sequences
    }

    pub fn generate_black_slides(&self) -> (Bitboard, Bitboard, Bitboard, Bitboard) {
        let not_occupied = self.not_occupied();
        let black = self.black;
        let black_kings = self.black_kings;

        let mut left_forward = (not_occupied >> 5) & black;
        let mut right_forward = (not_occupied >> 4) & black;
        let mut left_backward = (not_occupied << 5) & black_kings;
        let mut right_backward = (not_occupied << 4) & black_kings;

        (left_forward, right_forward, left_backward, right_backward)
    }

    pub fn generate_white_move_sequences(&mut self) -> Vec<MoveSequence> {
        let capture_moves_sequences = self.generate_white_capture_sequences(&Bitboard::ALL);
        if capture_moves_sequences.len() > 0 {
            return capture_moves_sequences;
        }

        self.generate_white_sliding_moves()
    }

    pub fn generate_white_capture_sequences(&mut self, mask: &Bitboard) -> Vec<MoveSequence> {
        let (left_forward, right_forward, left_backward, right_backward) =
            self.generate_white_jumps(mask);
        let mut move_sequences = Vec::new();

        for from in left_forward.into_iter() {
            let to = from - 10;
            let capture = from - 5;
            let is_king_move = self.white_kings.get(from);
            let is_king_capture = self.black_kings.get(capture);
            let is_promotion = to <= 13 && !is_king_move;

            let mov = Move::new(
                Color::White,
                from,
                to,
                Some(capture),
                is_king_move,
                is_king_capture,
                is_promotion,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_white_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in right_forward.into_iter() {
            let to = from - 8;
            let capture = from - 4;
            let is_king_move = self.white_kings.get(from);
            let is_king_capture = self.black_kings.get(capture);
            let is_promotion = to <= 13 && !is_king_move;

            let mov = Move::new(
                Color::White,
                from,
                to,
                Some(capture),
                is_king_move,
                is_king_capture,
                is_promotion,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_white_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in left_backward.into_iter() {
            let to = from + 10;
            let capture = from + 5;
            let is_king_capture = self.black_kings.get(capture);

            let mov = Move::new(
                Color::White,
                from,
                to,
                Some(capture),
                true,
                is_king_capture,
                false,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_white_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        for from in right_backward.into_iter() {
            let to = from + 8;
            let capture = from + 4;
            let is_king_capture = self.black_kings.get(capture);

            let mov = Move::new(
                Color::White,
                from,
                to,
                Some(capture),
                true,
                is_king_capture,
                false,
            );

            let mut move_sequence = MoveSequence::new(vec![mov.clone()]);
            if mov.is_promotion {
                move_sequences.push(move_sequence);
                continue;
            }

            // Look if further captures are possible from this position
            // We need to make move and unmake move after
            self.make_move(&mov);
            let sub_sequences =
                self.generate_white_capture_sequences(&Bitboard::create_one_hot(to));
            if sub_sequences.len() == 0 {
                move_sequences.push(move_sequence);
            } else {
                sub_sequences.into_iter().for_each(|mut seq: MoveSequence| {
                    move_sequences.push(move_sequence.extended(&mut seq));
                });
            }
            self.unmake_move(&mov);
        }

        move_sequences
    }

    pub fn generate_white_jumps(
        &self,
        mask: &Bitboard,
    ) -> (Bitboard, Bitboard, Bitboard, Bitboard) {
        let not_occupied = self.not_occupied();
        let white = self.white & *mask;
        let black = self.black;
        let white_kings = self.white_kings & *mask;

        let mut left_forward = (((not_occupied << 5) & black) << 5) & white;
        let mut right_forward = (((not_occupied << 4) & black) << 4) & white;
        let mut left_backward = (((not_occupied >> 5) & black) >> 5) & white_kings;
        let mut right_backward = (((not_occupied >> 4) & black) >> 4) & white_kings;

        (left_forward, right_forward, left_backward, right_backward)
    }

    pub fn generate_white_sliding_moves(&self) -> Vec<MoveSequence> {
        let mut move_sequences = Vec::new();

        let (left_forward, right_forward, left_backward, right_backward) =
            self.generate_white_slides();

        for from in left_forward.into_iter() {
            let to = from - 5;
            let is_king_move = self.white_kings.get(from);
            let is_promotion = to <= 13 && !is_king_move;

            let mov = Move::new(
                Color::White,
                from,
                to,
                None,
                is_king_move,
                false,
                is_promotion,
            );
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in right_forward.into_iter() {
            let to = from - 4;
            let is_king_move = self.white_kings.get(from);
            let is_promotion = to <= 13 && !is_king_move;

            let mov = Move::new(
                Color::White,
                from,
                to,
                None,
                is_king_move,
                false,
                is_promotion,
            );
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in left_backward.into_iter() {
            let to = from + 5;

            let mov = Move::new(Color::White, from, to, None, true, false, false);
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        for from in right_backward.into_iter() {
            let to = from + 4;

            let mov = Move::new(Color::White, from, to, None, true, false, false);
            let move_sequence = MoveSequence::new(vec![mov]);
            move_sequences.push(move_sequence);
        }

        move_sequences
    }

    pub fn generate_white_slides(&self) -> (Bitboard, Bitboard, Bitboard, Bitboard) {
        let not_occupied = self.not_occupied();
        let white = self.white;
        let white_kings = self.white_kings;

        let mut left_forward = (not_occupied << 5) & white;
        let mut right_forward = (not_occupied << 4) & white;
        let mut left_backward = (not_occupied >> 5) & white_kings;
        let mut right_backward = (not_occupied >> 4) & white_kings;

        (left_forward, right_forward, left_backward, right_backward)
    }
}

/// Perft tests up to depth 12
///
/// see: [https://aartbik.blogspot.com/2009/02/perft-for-checkers.html]
#[cfg(test)]
pub mod perft_tests {
    use super::*;

    #[test]
    fn perft_0() {
        let mut game = Game::new();
        assert_eq!(game.perft(0), 1);
    }

    #[test]
    fn perft_1() {
        let mut game = Game::new();
        assert_eq!(game.perft(1), 7);
    }

    #[test]
    fn perft_2() {
        let mut game = Game::new();
        assert_eq!(game.perft(2), 49);
    }

    #[test]
    fn perft_3() {
        let mut game = Game::new();
        assert_eq!(game.perft(3), 302);
    }

    #[test]
    fn perft_4() {
        let mut game = Game::new();
        assert_eq!(game.perft(4), 1469);
    }

    #[test]
    fn perft_5() {
        let mut game = Game::new();
        assert_eq!(game.perft(5), 7361);
    }

    #[test]
    fn perft_6() {
        let mut game = Game::new();
        assert_eq!(game.perft(6), 36_768);
    }

    #[test]
    fn perft_7() {
        let mut game = Game::new();
        assert_eq!(game.perft(7), 179_740);
    }

    #[test]
    fn perft_8() {
        let mut game = Game::new();
        assert_eq!(game.perft(8), 845_931);
    }

    #[test]
    fn perft_9() {
        let mut game = Game::new();
        assert_eq!(game.perft(9), 3_963_680);
    }

    #[test]
    fn perft_10() {
        let mut game = Game::new();
        assert_eq!(game.perft(10), 18_391_564);
    }

    #[test]
    fn perft_11() {
        let mut game = Game::new();
        assert_eq!(game.perft(11), 85_242_128);
    }

    #[test]
    fn perft_12() {
        let mut game = Game::new();
        assert_eq!(game.perft(12), 388_623_673);
    }
}
