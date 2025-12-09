mod prng;

pub(crate) use prng::Prng;

macro_rules! set_bit {
    ($bb: expr, $pos: expr) => {
        $bb |= 1u64 << $pos
    };
}

macro_rules! clear_bit {
    ($bb: expr, $pos: expr) => {
        $bb &= !(1u64 << $pos)
    };
}

/// Iterate over the positions of the sets bits in a bit board.
macro_rules! set_bits {
    ($bb: expr, $sq: ident, $block: block) => {
        let mut bb = $bb;

        while bb != 0 {
            let $sq = bb.trailing_zeros() as usize;
            bb &= bb - 1;
            $block
        }
    };
}

pub(crate) use clear_bit;
pub(crate) use set_bit;
pub(crate) use set_bits;

pub(crate) const fn bit_mask(sq: usize) -> u64 {
    1 << sq
}

pub(crate) const fn is_bit_set(bb: u64, pos: usize) -> bool {
    bb & bit_mask(pos) != 0
}

/// Create a bitboard where bits are set in a given range.
/// * `from` - Position of first set bit.
/// * `to` - Position of last set bit.
pub(crate) const fn mask_consecutive(from: usize, to: usize) -> u64 {
    let width = to - from + 1;
    (bit_mask(width) - 1) << from
}
