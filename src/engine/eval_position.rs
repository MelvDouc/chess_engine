/*
 * The material difference should serve as the base for the evaluation.
 * The pawn quality should be taken into account:
 * - every passed pawn yields a bonus;
 * - the above bonus should be increased for protected passers the more advanced they are;
 * - every backwards pawn yield a malus;
 * - doubled pawns yield a malus when there is more than 1 set of them;
 * - the fewer pawn islands, the better.
 * A bonus should be (always?) added for the bishop pair.
 */

use pawns::eval_pawns;

use crate::{
    constants::{
        board_constants::{is_dark_sq, NB_PIECE_TYPES},
        piece::{get_piece, PTYPE_BISHOP, PTYPE_KNIGHT, PTYPE_PAWN, PTYPE_QUEEN, PTYPE_ROOK},
        Color,
    },
    game::Position,
    utils::bitboard::pop_right,
};

mod eval_squares;
pub(crate) mod pawns;

const MASK4: u64 = (1 << 4) - 1;
const PTYPE_VALUES: [u64; NB_PIECE_TYPES] = [1, 3, 0, 3, 5, 10];

/// Scores the board in **millipawns**.
pub(super) fn evaluate_position(pos: &Position) -> i32 {
    if pos.half_move_clock >= 50 {
        return 0;
    }

    let piece_count = pos.piece_count();
    evaluate_side(pos, piece_count, Color::White) - evaluate_side(pos, piece_count, Color::Black)
}

fn evaluate_side(pos: &Position, piece_count: PieceCount, color: Color) -> i32 {
    if !has_material_to_mate(color, color.reverse(), piece_count) {
        return 0i32;
    }

    let m_count = material_count(piece_count, color);
    let mut score = m_count * 1000;

    if has_bishop_pair(pos, color) {
        score += 200;
    }

    score += eval_pawns(pos, color);

    score
}

const fn get_count(piece_count: PieceCount, piece_type: u8, color: Color) -> u64 {
    piece_count >> (4 * get_piece(piece_type, color)) & MASK4
}

fn material_count(piece_count: PieceCount, color: Color) -> i32 {
    let mut m_count = 0;

    for piece_type in PTYPE_PAWN..=PTYPE_QUEEN {
        m_count += get_count(piece_count, piece_type, color) * PTYPE_VALUES[piece_type as usize];
    }

    m_count.try_into().unwrap()
}

/// Returns whether a player has at least a pawn, rook or queen.
fn has_lone_mating_piece(color: Color, piece_count: PieceCount) -> bool {
    get_count(piece_count, PTYPE_PAWN, color) > 0
        || get_count(piece_count, PTYPE_ROOK, color) > 0
        || get_count(piece_count, PTYPE_QUEEN, color) > 0
}

fn has_non_king_piece(color: Color, piece_count: PieceCount) -> bool {
    has_lone_mating_piece(color, piece_count)
        || get_count(piece_count, PTYPE_KNIGHT, color) > 0
        || get_count(piece_count, PTYPE_BISHOP, color) > 0
}

fn has_material_to_mate(color: Color, inactive_color: Color, piece_count: PieceCount) -> bool {
    let nb_bishops = get_count(piece_count, PTYPE_BISHOP, color);

    if has_lone_mating_piece(color, piece_count) || nb_bishops > 1 {
        return true;
    }

    match get_count(piece_count, PTYPE_KNIGHT, color) {
        0 => false,
        1 => nb_bishops > 0,
        2 => nb_bishops > 0 || has_non_king_piece(inactive_color, piece_count),
        _ => true,
    }
}

fn has_bishop_pair(pos: &Position, color: Color) -> bool {
    let mut occ = pos.piece_occupancy(PTYPE_BISHOP, color);
    let mut has_lsb = false;
    let mut has_dsb = false;

    while occ != 0u64 {
        let sq = pop_right(&mut occ);

        if is_dark_sq(sq) {
            has_dsb = true
        } else {
            has_lsb = true;
        }

        if has_lsb && has_dsb {
            return true;
        }
    }

    false
}

type PieceCount = u64;
