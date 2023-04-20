use std::vec;

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
            score: 0,
            depth: 0,
            flag: TranspositionTableFlag::Unknown,
        }
    }
}

#[derive(Debug)]
struct TranspositionTableHashMap([u64; 64]);

impl TranspositionTableHashMap {
    fn new() -> Self {
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
}

impl TranspositionTable {
    pub fn new(table_size: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let side_hash = rng.gen();

        TranspositionTable {
            table_size,
            table: vec![None; table_size],
            white_hashmap: TranspositionTableHashMap::default(),
            black_hashmap: TranspositionTableHashMap::default(),
            white_kings_hashmap: TranspositionTableHashMap::default(),
            black_kings_hashmap: TranspositionTableHashMap::default(),
            side_hash,
        }
    }
}

/// Insertion and retrieval of entries in the transposition table.
impl TranspositionTable {
    pub fn insert(&mut self, entry: TranspositionTableEntry) {
        let index = (entry.key % self.table_size as u64) as usize;

        // Before insertion we should check wether it's worth overwriting the entry.
        // If the entry is deeper than the one we're trying to insert, we should
        // not overwrite it.
        if let Some(existing_entry) = &self.table[index] {
            if existing_entry.depth > entry.depth {
                return;
            }
        }

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
}

/// Hashing of game states.
impl TranspositionTable {
    pub fn hash(&self, game: &Game) -> u64 {
        let mut hash = 0;

        for index in game.white.clone().into_iter() {
            hash ^= self.white_hashmap.0[index];
        }

        for index in game.black.clone().into_iter() {
            hash ^= self.black_hashmap.0[index];
        }

        for index in game.white_kings.clone().into_iter() {
            hash ^= self.white_kings_hashmap.0[index];
        }

        for index in game.black_kings.clone().into_iter() {
            hash ^= self.black_kings_hashmap.0[index];
        }

        match game.side_to_move {
            Color::White => hash ^= self.side_hash,
            Color::Black => (),
        }

        hash
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(1_000_000)
    }
}
