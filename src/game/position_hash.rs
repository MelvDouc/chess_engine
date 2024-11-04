use std::collections::HashSet;

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

use crate::{
    constants::board_constants::{file_of, BOARD_WIDTH, NB_PIECES, NB_SQUARES},
    utils::bitboard::MAX_BITBOARD,
};

const HASH_COUNT_PIECES: usize = NB_SQUARES * NB_PIECES;
const HASH_COUNT_CASTLING: usize = 16;
const HASH_COUNT_EP_FILES: usize = BOARD_WIDTH + 1;
const HASH_COUNT: usize = HASH_COUNT_PIECES + 1 + HASH_COUNT_CASTLING + HASH_COUNT_EP_FILES;

const INDEX_COLOR_HASH: usize = HASH_COUNT_PIECES;
const INDEX_CASTLING_HASH_0: usize = INDEX_COLOR_HASH + 1;
const INDEX_EP_FILE_HASH_0: usize = INDEX_CASTLING_HASH_0 + HASH_COUNT_CASTLING;

lazy_static! {
    static ref HASHES: [u64; HASH_COUNT] = create_hashes();
}

pub(super) fn piece_hash(piece: usize, sq: usize) -> u64 {
    HASHES[NB_SQUARES * piece + sq]
}

pub(super) fn color_hash() -> u64 {
    HASHES[INDEX_COLOR_HASH]
}

pub(super) fn castling_hash(castling: u8) -> u64 {
    HASHES[INDEX_CASTLING_HASH_0 + (castling as usize)]
}

pub(super) fn en_passant_hash(sq: usize) -> u64 {
    HASHES[INDEX_EP_FILE_HASH_0 + file_of(sq)]
}

fn create_hashes() -> [u64; HASH_COUNT] {
    let mut hashes: [u64; HASH_COUNT] = [0; HASH_COUNT];
    let mut memo = HashSet::<u64>::new();
    let mut rng = thread_rng();

    while memo.len() < HASH_COUNT {
        let hash = rng.gen_range(1u64..=MAX_BITBOARD);

        if !memo.contains(&hash) {
            hashes[memo.len()] = hash;
            memo.insert(hash);
        }
    }

    hashes
}
