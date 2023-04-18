pub fn get_tile_position(row_number: usize, col_number: usize) -> Option<usize> {
    if row_number > 7 || col_number > 7 {
        return None;
    }

    match row_number & 1 {
        0 => match col_number & 1 {
            1 => Some(((row_number * 8) + col_number) / 2 + 1),
            _ => None,
        },
        1 => match col_number & 1 {
            0 => Some(((row_number * 8) + col_number) / 2 + 1),
            _ => None,
        },
        _ => None,
    }
}

/// Translate positions into indices and the other way around.
/// A *position* is considered the official checkers game notaion, e.g. each dark square numbered 1-32.
/// An *index* is considered an internal representation off the checkers pieces going from 0-55.
#[derive(Debug)]
pub struct PositionMapper {
    /// Maps official checkers notation 1-32 to internal 0-55.
    pub position_to_index: [usize; 33],
    /// Maps internal 0-55 to checkers 1-32
    pub index_to_position: [usize; 55],
    /// Same as [index_to_position] but returns an Option
    pub index_to_position_opt: [Option<usize>; 55],
}

impl Default for PositionMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl PositionMapper {
    pub const fn new() -> Self {
        let position_to_index: [usize; 33] = [
            0, // There is no actual position "0"
            10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 28, 29, 30, 31, 32, 33,
            34, 35, 37, 38, 39, 40, 41, 42, 43, 44,
        ];
        let index_to_position: [usize; 55] = [
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            1, 2, 3, 4, 5, 6, 7, 8, 0, // ins
            9, 10, 11, 12, 13, 14, 15, 16, 0, // ins
            17, 18, 19, 20, 21, 22, 23, 24, 0, // ins
            25, 26, 27, 28, 29, 30, 31, 32, 0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
            0, // ins
        ];

        let index_to_position_opt: [Option<usize>; 55] = [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(01),
            Some(02),
            Some(03),
            Some(04),
            Some(05),
            Some(06),
            Some(07),
            Some(08),
            None,
            Some(09),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            None,
            Some(17),
            Some(18),
            Some(19),
            Some(20),
            Some(21),
            Some(22),
            Some(23),
            Some(24),
            None,
            Some(25),
            Some(26),
            Some(27),
            Some(28),
            Some(29),
            Some(30),
            Some(31),
            Some(32),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        PositionMapper {
            position_to_index,
            index_to_position,
            index_to_position_opt,
        }
    }
}

pub struct ImprovedPositionMapper;

impl ImprovedPositionMapper {
    pub const position_to_index: [usize; 33] = [
        0, // There is no actual position "0"
        10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 28, 29, 30, 31, 32, 33, 34,
        35, 37, 38, 39, 40, 41, 42, 43, 44,
    ];
    pub const index_to_position: [usize; 55] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 9, 10, 11, 12, 13, 14, 15, 16, 0,
        17, 18, 19, 20, 21, 22, 23, 24, 0, 25, 26, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
}
