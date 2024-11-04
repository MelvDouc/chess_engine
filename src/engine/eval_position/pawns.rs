use crate::{
    constants::{
        board_constants::{file_of, rank_of, BOARD_WIDTH, FILE_A, FILE_H, NB_SQUARES},
        piece::{get_piece, PTYPE_PAWN},
        square, Color,
    },
    game::Position,
    moves::piece_attacks,
    utils::bitboard::{bitboard_of, pop_right, CLEAR_FILES},
};

const MASK3: u64 = (1 << 3) - 1;

/// e.g. e4, white -> mask[[d5:f8]]
const PASSED_MASKS: [[u64; NB_SQUARES]; 2] = passed_masks();
const DOUBLED_MASKS: [u64; NB_SQUARES] = doubled_masks();

const BONUS_PASSER: i32 = 10;

const MALUS_BACKWARDS: i32 = 8;
const MALUS_DOUBLED: i32 = 20;
const MALUS_ISLAND: i32 = 4;

pub(crate) fn eval_pawns(pos: &Position, color: Color) -> i32 {
    let own_pawns = pos.piece_occupancy(PTYPE_PAWN, color);
    let enemy_pawns = pos.piece_occupancy(PTYPE_PAWN, color.reverse());
    let mut bb = own_pawns;
    let mut count = 0i32;
    let mut doubled_count = 0;
    let mut islands_map: [bool; BOARD_WIDTH] = [false; BOARD_WIDTH];

    while bb != 0u64 {
        let sq = pop_right(&mut bb);
        let rank = rank_of(sq);
        islands_map[file_of(sq)] = true;

        if is_passed(sq, color, enemy_pawns) {
            if is_protected(sq, color, own_pawns) {
                count += protected_passer_value(rank, color);
            } else {
                count += BONUS_PASSER;
            }

            continue;
        }

        if is_doubled(sq, own_pawns) {
            if doubled_count > 1 {
                count -= MALUS_DOUBLED;
            }
            doubled_count += 1;
            continue;
        }

        if is_backwards(sq, rank, color, own_pawns) {
            count -= MALUS_BACKWARDS;
        }
    }

    count -= pawn_island_malus(islands_map);
    count
}

const fn passed_masks() -> [[u64; NB_SQUARES]; 2] {
    let mut masks: [[u64; NB_SQUARES]; 2] = [[0u64; NB_SQUARES]; 2];
    let mut sq = square::A2;

    while sq <= square::H7 {
        masks[Color::White as usize][sq] = passed_mask(sq, Color::White);
        masks[Color::Black as usize][sq] = passed_mask(sq, Color::Black);
        sq += 1;
    }

    masks
}

const fn passed_mask(sq: usize, color: Color) -> u64 {
    let promotion_rank = color.reverse().initial_piece_rank();
    let file = file_of(sq);
    let base_mask = match file {
        FILE_A => MASK3 >> 1,
        FILE_H => MASK3 << (file - 1) & CLEAR_FILES[FILE_A],
        _ => MASK3 << (file - 1),
    };

    let mut passed_mask = 0u64;
    let mut rank = rank_of(sq);

    loop {
        match color {
            Color::White => {
                rank += 1;
            }
            Color::Black => {
                rank -= 1;
            }
        };

        passed_mask |= base_mask << (rank * BOARD_WIDTH);

        if rank == promotion_rank {
            break;
        }
    }

    passed_mask
}

const fn doubled_masks() -> [u64; NB_SQUARES] {
    let mut masks: [u64; NB_SQUARES] = [0u64; NB_SQUARES];
    let mut sq = square::A2;

    while sq <= square::H7 {
        let bb = bitboard_of(sq);
        masks[sq] = bb | bitboard_of(sq + BOARD_WIDTH);
        sq += 1;
    }

    masks
}

const fn is_passed(sq: usize, color: Color, enemy_pawns: u64) -> bool {
    PASSED_MASKS[color as usize][sq] & enemy_pawns == 0u64
}

fn is_protected(sq: usize, color: Color, own_pawns: u64) -> bool {
    let enemy_pawn = get_piece(PTYPE_PAWN, color.reverse());
    let protector_mask = piece_attacks(enemy_pawn, sq, 0u64);
    protector_mask & own_pawns != 0u64
}

const fn is_backwards(sq: usize, rank: usize, color: Color, own_pawns: u64) -> bool {
    if rank == color.initial_pawn_rank() {
        return false;
    }

    let back_mask = PASSED_MASKS[color.reverse() as usize][sq];
    back_mask & own_pawns == 0u64
}

const fn is_doubled(sq: usize, own_pawns: u64) -> bool {
    let doubled_mask = DOUBLED_MASKS[sq];
    own_pawns & doubled_mask == doubled_mask
}

fn protected_passer_value(rank: usize, color: Color) -> i32 {
    const PROTECTED_PASSER_FACTORS: [[i32; BOARD_WIDTH]; 2] =
        [[0, 0, 1, 1, 3, 4, 6, 0], [0, 6, 4, 3, 1, 1, 0, 0]];

    BONUS_PASSER * PROTECTED_PASSER_FACTORS[color as usize][rank]
}

fn pawn_island_malus(islands_map: [bool; BOARD_WIDTH]) -> i32 {
    let mut in_island = false;
    let mut count = 0;

    for has_pawn_on_file in islands_map {
        if !has_pawn_on_file {
            in_island = false;
            continue;
        }

        if !in_island {
            in_island = true;
            count += 1
        }
    }

    // To not add malus for a single pawn island.
    if count == 1 {
        count = 0;
    }

    count * MALUS_ISLAND
}
