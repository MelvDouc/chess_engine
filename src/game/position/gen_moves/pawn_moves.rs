use crate::{
    bit_boards::{is_bit_set, set_bits},
    game::{
        board::{NB_COLORS, colors, directions as dirs, pieces, squares},
        moves::{MoveList, encoding, piece_attacks},
        position::Position,
    },
    macros::const_while,
};

pub(crate) const fn pawn_moves(
    pos: &Position,
    moves: &mut MoveList,
    pin_check_mask: u64,
    pawn: usize,
    src_sq: usize,
) {
    add_pawn_captures(pos, moves, pin_check_mask, pawn, src_sq);
    add_pawn_pushes(pos, moves, pin_check_mask, pawn, src_sq);
}

const fn add_pawn_pushes(
    pos: &Position,
    moves: &mut MoveList,
    pin_check_mask: u64,
    pawn: usize,
    src_sq: usize,
) {
    let dest_sq = push_dest_square(src_sq, pos.active_color);

    if is_bit_set(pos.full_occupancy(), dest_sq) {
        return;
    }

    if is_bit_set(pin_check_mask, dest_sq) {
        if is_promotion(dest_sq, pos.active_color) {
            add_promotions(pos, moves, src_sq, dest_sq, pawn, pieces::NONE);
            return;
        }

        super::add_move(
            pos,
            moves,
            encoding::normal_move(src_sq, dest_sq, pawn, pieces::NONE),
        );
    }

    if squares::rank_of(src_sq) == colors::pawn_rank(pos.active_color) {
        let dest_sq = push_dest_square(dest_sq, pos.active_color);

        if is_bit_set(!pos.full_occupancy() & pin_check_mask, dest_sq) {
            super::add_move(
                pos,
                moves,
                encoding::normal_move(src_sq, dest_sq, pawn, pieces::NONE),
            );
        }
    }
}

const fn push_dest_square(sq: usize, color: usize) -> usize {
    const DIRECTIONS: [usize; NB_COLORS] = [dirs::NORTH, dirs::SOUTH];

    dirs::next_square(sq, DIRECTIONS[color])
}

const fn add_pawn_captures(
    pos: &Position,
    moves: &mut MoveList,
    pin_check_mask: u64,
    pawn: usize,
    src_sq: usize,
) {
    let bb = piece_attacks(pawn, src_sq, 0) & pin_check_mask;

    if pos.en_passant_sq != squares::NONE && is_bit_set(bb, pos.en_passant_sq) {
        let captured = pieces::rev_color(pawn);
        super::add_move(
            pos,
            moves,
            encoding::en_passant_move(src_sq, pos.en_passant_sq, pawn, captured),
        );
    }

    set_bits!(bb & pos.inactive_occupancy(), dest_sq, {
        let captured = pos.get_piece(dest_sq);

        if !is_promotion(dest_sq, pos.active_color) {
            super::add_move(
                pos,
                moves,
                encoding::normal_move(src_sq, dest_sq, pawn, captured),
            );
            continue;
        }

        add_promotions(pos, moves, src_sq, dest_sq, pawn, captured);
    });
}

const fn is_promotion(dest_sq: usize, color: usize) -> bool {
    squares::rank_of(dest_sq) == colors::piece_rank(colors::rev(color))
}

const fn add_promotions(
    pos: &Position,
    moves: &mut MoveList,
    src_sq: usize,
    dest_sq: usize,
    pawn: usize,
    captured: usize,
) {
    /// Sorted by frequency.
    const PROMOTION_TYPES: [usize; 4] = [
        pieces::piece_types::QUEEN,
        pieces::piece_types::KNIGHT,
        pieces::piece_types::ROOK,
        pieces::piece_types::BISHOP,
    ];

    const_while!(i, 0, PROMOTION_TYPES.len(), {
        let promoted = pieces::of(PROMOTION_TYPES[i], pos.active_color);
        super::add_move(
            pos,
            moves,
            encoding::promotion_move(src_sq, dest_sq, pawn, captured, promoted),
        );
    });
}
