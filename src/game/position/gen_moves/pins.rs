use crate::game::board::{
    directions as dirs,
    pieces::{self, piece_types},
};

use super::Position;

pub(crate) const fn get_pin_mask(
    pos: &Position,
    king_sq: usize,
    piece_sq: usize,
    enemy_color: usize,
) -> u64 {
    let next_piece = find_next_piece(pos, king_sq, piece_sq);

    if let Some((pinner, pinner_sq, dir)) = next_piece {
        if pieces::color_of(pinner) == enemy_color && can_pin(pinner, dir) {
            return dirs::ray_of(king_sq, dir) & !dirs::ray_of(pinner_sq, dir);
        }
    }

    u64::MAX
}

/// If two squares are orthogonally or diagonally aligned,
/// returns the next piece along the same direction, its square and the direction.
pub(crate) const fn find_next_piece(
    pos: &Position,
    sq1: usize,
    sq2: usize,
) -> Option<(usize, usize, usize)> {
    let dir = dirs::get(sq1, sq2);

    if dir != dirs::NONE {
        let ray_occ = pos.full_occupancy() & dirs::ray_of(sq2, dir);

        if ray_occ != 0 {
            let sq3 = dirs::first_occupied_square(ray_occ, dir);
            let piece = pos.get_piece(sq3);
            return Some((piece, sq3, dir));
        }
    }

    None
}

pub(crate) const fn can_pin(piece: usize, dir: usize) -> bool {
    match pieces::type_of(piece) {
        piece_types::BISHOP => dirs::is_diagonal(dir),
        piece_types::ROOK => dirs::is_orthogonal(dir),
        piece_types::QUEEN => true,
        _ => false,
    }
}
