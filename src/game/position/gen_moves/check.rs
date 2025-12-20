use crate::{
    bit_boards::{bit_mask, is_bit_set, set_bits},
    game::{
        board::{directions as dirs, pieces, squares},
        moves::{Move, castling, encoding, piece_attacks},
    },
    macros::ternary,
};

use super::{
    Position,
    pins::{can_pin, find_next_piece},
};

pub(crate) enum CheckType {
    None,
    Single(u64),
    Double,
}

impl CheckType {
    pub(crate) const fn get(pos: &Position, king_sq: usize) -> Self {
        let full_occ = pos.full_occupancy();
        let mut checker1: Option<(usize, usize)> = None;
        let mut checker2 = false;

        set_bits!(pos.inactive_occupancy(), sq, {
            let piece = pos.get_piece(sq);

            if is_bit_set(piece_attacks(piece, sq, full_occ), king_sq) {
                if checker1.is_none() {
                    checker1 = Some((piece, sq));
                } else {
                    checker2 = true;
                    break;
                }
            }
        });

        if checker2 {
            return Self::Double;
        }

        match checker1 {
            Some((piece, sq)) => {
                if pieces::is_slider(piece) {
                    let dir = dirs::get(king_sq, sq);
                    let ray = dirs::ray_of(king_sq, dir) & !dirs::ray_of(sq, dir);
                    Self::Single(ray)
                } else {
                    Self::Single(bit_mask(sq))
                }
            }
            None => Self::None,
        }
    }

    pub(crate) const fn is_none(&self) -> bool {
        match self {
            CheckType::None => true,
            _ => false,
        }
    }
}

pub(crate) const fn gives_check(pos: &Position, mv: Move) -> bool {
    let enemy_king_sq = pos.king_square(pos.inactive_color());
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let src_piece = encoding::src_piece(mv);
    let color = pieces::color_of(src_piece);
    let mut occ = pos.full_occupancy() & !bit_mask(src_sq) | bit_mask(dest_sq);

    if encoding::is_castling(mv) {
        let wing = castling::get_wing(src_sq, dest_sq);
        let rook_src_sq = castling::rook_src_square(color, wing);
        let rook_dest_sq = castling::rook_dest_square(color, wing);
        occ = occ & !bit_mask(rook_src_sq) | bit_mask(rook_dest_sq);

        return is_bit_set(
            piece_attacks(pieces::WHITE_ROOK, rook_dest_sq, occ),
            enemy_king_sq,
        );
    }

    if encoding::is_en_passant(mv) {
        let capture_sq = squares::ep_capture_square(src_sq, dest_sq);
        occ &= !bit_mask(capture_sq);
    }

    let dest_piece = ternary!(
        encoding::is_promotion(mv),
        encoding::promoted(mv),
        src_piece
    );

    if is_bit_set(piece_attacks(dest_piece, dest_sq, occ), enemy_king_sq) {
        return true;
    }

    // discovered check
    if let Some((checker, _, dir)) = find_next_piece(pos, enemy_king_sq, src_sq) {
        return pieces::color_of(checker) == color && can_pin(checker, dir);
    }

    false
}
