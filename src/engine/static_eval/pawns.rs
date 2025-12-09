use crate::{
    bit_boards::set_bits,
    engine::score::Score,
    game::{
        board::{
            self, colors,
            lines::{self, file_mask},
            pieces, squares,
        },
        moves::piece_attacks,
        position::Position,
    },
    macros::const_while,
};

pub(super) const fn eval_pawns(pos: &Position, color: usize) -> Score {
    let pawn_occ = pos.pawn_occupancy(color);
    let mut score = 0;

    set_bits!(pawn_occ, sq, {
        if is_passed(sq, color, pawn_occ) {
            score += 71;

            if is_protected(sq, color, pawn_occ) {
                score += 41;
            }

            continue;
        }

        if is_isolated(sq, pawn_occ) {
            score -= 53;
            continue;
        }

        if is_backward(sq, color, pawn_occ) {
            score -= 37;
        }
    });

    score
}

const fn adjacent_file_mask(file: usize) -> u64 {
    match file {
        lines::FILE_A => file_mask(file + 1),
        lines::FILE_H => file_mask(file - 1),
        _ => file_mask(file - 1) | file_mask(file + 1),
    }
}

const PASSED_MASKS: [[u64; board::NB_SQUARES]; board::NB_COLORS] = {
    let mut table = [[0; board::NB_SQUARES]; board::NB_COLORS];

    const_while!(sq, squares::A2, squares::A8, {
        let rank = squares::rank_of(sq);
        let file = squares::file_of(sq);
        let mask = file_mask(file) | adjacent_file_mask(file);
        table[colors::WHITE][sq] = mask << (board::NB_FILES * (rank + 1));
        table[colors::BLACK][sq] = mask >> (board::NB_FILES * (squares::rev_coord(rank) + 1));
    });

    table
};

const fn is_passed(sq: usize, color: usize, pawn_occ: u64) -> bool {
    pawn_occ & PASSED_MASKS[color][sq] == 0
}

const fn is_protected(sq: usize, color: usize, pawn_occ: u64) -> bool {
    let enemy_pawn = pieces::pawn_of(colors::rev(color));

    piece_attacks(enemy_pawn, sq, 0) & pawn_occ != 0
}

const fn is_isolated(sq: usize, pawn_occ: u64) -> bool {
    pawn_occ & adjacent_file_mask(squares::file_of(sq)) == 0
}

const fn is_backward(sq: usize, color: usize, pawn_occ: u64) -> bool {
    (pawn_occ & piece_attacks(pieces::pawn_of(color), sq, 0)) != 0
        && (pawn_occ & PASSED_MASKS[colors::rev(color)][sq]) == 0
}
