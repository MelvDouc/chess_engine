use crate::{
    game::{
        board::{NB_WINGS, pieces},
        moves::{
            MoveList,
            castling::{can_castle_to_wing, king_dest_square, king_src_square},
            encoding::castling_move,
        },
    },
    macros::const_while,
};

pub(crate) const fn castling_moves(
    pos: &super::Position,
    moves: &mut MoveList,
    enemy_attacks: u64,
) {
    let color = pos.active_color;

    const_while!(wing, 0, NB_WINGS, {
        if can_castle_to_wing(
            pos.castling_rights,
            color,
            wing,
            pos.full_occupancy(),
            enemy_attacks,
        ) {
            let king_src_sq = king_src_square(color);
            let king_dest_sq = king_dest_square(color, wing);
            super::add_move(
                pos,
                moves,
                castling_move(king_src_sq, king_dest_sq, pieces::king_of(color)),
            );
        }
    });
}
