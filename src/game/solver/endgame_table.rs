use std::{collections::HashMap, fs, str::FromStr};

use crate::game::Bitboard;

#[derive(Debug)]
pub struct EndgameTable {
    table: HashMap<String, EndgameTableFlag>,
}

/// constructor
impl EndgameTable {
    pub fn new() -> Self {
        EndgameTable {
            table: HashMap::new(),
        }
    }

    pub fn from_db(path: String) -> Result<Self, String> {
        // let mut table = HashMap::new();

        let contents = fs::read_to_string(path).expect("Should have been able to read the file");

        let table = HashMap::from_iter(contents.lines().into_iter().filter_map(|line| {
            if line.starts_with("BASE") {
                let line = &line[4..];
                let (identifier, expectation) =
                    line.split_once(" ").expect("number like 1041.66 and ++");

                if let Ok(flag) = expectation.parse() {
                    Some((identifier.to_string(), flag))
                } else {
                    None
                }
            } else {
                None
            }
        }));

        // println!("{:?}", table);
        // println!("{}", table.len());

        Ok(Self { table })
    }
}

impl EndgameTable {
    pub fn state_to_string(
        black: Bitboard,
        white: Bitboard,
        black_kings: Bitboard,
        white_kings: Bitboard,
    ) -> String {
        let mut s = String::new();

        // "3012.12"
        // The first four numbers determine the number of pieces
        // (bk, wk, bp, wp)
        s += &black_kings.count().to_string();
        s += &white_kings.count().to_string();
        s += &(black ^ black_kings).count().to_string();
        s += &(white ^ white_kings).count().to_string();

        s += ".";

        // The next 2 numbers tell you where the pieces sit
        // (black pieces rank, white pieces rank)

        s += black.to_rank().to_string().as_str();
        s += white.to_rank().to_string().as_str();

        s
    }
}

/// Fetch
impl EndgameTable {
    pub fn fetch(&self, position: String) -> Option<EndgameTableFlag> {
        self.table.get(&position)?.clone().into()
    }
}

impl Default for EndgameTable {
    fn default() -> Self {
        Self::from_db("./DB6/DB6.idx".to_string()).expect("no database installed")
    }
}

#[derive(Debug, Clone)]
pub enum EndgameTableFlag {
    Draw,
    BlackWin,
    WhiteWin,
    MostlyDraw,
    MostlyBlackWin,
    MostlyWhiteWin,
}

impl FromStr for EndgameTableFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "++" {
            return Ok(Self::BlackWin);
        } else if s == "--" {
            return Ok(Self::WhiteWin);
        } else if s == "==" {
            return Ok(Self::Draw);
        } else if s == "+" {
            return Ok(Self::MostlyBlackWin);
        } else if s == "-" {
            return Ok(Self::MostlyWhiteWin);
        } else if s == "=" {
            return Ok(Self::MostlyDraw);
        }

        Err("not valid".to_string())
    }
}
