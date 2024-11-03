pub(crate) const BOARD_WIDTH: usize = 8;
pub(crate) const NB_SQUARES: usize = BOARD_WIDTH * BOARD_WIDTH;

/// The number of **unique** pieces.
pub(crate) const NB_PIECES: usize = 12;

pub(crate) const RANK_1: usize = 0;
pub(crate) const RANK_2: usize = 1;
pub(crate) const RANK_3: usize = 2;
pub(crate) const RANK_4: usize = 3;
pub(crate) const RANK_5: usize = 4;
pub(crate) const RANK_6: usize = 5;
pub(crate) const RANK_7: usize = 6;
pub(crate) const RANK_8: usize = 7;

pub(crate) const FILE_A: usize = 0;
pub(crate) const FILE_B: usize = 1;
pub(crate) const FILE_C: usize = 2;
pub(crate) const FILE_D: usize = 3;
pub(crate) const FILE_E: usize = 4;
pub(crate) const FILE_F: usize = 5;
pub(crate) const FILE_G: usize = 6;
pub(crate) const FILE_H: usize = 7;

pub(crate) const INITIAL_PIECE_RANKS: [usize; 2] = [RANK_1, RANK_8];
pub(crate) const INITIAL_PAWN_RANKS: [usize; 2] = [RANK_2, RANK_7];

/// Get the zero-indexed rank of a square.
pub(crate) const fn rank_of(sq: usize) -> usize {
    sq >> 3
}

/// Get the zero-indexed file of a square.
pub(crate) const fn file_of(sq: usize) -> usize {
    sq & 7
}

/// Get the zero-indexed square at a given rank and file.
pub(crate) const fn square_of(rank: usize, file: usize) -> usize {
    (rank << 3) + file
}
