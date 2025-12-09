use crate::game::moves::castling::NB_BITS_CASTLING_RIGHTS;

/// A number that stores information about a position that has to be restored after undoing a move.
pub(super) type UndoInfo = u32;

const NB_BITS_CASTLING: UndoInfo = NB_BITS_CASTLING_RIGHTS as UndoInfo;
const NB_BITS_EP: UndoInfo = 7;

const OFFSET_EP: UndoInfo = NB_BITS_CASTLING;
const OFFSET_HMC: UndoInfo = OFFSET_EP + NB_BITS_EP;

const MASK_CASTLING: UndoInfo = (1 << NB_BITS_CASTLING) - 1;
const MASK_EP: UndoInfo = (1 << NB_BITS_EP) - 1;

pub(super) const fn encode(castling_rights: u8, ep_sq: usize, half_move_clock: u8) -> UndoInfo {
    castling_rights as UndoInfo
        | (ep_sq as UndoInfo) << OFFSET_EP
        | (half_move_clock as UndoInfo) << OFFSET_HMC
}

pub(super) const fn castling_rights(undo_info: UndoInfo) -> u8 {
    (undo_info & MASK_CASTLING) as u8
}

pub(super) const fn ep_square(undo_info: UndoInfo) -> usize {
    (undo_info >> OFFSET_EP & MASK_EP) as usize
}

pub(super) const fn half_move_clock(undo_info: UndoInfo) -> u8 {
    (undo_info >> OFFSET_HMC) as u8
}
