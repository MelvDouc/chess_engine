use crate::{
    bit_boards::mask_consecutive,
    game::board::{NB_COLORS, NB_WINGS, squares, wings},
    macros::ternary,
};

pub(crate) const NB_BITS_CASTLING_RIGHTS: usize = NB_COLORS * NB_WINGS;

pub(crate) const fn king_src_square(color: usize) -> usize {
    const SQUARES: [usize; NB_COLORS] = [squares::E1, squares::E8];

    SQUARES[color]
}

pub(crate) const fn king_dest_square(color: usize, wing: usize) -> usize {
    const SQUARES: [[usize; NB_WINGS]; NB_COLORS] =
        [[squares::C1, squares::G1], [squares::C8, squares::G8]];

    SQUARES[color][wing]
}

pub(crate) const fn rook_src_square(color: usize, wing: usize) -> usize {
    const SQUARES: [[usize; NB_WINGS]; NB_COLORS] =
        [[squares::A1, squares::H1], [squares::A8, squares::H8]];

    SQUARES[color][wing]
}

pub(crate) const fn rook_dest_square(color: usize, wing: usize) -> usize {
    const SQUARES: [[usize; NB_WINGS]; NB_COLORS] =
        [[squares::D1, squares::F1], [squares::D8, squares::F8]];

    SQUARES[color][wing]
}

pub(crate) const fn get_wing(king_src_sq: usize, king_dest_sq: usize) -> usize {
    ternary!(
        king_dest_sq < king_src_sq,
        wings::QUEEN_SIDE,
        wings::KING_SIDE
    )
}

pub(crate) const fn castling_bit(color: usize, wing: usize) -> u8 {
    const CASTLING_BITS: [[u8; NB_WINGS]; NB_COLORS] = [[1, 1 << 1], [1 << 2, 1 << 3]];

    CASTLING_BITS[color][wing]
}

pub(crate) const fn castling_color_mask(color: usize) -> u8 {
    castling_bit(color, wings::QUEEN_SIDE) | castling_bit(color, wings::KING_SIDE)
}

pub(crate) const fn has_castling_right(castling_rights: u8, color: usize, wing: usize) -> bool {
    castling_rights & castling_bit(color, wing) != 0
}

pub(crate) const fn can_castle_to_wing(
    castling_rights: u8,
    color: usize,
    wing: usize,
    full_occ: u64,
    enemy_attacks: u64,
) -> bool {
    /// Masks of squares that must be free during castling.
    const OCC_MASKS: [[u64; NB_WINGS]; NB_COLORS] = [
        [
            mask_consecutive(squares::B1, squares::D1),
            mask_consecutive(squares::F1, squares::G1),
        ],
        [
            mask_consecutive(squares::B8, squares::D8),
            mask_consecutive(squares::F8, squares::G8),
        ],
    ];

    /// Masks of squares that must not be attacked by the opponent during castling.
    const ATTACK_MASKS: [[u64; NB_WINGS]; NB_COLORS] = [
        [
            mask_consecutive(squares::C1, squares::D1),
            mask_consecutive(squares::F1, squares::G1),
        ],
        [
            mask_consecutive(squares::C8, squares::D8),
            mask_consecutive(squares::F8, squares::G8),
        ],
    ];

    has_castling_right(castling_rights, color, wing)
        && (full_occ & OCC_MASKS[color][wing] == 0)
        && (enemy_attacks & ATTACK_MASKS[color][wing] == 0)
}
