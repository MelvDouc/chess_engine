use crate::{
    constants::{
        board_constants::{file_of, rank_of},
        piece::{BLACK_KNIGHT, WHITE_KING, WHITE_ROOK},
        square::{A1, H8},
    },
    moves::piece_attacks,
    utils::{bitboard::pop_right, math::usize_abs_diff},
};

#[test]
fn a_rook_should_always_attack_14_squares() -> () {
    let occupancy = 0u64;

    for sq in A1..=H8 {
        let attacks = piece_attacks(WHITE_ROOK, sq, occupancy);
        assert_eq!(attacks.count_ones(), 14);
    }
}

#[test]
fn knight_attacks() -> () {
    for src_sq in A1..=H8 {
        let mut attacks = piece_attacks(BLACK_KNIGHT, src_sq, 0u64);
        let src_rank = rank_of(src_sq);
        let src_file = file_of(src_sq);

        while attacks != 0u64 {
            let dest_sq = pop_right(&mut attacks);
            assert_ne!(src_sq, dest_sq);
            let dest_rank = rank_of(dest_sq);
            let dest_file = file_of(dest_sq);
            let rank_diff = usize_abs_diff(src_rank, dest_rank);
            let file_diff = usize_abs_diff(src_file, dest_file);
            assert!(rank_diff == 2 && file_diff == 1 || rank_diff == 1 && file_diff == 2);
        }
    }
}

#[test]
fn king_attacks() -> () {
    for src_sq in A1..=H8 {
        let mut attacks = piece_attacks(WHITE_KING, src_sq, 0u64);
        let src_rank = rank_of(src_sq);
        let src_file = file_of(src_sq);

        while attacks != 0u64 {
            let dest_sq = pop_right(&mut attacks);
            assert_ne!(src_sq, dest_sq);
            let dest_rank = rank_of(dest_sq);
            let dest_file = file_of(dest_sq);
            let rank_diff = usize_abs_diff(src_rank, dest_rank);
            let file_diff = usize_abs_diff(src_file, dest_file);
            assert!(rank_diff <= 1 && file_diff <= 1);
        }
    }
}
