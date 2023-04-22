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

pub struct PositionMapper;

#[allow(non_upper_case_globals)]
impl PositionMapper {
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
