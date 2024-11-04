use crate::constants::piece::{
    type_of, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
};

mod dumb_fill;
mod short_attacks;
// mod sliding_attacks;

pub(crate) fn piece_attacks(piece: usize, sq: usize, occupancy: u64) -> u64 {
    match piece {
        WHITE_PAWN | BLACK_PAWN | WHITE_KNIGHT | BLACK_KNIGHT | WHITE_KING | BLACK_KING => {
            short_attacks::SHORT_RANGE_ATTACKS[piece][sq]
        }
        _ => dumb_fill::sliding_attacks(type_of(piece), sq, occupancy),
    }
}
