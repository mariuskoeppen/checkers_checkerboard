#![allow(dead_code)]

#[derive(Debug)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard::EMPTY
    }

    /// Bit at index is set to one.
    pub fn set(&mut self, index: usize) {
        println!("index: {}", index);
        println!("self.0: {}", self.0);
        self.0 |= 1 << index;
        println!("self.0: {}", self.0);
    }

    pub fn set_multiple(&mut self, bb: &Bitboard) -> Self {
        self.0 |= bb.0;
        // *self
        Bitboard(self.0)
    }

    /// Bit at index is set to zero.
    pub fn unset(&mut self, index: usize) {
        self.0 &= !(1 << index);
    }

    pub fn get(&self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }

    /// Create Bitboard with only one bit at index set to one.
    pub fn create_one_hot(index: usize) -> Self {
        Bitboard(1 << index)
    }

    /// Number of ones in the bitboard.
    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Number of zeros in the bitboard.
    pub fn count_zeroes(&self) -> usize {
        self.0.count_zeros() as usize
    }

    /// Returns the inner u64 value.
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Create Bitboard from u64.
    pub fn from_u64(value: u64) -> Self {
        Bitboard(value)
    }
}

/// Constants
impl Bitboard {
    /// All tiles.
    pub const ALL: Bitboard = Bitboard(0x1FEFF7FBFC00);
    /// Row 1
    pub const ONE: Bitboard = Bitboard(0x1E0000000000);
    /// Row 2
    pub const TWO: Bitboard = Bitboard(0x1E000000000);
    /// Row 3
    pub const THREE: Bitboard = Bitboard(0xF00000000);
    /// Row 4
    pub const FOUR: Bitboard = Bitboard(0xF0000000);
    /// Row 5
    pub const FIVE: Bitboard = Bitboard(0x7800000);
    /// Row 6
    pub const SIX: Bitboard = Bitboard(0x780000);
    /// Row 7
    pub const SEVEN: Bitboard = Bitboard(0x3C000);
    /// Row 8
    pub const EIGHT: Bitboard = Bitboard(0x3C00);

    /// Column A
    pub const A: Bitboard = Bitboard(0x20100804000);
    /// Column B
    pub const B: Bitboard = Bitboard(0x2010080400);
    /// Column C
    pub const C: Bitboard = Bitboard(0x40201008000);
    /// Column D
    pub const D: Bitboard = Bitboard(0x4020100800);
    /// Column E
    pub const E: Bitboard = Bitboard(0x80402010000);
    /// Column F
    pub const F: Bitboard = Bitboard(0x8040201000);
    /// Column G
    pub const G: Bitboard = Bitboard(0x100804020000);
    /// Column H
    pub const H: Bitboard = Bitboard(0x10080402000);

    /// All even rows
    pub const ALL_EVEN_ROWS: Bitboard = Bitboard(0x1E0F0783C00);
    /// All odd rows
    pub const ALL_ODD_ROWS: Bitboard = Bitboard(0x1E0F0783C000);
    /// All even columns
    pub const ALL_EVEN_COLUMNS: Bitboard = Bitboard(0x1E0F0783C00);
    /// All odd columns
    pub const ALL_ODD_COLUMNS: Bitboard = Bitboard(0x1E0F0783C000);

    /// Empty
    pub const EMPTY: Bitboard = Bitboard(0x0);

    pub const BLACK_SIDE: Bitboard = Bitboard(0x7BFC00);
    pub const WHITE_SIDE: Bitboard = Bitboard(0x1FEF00000000);
}

impl Bitboard {}

// region: std Bitboard implentations

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard(0)
    }
}

impl AsMut<u64> for Bitboard {
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl std::fmt::UpperHex for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

impl std::fmt::Binary for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0b{:b}", self.0)
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

impl std::str::FromStr for Bitboard {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bitboard(u64::from_str_radix(s, 16)?))
    }
}

impl Iterator for Bitboard {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as usize;
            self.0 &= !(1 << index);
            Some(index)
        }
    }
}

impl Clone for Bitboard {
    fn clone(&self) -> Self {
        Bitboard(self.0)
        // *self
    }
}

impl Copy for Bitboard {}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOr for &Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXor for &Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & !rhs.0)
    }
}

impl std::ops::Sub for &Bitboard {
    type Output = Bitboard;

    fn sub(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & !rhs.0)
    }
}

impl std::ops::Add for Bitboard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::Add for &Bitboard {
    type Output = Bitboard;

    fn add(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::Shl<usize> for &Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::Shr<usize> for &Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitAndAssign for &mut Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitOrAssign for &mut Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::BitXorAssign for &mut Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::SubAssign for Bitboard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !rhs.0;
    }
}

impl std::ops::SubAssign for &mut Bitboard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !rhs.0;
    }
}

impl std::ops::AddAssign for Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::AddAssign for &mut Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl std::ops::ShlAssign<usize> for &mut Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl std::ops::ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl std::ops::ShrAssign<usize> for &mut Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl core::cmp::PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl core::cmp::Eq for Bitboard {}

impl core::cmp::PartialOrd for Bitboard {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Bitboard {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

// endregion std Bitboard implentations
