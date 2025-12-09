mod long_range;
mod short_range;

use crate::game::board::pieces;
use long_range::*;
use short_range::*;

pub(crate) const fn piece_attacks(piece: usize, sq: usize, occ: u64) -> u64 {
    match piece {
        pieces::WHITE_PAWN => WHITE_PAWN_ATTACKS[sq],
        pieces::BLACK_PAWN => BLACK_PAWN_ATTACKS[sq],
        pieces::WHITE_KNIGHT | pieces::BLACK_KNIGHT => KNIGHT_ATTACKS[sq],
        pieces::WHITE_KING | pieces::BLACK_KING => KING_ATTACKS[sq],
        pieces::WHITE_BISHOP | pieces::BLACK_BISHOP => bishop_attacks(sq, occ),
        pieces::WHITE_ROOK | pieces::BLACK_ROOK => rook_attacks(sq, occ),
        pieces::WHITE_QUEEN | pieces::BLACK_QUEEN => {
            bishop_attacks(sq, occ) | rook_attacks(sq, occ)
        }
        _ => panic!("Invalid piece"),
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use crate::{
        bit_boards::{set_bit, set_bits},
        game::board::{NB_SQUARES, directions as dirs, squares},
    };

    use super::*;

    fn dumb_fill(src_sq: usize, dir_array: &[usize], occ: u64) -> u64 {
        let mut attacks = 0;

        for &dir in dir_array {
            let dist = dirs::distance_to_edge(src_sq, dir);
            let mut dest_sq = src_sq;
            let mut dir_attacks = 0;

            for _ in 0..dist {
                dest_sq = dirs::next_square(dest_sq, dir);
                set_bit!(dir_attacks, dest_sq);

                if dir_attacks & occ != 0 {
                    break;
                }
            }

            attacks |= dir_attacks;
        }

        attacks
    }

    fn test_short_range(piece: usize, test: impl Fn(usize, usize) -> bool) {
        for src_sq in 0..NB_SQUARES {
            let attacks = piece_attacks(piece, src_sq, 0);

            set_bits!(attacks, dest_sq, {
                assert!(test(src_sq, dest_sq));
            });
        }
    }

    fn test_long_range(piece: usize, dir_array: &[usize]) {
        let occ = random::<u64>() & random::<u64>();

        for src_sq in 0..NB_SQUARES {
            let attacks1 = piece_attacks(piece, src_sq, occ);
            let attacks2 = dumb_fill(src_sq, dir_array, occ);
            assert_eq!(attacks1, attacks2);
        }
    }

    #[test]
    fn white_pawn() {
        fn test(src_sq: usize, dest_sq: usize) -> bool {
            let src_rank = squares::rank_of(src_sq);
            let src_file = squares::file_of(src_sq);
            let dest_rank = squares::rank_of(dest_sq);
            let dest_file = squares::file_of(dest_sq);
            dest_rank - src_rank == 1 && src_file.abs_diff(dest_file) == 1
        }

        test_short_range(pieces::WHITE_PAWN, test);
    }

    #[test]
    fn black_pawn() {
        fn test(src_sq: usize, dest_sq: usize) -> bool {
            let src_rank = squares::rank_of(src_sq);
            let src_file = squares::file_of(src_sq);
            let dest_rank = squares::rank_of(dest_sq);
            let dest_file = squares::file_of(dest_sq);
            src_rank - dest_rank == 1 && src_file.abs_diff(dest_file) == 1
        }

        test_short_range(pieces::BLACK_PAWN, test);
    }

    #[test]
    fn knight() {
        fn test(src_sq: usize, dest_sq: usize) -> bool {
            let rank_diff = squares::rank_of(src_sq).abs_diff(squares::rank_of(dest_sq));
            let file_diff = squares::file_of(src_sq).abs_diff(squares::file_of(dest_sq));
            rank_diff == 2 && file_diff == 1 || rank_diff == 1 && file_diff == 2
        }

        test_short_range(pieces::WHITE_KNIGHT, test);
    }

    #[test]
    fn king() {
        fn test(src_sq: usize, dest_sq: usize) -> bool {
            let rank_diff = squares::rank_of(src_sq).abs_diff(squares::rank_of(dest_sq));
            let file_diff = squares::file_of(src_sq).abs_diff(squares::file_of(dest_sq));

            rank_diff == 0 && file_diff == 1 || rank_diff == 1 && (file_diff == 0 || file_diff == 1)
        }

        test_short_range(pieces::BLACK_KING, test);
    }

    #[test]
    fn bishop() {
        let dir_array = [
            dirs::NORTH_WEST,
            dirs::NORTH_EAST,
            dirs::SOUTH_EAST,
            dirs::SOUTH_WEST,
        ];
        test_long_range(pieces::WHITE_BISHOP, &dir_array);
    }

    #[test]
    fn rook() {
        let dir_array = [dirs::NORTH, dirs::EAST, dirs::SOUTH, dirs::WEST];
        test_long_range(pieces::BLACK_ROOK, &dir_array);
    }
}
