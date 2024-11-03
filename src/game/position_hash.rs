use std::collections::HashSet;

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

use crate::{
    constants::board_constants::{file_of, BOARD_WIDTH, NB_PIECES, NB_SQUARES},
    utils::bitboard::MAX_BITBOARD
};

const NB_PIECE_HASHES: usize = NB_SQUARES * NB_PIECES;
const NB_CASTLING_HASHES: usize = 16;
const NB_FILE_HASHES: usize = BOARD_WIDTH + 1;
const NB_HASHES: usize = NB_PIECE_HASHES + 1 + NB_CASTLING_HASHES + NB_FILE_HASHES;

lazy_static! {
    static ref HASHES: [u64; NB_HASHES] = create_hashes();
}

pub(super) fn piece_hash(piece: usize, sq: usize) -> u64 {
    HASHES[NB_SQUARES * piece + sq]
}

pub(super) fn color_hash() -> u64 {
    HASHES[NB_PIECE_HASHES]
}

pub(super) fn castling_hash(castling: u8) -> u64 {
    HASHES[NB_PIECE_HASHES + 1 + (castling as usize)]
}

pub(super) fn en_passant_hash(sq: usize) -> u64 {
    HASHES[NB_PIECE_HASHES + 1 + NB_CASTLING_HASHES + file_of(sq)]
}

fn create_hashes() -> [u64; NB_HASHES] {
    let mut hashes: [u64; NB_HASHES] = [0; NB_HASHES];
    let mut memo = HashSet::<u64>::new();
    let mut rng = thread_rng();

    while memo.len() < NB_HASHES {
        let hash = rng.gen_range(1u64..=MAX_BITBOARD);

        if !memo.contains(&hash) {
            hashes[memo.len()] = hash;
            memo.insert(hash);
        }
    }

    hashes
}
