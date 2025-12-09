use crate::{
    bit_boards::set_bits,
    game::moves::{MoveList, encoding::normal_move, piece_attacks},
};

pub(crate) const fn figure_moves(
    pos: &super::Position,
    moves: &mut MoveList,
    attack_mask: u64,
    piece: usize,
    src_sq: usize,
) {
    let bb = piece_attacks(piece, src_sq, pos.full_occupancy()) & attack_mask;

    set_bits!(bb, dest_sq, {
        super::add_move!(
            pos,
            moves,
            normal_move(src_sq, dest_sq, piece, pos.get_piece(dest_sq))
        );
    });
}
