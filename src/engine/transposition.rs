use crate::{
    engine::score::Score,
    game::moves::{Move, NULL_MOVE},
    macros::ternary,
};

const TABLE_SIZE: usize = 1 << 22;

pub(super) mod flags {
    use super::Score;

    pub(crate) const NONE: Flag = 0;
    pub(crate) const EXACT: Flag = 1;
    pub(crate) const LOWER: Flag = 2;
    pub(crate) const UPPER: Flag = 3;

    pub(crate) const fn get_flag(old_alpha: Score, beta: Score, best_score: Score) -> Flag {
        if best_score <= old_alpha {
            return UPPER;
        }

        if best_score >= beta {
            return LOWER;
        }

        EXACT
    }

    pub(crate) type Flag = u8;
}

#[derive(Copy, Clone)]
pub(super) struct Entry {
    pub(super) flag: flags::Flag,
    pub(super) hash: u64,
    pub(super) score: Score,
    pub(super) depth: usize,
    pub(super) mv: Move,
}

impl Entry {
    pub(super) const fn new(
        flag: flags::Flag,
        hash: u64,
        score: Score,
        depth: usize,
        mv: Move,
    ) -> Self {
        Self {
            flag,
            hash,
            score,
            depth,
            mv,
        }
    }

    pub(super) const fn exact(hash: u64, score: Score, depth: usize, mv: Move) -> Self {
        Self::new(flags::EXACT, hash, score, depth, mv)
    }
}

pub(super) fn create_table() -> Table {
    let null_entry = Entry::new(flags::NONE, 0, 0, 0, NULL_MOVE);
    let table: Vec<Entry> = vec![null_entry; TABLE_SIZE];
    table.into_boxed_slice()
}

const fn get_index(hash: u64) -> usize {
    const INDEX_MASK: usize = TABLE_SIZE - 1;

    hash as usize & INDEX_MASK
}

pub(super) const fn get_entry(tt: &Table, hash: u64) -> Option<Entry> {
    let entry = tt[get_index(hash)];

    ternary!(entry.hash == hash, Some(entry), None)
}

pub(super) const fn set_entry(tt: &mut Table, entry: Entry) {
    let index = get_index(entry.hash);
    let prev_entry = tt[index];

    if prev_entry.flag == flags::NONE
        || entry.depth > prev_entry.depth
        || entry.flag == flags::EXACT && prev_entry.flag != flags::EXACT
    {
        tt[index] = entry;
    }
}

pub(super) type Table = Box<[Entry]>;
