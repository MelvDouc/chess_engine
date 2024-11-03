use crate::constants::board_constants::{BOARD_WIDTH, NB_SQUARES, square_of};

pub(crate) const MAX_BITBOARD: u64 = std::u64::MAX;

const MASK_SQUARES: [u64; NB_SQUARES + 1] = mask_squares();
pub(crate) const CLEAR_FILES: [u64; BOARD_WIDTH] = clear_files();

const fn mask_squares() -> [u64; NB_SQUARES + 1] {
    let mut arr: [u64; NB_SQUARES + 1] = [0; NB_SQUARES + 1];
    let mut sq = 0;

    while sq < NB_SQUARES {
        arr[sq] = 1 << sq;
        sq += 1;
    }

    arr
}

const fn clear_files() -> [u64; BOARD_WIDTH] {
    let mut arr: [u64; BOARD_WIDTH] = [MAX_BITBOARD; BOARD_WIDTH];
    let mut rank = 0;
    let mut file: usize;

    while rank < BOARD_WIDTH {
        file = 0;

        while file < BOARD_WIDTH {
            let sq = square_of(file, rank);
            arr[rank] &= !MASK_SQUARES[sq];
            file += 1;
        }

        rank += 1;
    }

    arr
}

/// Get the one-bit mask corresponding to a given square.
pub(crate) const fn bitboard_of(sq: usize) -> u64 {
    MASK_SQUARES[sq]
}

pub(crate) const fn bitboard_from_square(rank: usize, file: usize) -> u64 {
    bitboard_of(square_of(rank, file))
}

/// Get the square corresponding to a given one-bit mask.
pub(crate) fn bitboard_to_square(bb: u64) -> usize {
    (bb as f64).log2() as usize
}

/// Set the bit at the square position.
pub(crate) fn set_square(bb: &mut u64, sq: usize) {
    *bb |= MASK_SQUARES[sq]
}

/// Clear the bit at the square position.
pub(crate) fn clear_square(bb: &mut u64, sq: usize) {
    *bb &= !MASK_SQUARES[sq]
}

/// Clear the rightmost set bit in a bitboard and return its position.
pub(crate) fn pop_right(bb: &mut u64) -> usize {
    let sq = (*bb).trailing_zeros() as usize;
    clear_square(bb, sq);
    sq
}
