use std::{collections::HashMap, vec};

use super::game::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranspositionTableFlag {
    Exact,
    LowerBound,
    UpperBound,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TranspositionTableEntry {
    pub key: u64,
    pub best_move_sequence: MoveSequence,
    pub score: i32,
    pub depth: usize,
    pub flag: TranspositionTableFlag,
}

impl TranspositionTableEntry {
    pub fn create(
        transposition_table: &TranspositionTable,
        game: &Game,
        best_move_sequence: MoveSequence,
        score: i32,
        depth: usize,
        flag: TranspositionTableFlag,
    ) -> Self {
        TranspositionTableEntry::create_with_key(
            transposition_table.hash(game),
            best_move_sequence,
            score,
            depth,
            flag,
        )
    }

    pub fn create_with_key(
        key: u64,
        best_move_sequence: MoveSequence,
        score: i32,
        depth: usize,
        flag: TranspositionTableFlag,
    ) -> Self {
        TranspositionTableEntry {
            key,
            best_move_sequence,
            score,
            depth,
            flag,
        }
    }

    pub fn create_empty_with_key(key: u64) -> Self {
        TranspositionTableEntry {
            key,
            best_move_sequence: MoveSequence::new(vec![]),
            score: Score::DRAW,
            depth: 0,
            flag: TranspositionTableFlag::Unknown,
        }
    }
}

#[derive(Debug)]
struct TranspositionTableHashMap([u64; 64]);

impl TranspositionTableHashMap {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut array = [0; 64];

        for i in 0..64 {
            array[i] = rng.gen();
        }

        TranspositionTableHashMap(array)
    }
}

impl Default for TranspositionTableHashMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct TranspositionTable {
    table_size: usize,
    table: Vec<Option<TranspositionTableEntry>>,

    white_hashmap: TranspositionTableHashMap,
    black_hashmap: TranspositionTableHashMap,
    white_kings_hashmap: TranspositionTableHashMap,
    black_kings_hashmap: TranspositionTableHashMap,
    side_hash: u64,
    // draw_hash: u64,
}

impl TranspositionTable {
    pub fn new(table_size: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let side_hash = rng.gen();
        // let draw_hash = rng.gen();

        let white_hashmap = TranspositionTableHashMap::default();
        let black_hashmap = TranspositionTableHashMap::default();
        let white_kings_hashmap = TranspositionTableHashMap::default();
        let black_kings_hashmap = TranspositionTableHashMap::default();

        let mut tt = TranspositionTable {
            table_size,
            table: vec![None; table_size],
            white_hashmap,
            black_hashmap,
            white_kings_hashmap,
            black_kings_hashmap,
            side_hash,
        };

        // We should check for type 1 errors in the hashmaps.
        // Type 1 errors are when two different indices in the hashmap have the same value.
        // This is a problem because it means that the hash function is not injective.
        // This means that the hash function is not one-to-one, which means that it's not
        // possible to get the original value from the hash.

        while !tt.assert_no_collisions() {
            let white_hashmap = TranspositionTableHashMap::default();
            let black_hashmap = TranspositionTableHashMap::default();
            let white_kings_hashmap = TranspositionTableHashMap::default();
            let black_kings_hashmap = TranspositionTableHashMap::default();

            tt = TranspositionTable {
                table_size,
                table: vec![None; table_size],
                white_hashmap,
                black_hashmap,
                white_kings_hashmap,
                black_kings_hashmap,
                side_hash,
            };
        }

        tt
    }
}

/// Insertion and retrieval of entries in the transposition table.
impl TranspositionTable {
    pub fn insert(&mut self, entry: TranspositionTableEntry) {
        // Before insertion we should check wether it's worth overwriting the entry.
        // If the entry is deeper than the one we're trying to insert, we should
        // not overwrite it.
        if let Some(existing_entry) = self.fetch(entry.key) {
            if existing_entry.depth > entry.depth {
                return;
            }
        }

        let index = (entry.key % self.table_size as u64) as usize;

        self.table[index] = Some(entry);
    }

    pub fn fetch(&self, key: u64) -> Option<&TranspositionTableEntry> {
        let index = (key % self.table_size as u64) as usize;

        if let Some(entry) = &self.table[index] {
            if entry.key == key {
                return Some(entry);
            } else {
                return None;
            }
        }

        None
    }

    pub fn get_principal_variation_line(&self, key: u64) -> Vec<MoveSequence> {
        let mut line = vec![];

        let mut current_key = key;

        while let Some(entry) = self.fetch(current_key) {
            if line
                .iter()
                .any(|line_entry: &TranspositionTableEntry| line_entry.key == entry.key)
            {
                break;
            }

            line.push(entry.clone());

            current_key = self.hash_move_sequence(current_key, &entry.best_move_sequence, true);
        }

        line.iter()
            .map(|entry| entry.best_move_sequence.clone())
            .collect()
    }
}

/// Hashing of game states.
impl TranspositionTable {
    pub fn hash(&self, game: &Game) -> u64 {
        let mut hash = 0;

        for index in game.white.into_iter() {
            hash ^= self.white_hashmap.0[index];
        }

        for index in game.black.into_iter() {
            hash ^= self.black_hashmap.0[index];
        }

        for index in game.white_kings.into_iter() {
            hash ^= self.white_kings_hashmap.0[index];
        }

        for index in game.black_kings.into_iter() {
            hash ^= self.black_kings_hashmap.0[index];
        }

        match game.side_to_move {
            Color::White => hash ^= self.side_hash,
            Color::Black => (),
        }

        hash
    }

    pub fn hash_move_sequence(
        &self,
        key: u64,
        move_sequence: &MoveSequence,
        is_side_switch: bool,
    ) -> u64 {
        let mut hash = key;

        for mov in move_sequence.clone() {
            match mov.side_to_move {
                Color::White => {
                    hash ^= self.white_hashmap.0[mov.from];
                    hash ^= self.white_hashmap.0[mov.to];

                    if mov.is_king_move {
                        hash ^= self.white_kings_hashmap.0[mov.from];
                        hash ^= self.white_kings_hashmap.0[mov.to];
                    }

                    if let Some(capture) = mov.capture {
                        hash ^= self.black_hashmap.0[capture];

                        if mov.is_king_capture {
                            hash ^= self.black_kings_hashmap.0[capture];
                        }
                    }

                    if mov.is_promotion {
                        hash ^= self.white_kings_hashmap.0[mov.to];
                    }
                }
                Color::Black => {
                    hash ^= self.black_hashmap.0[mov.from];
                    hash ^= self.black_hashmap.0[mov.to];

                    if mov.is_king_move {
                        hash ^= self.black_kings_hashmap.0[mov.from];
                        hash ^= self.black_kings_hashmap.0[mov.to];
                    }

                    if let Some(capture) = mov.capture {
                        hash ^= self.white_hashmap.0[capture];

                        if mov.is_king_capture {
                            hash ^= self.white_kings_hashmap.0[capture];
                        }
                    }

                    if mov.is_promotion {
                        hash ^= self.black_kings_hashmap.0[mov.to];
                    }
                }
            }
        }

        if is_side_switch {
            hash ^= self.side_hash;
        }

        // assert!(hash != key);

        hash
    }
}

impl TranspositionTable {
    fn assert_no_collisions(&self) -> bool {
        let mut key_map = HashMap::new();

        for key in self.white_hashmap.0.iter() {
            if key_map.contains_key(key) {
                return false;
            } else {
                key_map.insert(key, true);
            }
        }

        for key in self.black_hashmap.0.iter() {
            if key_map.contains_key(key) {
                return false;
            } else {
                key_map.insert(key, true);
            }
        }

        for key in self.white_kings_hashmap.0.iter() {
            if key_map.contains_key(key) {
                return false;
            } else {
                key_map.insert(key, true);
            }
        }

        for key in self.black_kings_hashmap.0.iter() {
            if key_map.contains_key(key) {
                return false;
            } else {
                key_map.insert(key, true);
            }
        }

        if key_map.contains_key(&self.side_hash) {
            return false;
        }

        true
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        // 2^20 + 7 to make it a prime number
        // Will result in a table size of 2^20 * 48 bytes = 50.3 MB
        Self::new(1_048_583)
    }
}
