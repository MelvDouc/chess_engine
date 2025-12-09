pub(crate) mod colors;
pub(crate) mod directions;
pub(crate) mod lines;
pub(crate) mod pieces;
pub(crate) mod squares;
pub(crate) mod wings;

pub(crate) const NB_COLORS: usize = 2;
pub(crate) const NB_PIECE_TYPES: usize = 6;
pub(crate) const NB_PIECES: usize = NB_PIECE_TYPES * NB_COLORS;
pub(crate) const NB_RANKS: usize = 8;
pub(crate) const NB_FILES: usize = 8;
pub(crate) const NB_SQUARES: usize = NB_RANKS * NB_FILES;
pub(crate) const NB_WINGS: usize = 2;
pub(crate) const NB_DIRECTIONS: usize = 8;

pub(crate) type Board = [usize; NB_SQUARES];
