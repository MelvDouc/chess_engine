use crate::{
    bit_boards::{bit_mask, is_bit_set, set_bits},
    game::{
        board::{
            directions as dirs,
            pieces::{self, piece_types},
            squares,
        },
        moves::{
            Move,
            castling::{get_wing, rook_dest_square},
            encoding, piece_attacks,
        },
        position::Position,
    },
    macros::ternary,
};

pub(crate) enum CheckType {
    None,
    Single(u64),
    Double,
}

impl CheckType {
    pub(crate) const fn get(pos: &Position) -> Self {
        let full_occ = pos.full_occupancy();
        let king_sq = pos.king_square(pos.active_color);
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

pub(crate) const fn gives_check(pos: &Position, enemy_king_sq: usize, mv: Move) -> bool {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let src_piece = encoding::src_piece(mv);
    let mut occ = pos.full_occupancy() & !bit_mask(src_sq) | bit_mask(dest_sq);

    if encoding::is_castling(mv) {
        let color = pieces::color_of(src_piece);
        let wing = get_wing(src_sq, dest_sq);
        let rook_dest_sq = rook_dest_square(color, wing);

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

    if !pieces::is_queen(src_piece) {
        if !pieces::is_bishop(src_piece) {
            return is_discovered_check(pos, occ, enemy_king_sq, piece_types::BISHOP);
        }

        if !pieces::is_rook(src_piece) {
            return is_discovered_check(pos, occ, enemy_king_sq, piece_types::ROOK);
        }
    }

    false
}

const fn is_discovered_check(
    pos: &Position,
    occ: u64,
    enemy_king_sq: usize,
    slider_type: usize,
) -> bool {
    let color = pos.active_color;
    let slider = pieces::of(slider_type, color);
    let slider_occ = pos.piece_occupancy(slider) | pos.queen_occupancy(color);

    return piece_attacks(slider, enemy_king_sq, occ) & slider_occ != 0;
}
