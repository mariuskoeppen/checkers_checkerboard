#![allow(unused)]

use std::time::Duration;

use super::*;
use crate::transposition_table::TranspositionTable;

const CHECK_EVERY_N_NODES: usize = 2_047;

#[derive(Debug)]
pub struct Engine {
    pub color: Color,
    pub current_depth: usize,
    pub max_time: Duration,
    start_time: std::time::Instant,
    stopped_searching: bool,
    pub best_move: Option<MoveSequence>,
    pub best_score: i32,
    pub searched_nodes: usize,
    transposition_table: TranspositionTable,
}

impl Engine {
    pub fn new(color: Color, max_time: Duration) -> Self {
        Engine {
            color,
            current_depth: 0,
            max_time,
            start_time: std::time::Instant::now(),
            stopped_searching: false,
            best_move: None,
            best_score: Score::DRAW,
            searched_nodes: 0,
            transposition_table: TranspositionTable::default(),
        }
    }
}

impl Engine {
    pub async fn find_best_move(&mut self, game: &mut Game) -> (Option<MoveSequence>, i32, String) {
        if game.side_to_move != self.color {
            panic!("Engine is not playing as the side to move.");
        }

        // Iterative deepening.
        self.start_time = std::time::Instant::now();
        self.searched_nodes = 0;
        self.current_depth = 0;
        self.stopped_searching = false;

        loop {
            self.current_depth += 1;

            self.search_root(game, self.current_depth, -Score::INFINITY, Score::INFINITY);

            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            // We might also need to check if the time left is greater than 2* the time it took to
            // search the previous depth
            if elapsed_time >= self.max_time || self.current_depth >= 32 || self.stopped_searching {
                break;
            }
        }

        let principal_variation_line = self
            .transposition_table
            .get_principal_variation_line(self.transposition_table.hash(game))
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        (
            self.best_move.clone(),
            match self.color {
                Color::Black => -self.best_score,
                Color::White => self.best_score,
            },
            principal_variation_line,
        )
    }
}

impl Engine {
    fn search_root(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
        if game.is_black_win() {
            return match game.side_to_move {
                Color::Black => Score::WIN - game.ply as i32,
                Color::White => -Score::WIN + game.ply as i32,
            };
        } else if game.is_white_win() {
            return match game.side_to_move {
                Color::Black => -Score::WIN + game.ply as i32,
                Color::White => Score::WIN - game.ply as i32,
            };
        } else if game.is_draw() {
            return Score::DRAW;
        }

        let original_alpha = alpha;
        let current_hash = self.transposition_table.hash(game);
        let mut principal_variation_move = None;

        // if let Some(transposition_table_entry) = self.transposition_table.fetch(current_hash) {
        //     // draft ::= depth at the root - ply index
        //     if transposition_table_entry.depth >= depth {
        //         match transposition_table_entry.flag {
        //             TranspositionTableFlag::Exact => {
        //                 self.best_score = transposition_table_entry.score;
        //                 self.best_move = Some(transposition_table_entry.best_move_sequence.clone());

        //                 return transposition_table_entry.score;
        //             }
        //             TranspositionTableFlag::LowerBound => {
        //                 alpha = alpha.max(transposition_table_entry.score)
        //             }
        //             TranspositionTableFlag::UpperBound => {
        //                 beta = beta.min(transposition_table_entry.score)
        //             }
        //             _ => panic!("should not have unknown flag in transposition table"),
        //         }

        //         if alpha >= beta {
        //             self.best_score = transposition_table_entry.score;
        //             self.best_move = Some(transposition_table_entry.best_move_sequence.clone());

        //             return transposition_table_entry.score;
        //         }
        //     }

        //     principal_variation_move = Some(transposition_table_entry.best_move_sequence.clone());
        // }

        let mut best_score = -Score::INFINITY;
        let mut best_move = None;
        let mut available_moves = game.generate_move_sequences();
        Engine::order_moves(&mut available_moves, &principal_variation_move);

        for m in available_moves {
            game.make_move_sequence(&m);
            let score = -self.search(game, depth - 1, -beta, -alpha);
            game.unmake_move_sequence();

            if self.stopped_searching {
                return Score::DRAW;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(m);

                if score > alpha {
                    if score >= beta {
                        // Might want to store this in the transposition table
                        // Also might want to store killer moves
                        /*  */
                        best_score = beta;
                        break;
                    }

                    alpha = score;
                }
            }
        }

        if !self.stopped_searching {
            // let transposition_table_entry = TranspositionTableEntry::create_with_key(
            //     current_hash,
            //     best_move.clone().unwrap(),
            //     best_score,
            //     depth,
            //     if best_score <= original_alpha {
            //         TranspositionTableFlag::UpperBound
            //     } else if best_score >= beta {
            //         TranspositionTableFlag::LowerBound
            //     } else {
            //         TranspositionTableFlag::Exact
            //     },
            // );

            // self.transposition_table.insert(transposition_table_entry);

            self.best_move = best_move;
            self.best_score = best_score;
        }

        // if self.best_move.is_none() {
        //     panic!(
        //         "{:?}, stopped: {} ",
        //         game.generate_move_sequences(),
        //         self.stopped_searching
        //     );
        //     self.best_move = Some(game.generate_move_sequences()[0].clone());
        // }

        best_score
    }

    fn search(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
        self.searched_nodes += 1;

        if self.searched_nodes % CHECK_EVERY_N_NODES == 0 {
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            if elapsed_time >= self.max_time {
                self.stopped_searching = true;
                return Score::DRAW;
            }
        }

        if depth == 0 {
            return self.quiescence_search(game, alpha, beta);
        }

        if game.is_black_win() {
            return match game.side_to_move {
                Color::Black => Score::WIN - game.ply as i32,
                Color::White => -Score::WIN + game.ply as i32,
            };
        } else if game.is_white_win() {
            return match game.side_to_move {
                Color::Black => -Score::WIN + game.ply as i32,
                Color::White => Score::WIN - game.ply as i32,
            };
        } else if game.is_draw() {
            return Score::DRAW;
        }

        let original_alpha = alpha;
        let current_hash = self.transposition_table.hash(game);
        let mut principal_variation_move = None;

        // if let Some(transposition_table_entry) = self.transposition_table.fetch(current_hash) {
        //     if transposition_table_entry.depth >= depth {
        //         match transposition_table_entry.flag {
        //             TranspositionTableFlag::Exact => {
        //                 return transposition_table_entry.score;
        //             }
        //             TranspositionTableFlag::LowerBound => {
        //                 alpha = alpha.max(transposition_table_entry.score)
        //             }
        //             TranspositionTableFlag::UpperBound => {
        //                 beta = beta.min(transposition_table_entry.score)
        //             }
        //             _ => panic!("should not have unknown flag in transposition table"),
        //         }

        //         if alpha >= beta {
        //             return transposition_table_entry.score;
        //         }
        //     }

        //     principal_variation_move = Some(transposition_table_entry.best_move_sequence.clone());
        // }

        let mut best_score = -Score::INFINITY;
        let mut best_move = None;
        let mut available_moves = game.generate_move_sequences();
        Engine::order_moves(&mut available_moves, &principal_variation_move);

        for m in available_moves {
            game.make_move_sequence(&m);
            let score = -self.search(game, depth - 1, -beta, -alpha);
            game.unmake_move_sequence();

            if self.stopped_searching {
                return Score::DRAW;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(m);

                if score > alpha {
                    if score >= beta {
                        // Might want to store this in the transposition table
                        // Also might want to store killer moves
                        /*  */
                        best_score = beta;
                        break;
                    }

                    alpha = score;
                }
            }
        }

        // if best_move.is_none() {
        //     println!("side to move: {:?}, depht: {}, best score: {}, alpha: {}, beta: {}, original alpha: {} ",game.side_to_move,  depth, best_score, alpha, beta, original_alpha);
        // }

        // if !self.stopped_searching {
        //     let transposition_table_entry = TranspositionTableEntry::create_with_key(
        //         current_hash,
        //         best_move.unwrap(),
        //         best_score,
        //         depth,
        //         if best_score <= original_alpha {
        //             TranspositionTableFlag::UpperBound
        //         } else if best_score >= beta {
        //             TranspositionTableFlag::LowerBound
        //         } else {
        //             TranspositionTableFlag::Exact
        //         },
        //     );

        //     self.transposition_table.insert(transposition_table_entry);
        // }

        best_score
    }

    fn quiescence_search(&mut self, game: &mut Game, mut alpha: i32, beta: i32) -> i32 {
        self.searched_nodes += 1;

        if self.searched_nodes % CHECK_EVERY_N_NODES == 0 {
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - self.start_time;

            if elapsed_time >= self.max_time {
                self.stopped_searching = true;
                return Score::DRAW;
            }
        }

        if game.is_black_win() {
            return match game.side_to_move {
                Color::Black => Score::WIN - game.ply as i32,
                Color::White => -Score::WIN + game.ply as i32,
            };
        } else if game.is_white_win() {
            return match game.side_to_move {
                Color::Black => -Score::WIN + game.ply as i32,
                Color::White => Score::WIN - game.ply as i32,
            };
        } else if game.is_draw() {
            return Score::DRAW;
        }

        let standing_pat = self.evaluate(game);

        if standing_pat >= beta {
            return beta;
        }

        if alpha < standing_pat {
            alpha = standing_pat;
        }

        let mut available_moves = game.generate_capture_move_sequences();
        Engine::order_moves(&mut available_moves, &None);

        for m in available_moves {
            game.make_move_sequence(&m);
            let score = -self.quiescence_search(game, -beta, -alpha);
            game.unmake_move_sequence();

            if self.stopped_searching {
                return Score::DRAW;
            }

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}

/// Evaluation.
impl Engine {
    /// Evaluate the current position from the perspective of white.
    /// White has generally speaking a positive score while black has a negative score.
    /// Returns a positive score if white is winning, and a negative score if black is winning.
    ///
    /// The score is calculated as follows:
    /// - Material: 100 for every man
    /// - Kings: 141 for every king
    /// - positional advantages
    pub fn evaluate(&mut self, game: &Game) -> i32 {
        // Score increases as white is winning, and decreases as black is winning.
        let mut score = Score::DRAW; /* score = 0 */

        // Material
        score += 1000 * (game.white.count() as i32 - game.black.count() as i32);

        // Kings
        score += 410 * (game.white_kings.count() as i32 - game.black_kings.count() as i32);

        // Men advantages
        score += 10
            * ((game.white & Engine::WHITE_MEN_LIGHT).count() as i32
                - (game.black & Engine::BLACK_MEN_LIGHT).count() as i32);
        score += 20
            * ((game.white & Engine::WHITE_MEN_MID).count() as i32
                - (game.black & Engine::BLACK_MEN_MID).count() as i32);
        score += 30
            * ((game.white & Engine::WHITE_MEN_STRONG).count() as i32
                - (game.black & Engine::BLACK_MEN_STRONG).count() as i32);

        // Kings advantages
        score += 20
            * ((game.white_kings & Engine::WHITE_KINGS_LIGHT).count() as i32
                - (game.black_kings & Engine::BLACK_KINGS_LIGHT).count() as i32);
        score += 30
            * ((game.white_kings & Engine::WHITE_KINGS_MID).count() as i32
                - (game.black_kings & Engine::BLACK_KINGS_MID).count() as i32);
        score += 40
            * ((game.white_kings & Engine::WHITE_KINGS_STRONG).count() as i32
                - (game.black_kings & Engine::BLACK_KINGS_STRONG).count() as i32);

        // // Mobility
        // // since we're only evaluating quiet moves, we can consider the mobility of sliding pieces
        // match game.side_to_move {
        //     Color::White => {
        //         let (lf, rf, lb, rb) = game.generate_white_slides();
        //         score += (lf | rf | lb | rb).count() as i32;
        //     }
        //     Color::Black => {
        //         let (lf, rf, lb, rb) = game.generate_black_slides();
        //         score -= (lf | rf | lb | rb).count() as i32;
        //     }
        // }

        match game.side_to_move {
            Color::White => {
                return score;
            }
            Color::Black => {
                return -score;
            }
        }
    }
}

impl Engine {
    pub const BLACK_MEN_LIGHT: Bitboard = Bitboard(0xAE04285000);
    pub const BLACK_MEN_MID: Bitboard = Bitboard(0x41D3D02C00);
    pub const BLACK_MEN_STRONG: Bitboard = Bitboard(0x10020000000);

    pub const BLACK_KINGS_LIGHT: Bitboard = Bitboard(0xE8140B8000);
    pub const BLACK_KINGS_MID: Bitboard = Bitboard(0x641300000);
    pub const BLACK_KINGS_STRONG: Bitboard = Bitboard(0x22000000);

    pub const WHITE_MEN_LIGHT: Bitboard = Bitboard(0x50A103A8000);
    pub const WHITE_MEN_MID: Bitboard = Bitboard(0x1A05E5C10000);
    pub const WHITE_MEN_STRONG: Bitboard = Bitboard(0x2004000);

    pub const WHITE_KINGS_LIGHT: Bitboard = Engine::BLACK_KINGS_LIGHT;
    pub const WHITE_KINGS_MID: Bitboard = Engine::BLACK_KINGS_MID;
    pub const WHITE_KINGS_STRONG: Bitboard = Engine::BLACK_KINGS_STRONG;
}

impl Engine {
    /// Order moves to get better alpha beta pruning.
    /// The principal variation move is moved to the front of the list.
    /// The rest of the moves are sorted by score, which factors in
    /// - captures
    /// - king captures
    /// - promotions
    fn order_moves(moves: &mut Vec<MoveSequence>, principal_variation_move: &Option<MoveSequence>) {
        // moves.sort_unstable_by(|a, b| {
        //     let a_score = a.score();
        //     let b_score = b.score();
        //     b_score.partial_cmp(&a_score).unwrap()
        // });

        // // Move the principal variation move to the front of the list.
        // // Might further improve this by instead of swapping, moving the principal variation move to the front of the list and filling the gap.
        // if let Some(principal_variation_move) = principal_variation_move {
        //     let index = moves
        //         .iter()
        //         .position(|m| m == principal_variation_move)
        //         .expect("principal variation move should be in list of moves");

        //     if index > 1 {
        //         let item = moves.remove(index);
        //         moves.insert(0, item);
        //     } else {
        //         moves.swap(0, index);
        //     }
        // }

        if let Some(principal_variation_move) = principal_variation_move {
            moves.sort_unstable_by(|a, b| {
                if a == principal_variation_move {
                    return std::cmp::Ordering::Less;
                } else if b == principal_variation_move {
                    return std::cmp::Ordering::Greater;
                }

                let a_score = a.score();
                let b_score = b.score();
                b_score.partial_cmp(&a_score).unwrap()
            });
        } else {
            moves.sort_unstable_by(|a, b| {
                let a_score = a.score();
                let b_score = b.score();
                b_score.partial_cmp(&a_score).unwrap()
            });
        }
        // if principal_variation_move.is_some() && moves.iter().any(|ms| ms.is_promotion()) {
        //     println!(
        //         "moves: {:?}, pv: {:?} ",
        //         moves,
        //         principal_variation_move.clone()
        //     );
        // }
    }
}
