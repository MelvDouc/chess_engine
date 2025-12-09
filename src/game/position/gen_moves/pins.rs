use crate::game::board::{
    Board, directions as dirs,
    pieces::{self, piece_types},
};

pub(crate) const fn get_pin_mask(
    board: &Board,
    full_occ: u64,
    king_sq: usize,
    piece_sq: usize,
    enemy_color: usize,
) -> u64 {
    let dir = dirs::get(king_sq, piece_sq);

    if dir == dirs::NONE {
        return u64::MAX;
    }

    if let Some((pinner, pinner_sq)) = find_piece(board, full_occ, piece_sq, dir, enemy_color) {
        if can_pin(pinner, dir) {
            return dirs::ray_of(king_sq, dir) & !dirs::ray_of(pinner_sq, dir);
        }
    }

    u64::MAX
}

const fn find_piece(
    board: &Board,
    full_occ: u64,
    sq: usize,
    dir: usize,
    color: usize,
) -> Option<(usize, usize)> {
    let ray_occ = full_occ & dirs::ray_of(sq, dir);

    if ray_occ != 0 {
        let sq = dirs::first_occupied_square(ray_occ, dir);
        let piece = board[sq];

        if pieces::color_of(piece) == color {
            return Some((piece, sq));
        }
    }

    None
}

const fn can_pin(piece: usize, dir: usize) -> bool {
    match pieces::type_of(piece) {
        piece_types::BISHOP => dirs::is_diagonal(dir),
        piece_types::ROOK => dirs::is_orthogonal(dir),
        piece_types::QUEEN => true,
        _ => false,
    }
}
