pub(crate) const KING_SIDE: usize = 0;
pub(crate) const QUEEN_SIDE: usize = 1;

pub(crate) const WINGS: [usize; 2] = [KING_SIDE, QUEEN_SIDE];

pub(crate) fn get_wing(king_src_sq: usize, king_dest_sq: usize) -> usize {
    if king_dest_sq < king_src_sq {
        return QUEEN_SIDE;
    }

    KING_SIDE
}
