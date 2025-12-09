use crate::game::moves::{Move, NULL_MOVE};

pub(crate) type Pair = (Move, Move);
pub(crate) type Table = [Pair; super::MAX_DEPTH];

pub(crate) const fn create_table() -> Table {
    [(NULL_MOVE, NULL_MOVE); super::MAX_DEPTH]
}

pub(crate) const fn update(kmt: &mut Table, mv: Move, depth: usize) {
    if mv != kmt[depth].0 {
        kmt[depth] = (mv, kmt[depth].0);
    }
}
