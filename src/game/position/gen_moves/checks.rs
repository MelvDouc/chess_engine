use crate::{
    bit_boards::{bit_mask, is_bit_set, set_bits},
    game::{
        board::{directions as dirs, pieces, squares},
        moves::{Move, castling, encoding, piece_attacks},
    },
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
                    continue;
                }

                checker2 = true;
                break;
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

    pub(crate) const fn get_mask(&self) -> u64 {
        match self {
            CheckType::None => u64::MAX,
            CheckType::Single(mask) => *mask,
            CheckType::Double => 0,
        }
    }
}

pub(crate) const fn gives_check(pos: &Position, enemy_king_sq: usize, mv: Move) -> bool {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let mut src_piece = encoding::src_piece(mv);
    let mut occ = pos.full_occupancy() & !bit_mask(src_sq) | bit_mask(dest_sq);

    match encoding::move_kind(mv) {
        encoding::move_kinds::EN_PASSANT => {
            let capture_sq = squares::ep_capture_square(src_sq, dest_sq);
            occ &= !bit_mask(capture_sq);
        }
        encoding::move_kinds::PROMOTION => {
            src_piece = encoding::promoted(mv);
        }
        encoding::move_kinds::CASTLING => {
            let wing = castling::get_wing(src_sq, dest_sq);
            let rook_src_sq = castling::rook_src_square(pos.active_color, wing);
            let rook_dest_sq = castling::rook_dest_square(pos.active_color, wing);
            occ = occ & !bit_mask(rook_src_sq) | bit_mask(rook_dest_sq);

            return is_bit_set(
                piece_attacks(pieces::WHITE_ROOK, rook_dest_sq, occ),
                enemy_king_sq,
            );
        }
        _ => {}
    };

    if is_bit_set(piece_attacks(src_piece, dest_sq, occ), enemy_king_sq) {
        return true;
    }

    // discovered check
    if let Some((checker, _, dir)) = find_next_piece(pos, enemy_king_sq, src_sq) {
        return pieces::color_of(checker) == pos.active_color && can_pin(checker, dir);
    }

    false
}
