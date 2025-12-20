use crate::game::board::{NB_COLORS, lines};

pub(crate) const WHITE: usize = 0;
pub(crate) const BLACK: usize = 1;

pub(crate) const fn rev(color: usize) -> usize {
    color ^ 1
}

/// The initial piece rank of a color.
pub(crate) const fn piece_rank(color: usize) -> usize {
    const RANKS: [usize; NB_COLORS] = [lines::RANK_1, lines::RANK_8];

    RANKS[color]
}

/// The initial pawn rank of a color.
pub(crate) const fn pawn_rank(color: usize) -> usize {
    const RANKS: [usize; NB_COLORS] = [lines::RANK_2, lines::RANK_7];

    RANKS[color]
}

pub(crate) fn initial_of(color: usize) -> char {
    const INITIALS: [char; NB_COLORS] = ['w', 'b'];

    INITIALS[color]
}

pub(crate) fn from_initial(initial: char) -> Result<usize, ()> {
    match initial {
        'w' => Ok(WHITE),
        'b' => Ok(BLACK),
        _ => Err(()),
    }
}
