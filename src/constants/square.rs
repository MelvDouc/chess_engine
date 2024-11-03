use super::board_constants::NB_SQUARES;

pub(crate) const A1: usize = 0;
pub(crate) const B1: usize = 1;
pub(crate) const C1: usize = 2;
pub(crate) const D1: usize = 3;
pub(crate) const E1: usize = 4;
pub(crate) const F1: usize = 5;
pub(crate) const G1: usize = 6;
pub(crate) const H1: usize = 7;
pub(crate) const A2: usize = 8;
pub(crate) const B2: usize = 9;
pub(crate) const C2: usize = 10;
pub(crate) const D2: usize = 11;
pub(crate) const E2: usize = 12;
pub(crate) const F2: usize = 13;
pub(crate) const G2: usize = 14;
pub(crate) const H2: usize = 15;
pub(crate) const A3: usize = 16;
pub(crate) const B3: usize = 17;
pub(crate) const C3: usize = 18;
pub(crate) const D3: usize = 19;
pub(crate) const E3: usize = 20;
pub(crate) const F3: usize = 21;
pub(crate) const G3: usize = 22;
pub(crate) const H3: usize = 23;
pub(crate) const A4: usize = 24;
pub(crate) const B4: usize = 25;
pub(crate) const C4: usize = 26;
pub(crate) const D4: usize = 27;
pub(crate) const E4: usize = 28;
pub(crate) const F4: usize = 29;
pub(crate) const G4: usize = 30;
pub(crate) const H4: usize = 31;
pub(crate) const A5: usize = 32;
pub(crate) const B5: usize = 33;
pub(crate) const C5: usize = 34;
pub(crate) const D5: usize = 35;
pub(crate) const E5: usize = 36;
pub(crate) const F5: usize = 37;
pub(crate) const G5: usize = 38;
pub(crate) const H5: usize = 39;
pub(crate) const A6: usize = 40;
pub(crate) const B6: usize = 41;
pub(crate) const C6: usize = 42;
pub(crate) const D6: usize = 43;
pub(crate) const E6: usize = 44;
pub(crate) const F6: usize = 45;
pub(crate) const G6: usize = 46;
pub(crate) const H6: usize = 47;
pub(crate) const A7: usize = 48;
pub(crate) const B7: usize = 49;
pub(crate) const C7: usize = 50;
pub(crate) const D7: usize = 51;
pub(crate) const E7: usize = 52;
pub(crate) const F7: usize = 53;
pub(crate) const G7: usize = 54;
pub(crate) const H7: usize = 55;
pub(crate) const A8: usize = 56;
pub(crate) const B8: usize = 57;
pub(crate) const C8: usize = 58;
pub(crate) const D8: usize = 59;
pub(crate) const E8: usize = 60;
pub(crate) const F8: usize = 61;
pub(crate) const G8: usize = 62;
pub(crate) const H8: usize = 63;
pub(crate) const NONE_SQUARE: usize = 64;

const SQUARE_NAMES: [&str; NB_SQUARES + 1] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "-",
];

pub(crate) fn name_of(sq: usize) -> &'static str {
    SQUARE_NAMES[sq]
}

pub(crate) fn from_name(name: &str) -> usize {
    for sq in A1..=H8 {
        if SQUARE_NAMES[sq].eq(name) {
            return sq;
        }
    }

    NONE_SQUARE
}
