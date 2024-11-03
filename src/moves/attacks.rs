use crate::{
    constants::{
        board_constants::{FILE_A, FILE_B, FILE_G, FILE_H, NB_SQUARES},
        piece::{
            BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
            WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
        },
        square,
    },
    utils::bitboard::{bitboard_of, CLEAR_FILES, MAX_BITBOARD},
};

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
const ROOK_OFFSETS: [OffsetTuple; 4] = [
    (8, MAX_BITBOARD),
    (1, CLEAR_FILE_A),
    (-8, MAX_BITBOARD),
    (-1, CLEAR_FILE_H),
];
const BISHOP_OFFSETS: [OffsetTuple; 4] = [
    (7, CLEAR_FILE_H),
    (9, CLEAR_FILE_A),
    (-9, CLEAR_FILE_H),
    (-7, CLEAR_FILE_A),
];

const SHORT_RANGE_ATTACKS: [[u64; NB_SQUARES]; 6] = create_short_range_attacks();

const fn apply_offset(bb: u64, offset: i8, clear: u64) -> u64 {
    if offset > 0 {
        return (bb << offset) & clear;
    }

    (bb >> -offset) & clear
}

const fn short_range_attacks(bb: u64, offsets: &[OffsetTuple]) -> u64 {
    let mut attacks = 0u64;
    let mut i = 0;

    while i < offsets.len() {
        let (offset, clear) = offsets[i];
        attacks |= apply_offset(bb, offset, clear);
        i += 1;
    }

    attacks
}

const fn create_short_range_attacks() -> [[u64; NB_SQUARES]; 6] {
    let mut attacks: [[u64; NB_SQUARES]; 6] = [[0u64; NB_SQUARES]; 6];
    let mut sq = square::A1;

    while sq <= square::H8 {
        let bb = bitboard_of(sq);

        attacks[WHITE_PAWN][sq] = short_range_attacks(bb, &WHITE_PAWN_OFFSETS);
        attacks[BLACK_PAWN][sq] = short_range_attacks(bb, &BLACK_PAWN_OFFSETS);

        let knight_attacks = short_range_attacks(bb, &KNIGHT_OFFSETS);
        attacks[WHITE_KNIGHT][sq] = knight_attacks;
        attacks[BLACK_KNIGHT][sq] = knight_attacks;

        let king_attacks = short_range_attacks(bb, &KING_OFFSETS);
        attacks[WHITE_KING][sq] = king_attacks;
        attacks[BLACK_KING][sq] = king_attacks;

        sq += 1;
    }

    attacks
}

const fn long_range_attacks(bb: u64, offsets: &[OffsetTuple], occupancy: u64) -> u64 {
    let mut attacks = 0u64;
    let mut i = 0;

    while i < offsets.len() {
        let (offset, clear) = offsets[i];
        let mut dest_bb = apply_offset(bb, offset, clear);

        while dest_bb != 0u64 {
            attacks |= dest_bb;

            if occupancy & dest_bb != 0u64 {
                break;
            }

            dest_bb = apply_offset(dest_bb, offset, clear);
        }

        i += 1;
    }

    attacks
}

pub(crate) const fn piece_attacks(piece: usize, sq: usize, occupancy: u64) -> u64 {
    match piece {
        WHITE_PAWN | BLACK_PAWN | WHITE_KNIGHT | BLACK_KNIGHT | WHITE_KING | BLACK_KING => {
            SHORT_RANGE_ATTACKS[piece][sq]
        }
        WHITE_BISHOP | BLACK_BISHOP => {
            long_range_attacks(bitboard_of(sq), &BISHOP_OFFSETS, occupancy)
        }
        WHITE_ROOK | BLACK_ROOK => long_range_attacks(bitboard_of(sq), &ROOK_OFFSETS, occupancy),
        WHITE_QUEEN | BLACK_QUEEN => {
            piece_attacks(WHITE_BISHOP, sq, occupancy) | piece_attacks(WHITE_ROOK, sq, occupancy)
        }
        _ => panic!("Invalid piece"),
    }
}

type OffsetTuple = (i8, u64);
