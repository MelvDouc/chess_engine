use crate::{
    constants::{
        board_constants::{FILE_A, FILE_B, FILE_G, FILE_H, NB_SQUARES},
        piece::{BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN},
        square,
    },
    utils::bitboard::{bitboard_of, CLEAR_FILES, MAX_BITBOARD},
};

const NB_SHORT_RANGE_TYPES: usize = 6;

const CLEAR_FILE_A: u64 = CLEAR_FILES[FILE_A];
const CLEAR_FILE_B: u64 = CLEAR_FILES[FILE_B];
const CLEAR_FILE_AB: u64 = CLEAR_FILE_A & CLEAR_FILE_B;
const CLEAR_FILE_G: u64 = CLEAR_FILES[FILE_G];
const CLEAR_FILE_H: u64 = CLEAR_FILES[FILE_H];
const CLEAR_FILE_GH: u64 = CLEAR_FILE_G & CLEAR_FILE_H;

const WHITE_PAWN_OFFSETS: [OffsetTuple; 2] = [(7, CLEAR_FILE_H), (9, CLEAR_FILE_A)];
const BLACK_PAWN_OFFSETS: [OffsetTuple; 2] = [(-9, CLEAR_FILE_H), (-7, CLEAR_FILE_A)];
const KING_OFFSETS: [OffsetTuple; 8] = [
    (8, MAX_BITBOARD),
    (7, CLEAR_FILE_H),
    (9, CLEAR_FILE_A),
    (1, CLEAR_FILE_A),
    (-8, MAX_BITBOARD),
    (-7, CLEAR_FILE_A),
    (-9, CLEAR_FILE_H),
    (-1, CLEAR_FILE_H),
];
const KNIGHT_OFFSETS: [OffsetTuple; 8] = [
    (15, CLEAR_FILE_H),
    (17, CLEAR_FILE_A),
    (6, CLEAR_FILE_GH),
    (10, CLEAR_FILE_AB),
    (-17, CLEAR_FILE_H),
    (-15, CLEAR_FILE_A),
    (-10, CLEAR_FILE_GH),
    (-6, CLEAR_FILE_AB),
];

pub(super) const SHORT_RANGE_ATTACKS: [[u64; NB_SQUARES]; NB_SHORT_RANGE_TYPES] =
    short_range_attacks();

const fn apply_offset(bb: u64, offset: i8, clear: u64) -> u64 {
    if offset > 0 {
        return (bb << offset) & clear;
    }

    (bb >> -offset) & clear
}

const fn short_range_attacks() -> [[u64; NB_SQUARES]; NB_SHORT_RANGE_TYPES] {
    let mut attacks: [[u64; NB_SQUARES]; NB_SHORT_RANGE_TYPES] =
        [[0u64; NB_SQUARES]; NB_SHORT_RANGE_TYPES];
    let mut sq = square::A1;

    while sq <= square::H8 {
        let bb = bitboard_of(sq);

        attacks[WHITE_PAWN][sq] = short_range_attack(bb, &WHITE_PAWN_OFFSETS);
        attacks[BLACK_PAWN][sq] = short_range_attack(bb, &BLACK_PAWN_OFFSETS);

        let knight_attacks = short_range_attack(bb, &KNIGHT_OFFSETS);
        attacks[WHITE_KNIGHT][sq] = knight_attacks;
        attacks[BLACK_KNIGHT][sq] = knight_attacks;

        let king_attacks = short_range_attack(bb, &KING_OFFSETS);
        attacks[WHITE_KING][sq] = king_attacks;
        attacks[BLACK_KING][sq] = king_attacks;

        sq += 1;
    }

    attacks
}

const fn short_range_attack(bb: u64, offsets: &[OffsetTuple]) -> u64 {
    let mut attack = 0u64;
    let mut i = 0;

    while i < offsets.len() {
        let (offset, clear) = offsets[i];
        attack |= apply_offset(bb, offset, clear);
        i += 1;
    }

    attack
}

type OffsetTuple = (i8, u64);
