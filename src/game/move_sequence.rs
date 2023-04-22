use crate::game::Color;
use crate::helpers::ImprovedPositionMapper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    /// The side that is making the move.
    // This is redundant, but it makes it easier to work with the move.
    // Might remove it later.
    pub side_to_move: Color,
    /// The index of the tile that the piece is moving from.
    pub from: usize,
    /// The index of the tile that the piece is moving to.
    pub to: usize,
    /// The index of the tile that the piece is capturing.
    pub capture: Option<usize>,
    /// Whether the move is a king move.
    pub is_king_move: bool,
    /// Whether the move is a king capture.
    pub is_king_capture: bool,
    /// Whether the move is a promotion.
    pub is_promotion: bool,
}

impl Move {
    pub fn new(
        side_to_move: Color,
        from: usize,
        to: usize,
        capture: Option<usize>,
        is_king_move: bool,
        is_king_capture: bool,
        is_promotion: bool,
    ) -> Self {
        Self {
            side_to_move,
            from,
            to,
            capture,
            is_king_move,
            is_king_capture,
            is_promotion,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveSequence(Vec<Move>);

impl Iterator for MoveSequence {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let el = self.0.remove(0);
        Some(el)
    }
}

impl DoubleEndedIterator for MoveSequence {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl MoveSequence {
    pub fn new(v: Vec<Move>) -> Self {
        Self(v)
    }

    pub fn extend(&mut self, other: &MoveSequence) {
        self.0.extend_from_slice(&other.0);
    }

    pub fn extended(&self, other: &MoveSequence) -> Self {
        let mut v = self.0.clone();
        v.extend_from_slice(&other.0);
        Self(v)
    }

    pub fn is_capture(&self) -> bool {
        self.0[0].capture.is_some()
    }

    pub fn is_promotion(&self) -> bool {
        self.0.last().unwrap().is_promotion
    }

    pub fn is_king_move(&self) -> bool {
        self.0[0].is_king_move
    }

    pub fn move_sequence_type(&self) -> MoveSequenceType {
        match self.is_capture() {
            true => MoveSequenceType::Jump,
            false => MoveSequenceType::Slide,
        }
    }

    pub fn indeces(&self) -> Vec<usize> {
        let mut indeces = Vec::new();
        let mut ms = self.0.iter();
        let first_move = ms.next().expect("there should be at least one move");
        indeces.push(first_move.from);
        indeces.push(first_move.to);
        for mov in ms {
            indeces.push(mov.to);
        }
        indeces
    }

    pub fn positions(&self) -> Vec<usize> {
        let indeces = self.indeces();
        indeces
            .iter()
            .map(|index| ImprovedPositionMapper::index_to_position[*index])
            .collect()
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;

        // King captures are better.
        for mov in &self.0 {
            if mov.is_king_capture {
                score += 1;
            }

            score += 2;
        }

        // Promotions are better.
        if self.is_promotion() {
            score += 1;
        }

        score
    }

    pub fn is_irreversible(&self) -> bool {
        !self.is_king_move() || self.is_capture()
    }
}

impl ToString for MoveSequence {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut moves = self.0.iter();
        let first_move = moves.next().expect("there should be at least one move");
        s.push_str(
            ImprovedPositionMapper::index_to_position[first_move.from]
                .to_string()
                .as_str(),
        );

        s.push_str(match first_move.capture {
            Some(_) => match first_move.is_king_capture {
                true => "X",
                false => "x",
            },
            None => "-",
        });

        s.push_str(
            ImprovedPositionMapper::index_to_position[first_move.to]
                .to_string()
                .as_str(),
        );

        while let Some(mov) = moves.next() {
            s.push_str(match mov.capture {
                Some(_) => match mov.is_king_capture {
                    true => "X",
                    false => "x",
                },
                None => "-",
            });

            s.push_str(
                ImprovedPositionMapper::index_to_position[mov.to]
                    .to_string()
                    .as_str(),
            );
        }

        if self.is_promotion() {
            s.push_str("#");
        }

        s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveSequenceType {
    Slide,
    Jump,
}
