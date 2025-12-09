mod attacks;
pub(crate) mod castling;
pub(crate) mod encoding;
mod move_list;

pub(crate) type Move = u32;

pub(crate) const NULL_MOVE: Move = 0;

pub(crate) use attacks::piece_attacks;
pub(crate) use move_list::MoveList;
