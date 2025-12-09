use crate::{
    game::board::{NB_FILES, NB_RANKS},
    macros::{const_while, ternary},
};

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

pub(crate) const fn rank_name(rank: usize) -> char {
    (b'1' + rank as u8) as char
}

pub(crate) const fn parse_rank(ch: char) -> Result<usize, ()> {
    match ch.to_digit(10) {
        Some(rank) => Ok((rank - 1) as usize),
        None => Err(()),
    }
}

pub(crate) const fn rank_mask(rank: usize) -> u64 {
    const RANK_1_MASK: u64 = (1 << NB_FILES) - 1;

    RANK_1_MASK << (rank * NB_FILES)
}

pub(crate) const fn file_name(file: usize) -> char {
    (b'a' + file as u8) as char
}

pub(crate) fn parse_file(ch: char) -> Result<usize, ()> {
    let file = ch as u8;

    ternary!(
        file >= b'a' && file <= b'z',
        Ok((file - b'a') as usize),
        Err(())
    )
}

pub(crate) const fn file_mask(file: usize) -> u64 {
    const FILE_A_MASK: u64 = {
        let mut mask = 1;

        const_while!(rank, 0, NB_RANKS, {
            mask |= mask << NB_FILES;
        });

        mask
    };

    FILE_A_MASK << file
}
