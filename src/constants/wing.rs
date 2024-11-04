pub(crate) const KING_SIDE: u8 = 0;
pub(crate) const QUEEN_SIDE: u8 = 1;

pub(crate) const WINGS: [u8; 2] = [KING_SIDE, QUEEN_SIDE];

pub(crate) fn get_wing(king_src_sq: usize, king_dest_sq: usize) -> u8 {
    if king_dest_sq < king_src_sq {
        return QUEEN_SIDE;
    }

    KING_SIDE
}
