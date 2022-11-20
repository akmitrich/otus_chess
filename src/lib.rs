pub mod tester;

use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Chess(u64);

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Chess {
    const VERTICAL_A: u64 = 0x0101010101010101;
    const HORIZONTAL_1: u64 = 0x00000000000000FF;

    pub fn to_int(&self) -> u64 {
        self.0
    }

    pub fn not(&self) -> Self {
        Self(!self.0)
    }

    pub fn vertical(n: u8) -> Self {
        assert!(0 < n && n < 9, "There are 8 verticals on a board.");
        Self(Self::VERTICAL_A << (n - 1))
    }

    pub fn horizontal(n: u8) -> Self {
        assert!(0 < n && n < 9, "There are 8 horizontals on a board.");
        Self(Self::HORIZONTAL_1 << (8 * (n - 1)))
    }

    pub fn exc_vertical(n: u8) -> Self {
        Self::vertical(n).not()
    }

    pub fn exc_horizontal(n: u8) -> Self {
        Self::horizontal(n).not()
    }

    pub fn king_moves(pos: u8) -> Self {
        assert!(pos < 64, "There are 0..63 cells on a board.");
        let king = 1_u64 << pos;
        let k_a = king & Self::exc_vertical(1).0;
        let k_h = king & Self::exc_vertical(8).0;
        let moves = (k_h << 9) //up right
        | (king << 8) // up
        | (k_a << 7) // up left
        | (k_h << 1) // right
        | (k_a >> 1) // left
        | (k_h >> 7) // down right
        | (king >> 8) // down
        | (k_a >> 9); // down left
        Self(moves)
    }

    pub fn knight_moves(pos: u8) -> Self {
        assert!(pos < 64, "There are 0..63 cells on a board.");
        let knight = 1_u64 << pos;
        let k_a = knight & Self::exc_vertical(1).0;
        let k_h = knight & Self::exc_vertical(8).0;
        let k_b = knight & Self::exc_vertical(2).0;
        let k_f = knight & Self::exc_vertical(7).0;
        let mask = (k_h << 17) // up-right
            | (k_a << 15) // up-left
            | ((k_h & k_f) << 10) // right-up
            | ((k_a & k_b) << 6) // left-up
            | ((k_h & k_f) >> 6) // right-down
            | ((k_a & k_b) >> 10) // left-down
            | (k_h >> 15) // down-right
            | (k_a >> 17); // down-left
        Self(mask)
    }

    pub fn rook_moves(pos: u8) -> Self {
        assert!(pos < 64, "There are 0..63 cells on a board.");
        let rook_v = pos % 8 + 1;
        let rook_h = pos / 8 + 1;
        let mask = Chess::vertical(rook_v).0 ^ Chess::horizontal(rook_h).0;
        Self(mask)
    }

    pub fn number_of_positions(&self) -> u8 {
        cache_bits(self.0)
    }
}

fn _popcnt0(number: u64) -> u8 {
    (0..64).fold(0, |sum, current| {
        if (number & (1 << current)) > 0 {
            sum + 1
        } else {
            sum
        }
    })
}

fn _popcnt1(number: u64) -> u8 {
    let mut cnt = 0;
    let mut mask = number;
    while mask > 0 {
        if (mask & 1) == 1 {
            cnt += 1;
        }
        mask >>= 1;
    }
    cnt
}

pub fn popcnt2(number: u64) -> u8 {
    let mut cnt = 0;
    let mut mask = number;
    while mask > 0 {
        cnt += 1;
        mask &= mask - 1;
    }
    cnt
}

pub fn cache_bits(number: u64) -> u8 {
    const CACHE: [u8; 256] = [
        0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4,
        4, 5, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5,
        4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4,
        4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6,
        4, 5, 5, 6, 5, 6, 6, 7, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4,
        4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5,
        4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4,
        4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
        4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    ]; // CACHE[x] = number of 1s in byte x
    let mask = number.to_be_bytes();
    mask.iter()
        .fold(0, |sum, current| sum + CACHE[*current as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = Chess(0);
        c.0 = 0x7f7f7f7f7f7f7f7f;
    }

    #[test]
    fn vertical_horizontal() {
        let vertical_c = Chess::vertical(3);
        assert_eq!(0x0404040404040404, vertical_c.to_int());
        let horizontal_4 = Chess::horizontal(4);
        assert_eq!(0x00000000FF000000, horizontal_4.to_int());
        let no_vertical_d = Chess::exc_vertical(4);
        assert_eq!(0xF7F7F7F7F7F7F7F7, no_vertical_d.to_int());
        let no_horizontal_5 = Chess::exc_horizontal(5);
        assert_eq!(0xFFFFFF00FFFFFFFF, no_horizontal_5.to_int());
        let no_vertical_a = Chess::exc_vertical(1);
        assert_eq!(0xFEFEFEFEFEFEFEFE, no_vertical_a.to_int());
    }

    #[test]
    fn popcnt() {
        assert_eq!(0, popcnt2(0));
        assert_eq!(1, popcnt2(1));
        assert_eq!(1, popcnt2(2));
        assert_eq!(8, popcnt2(255));
    }

    #[test]
    fn cached() {
        assert_eq!(8, cache_bits(0x0101010101010101));
        assert_eq!(0, cache_bits(0));
        assert_eq!(1, cache_bits(1));
        assert_eq!(1, cache_bits(2));
        assert_eq!(8, cache_bits(255));
    }
}
