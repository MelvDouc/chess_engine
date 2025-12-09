use crate::{
    engine::{score::Score, transposition as tp},
    game::position::Position,
    macros::ternary,
};

const REDUCTION: usize = 3;

const fn can_prune_null_move(pos: &Position, depth: usize) -> bool {
    if depth < REDUCTION || pos.is_check() {
        return false;
    }

    let color = pos.get_active_color();
    pos.color_occupancy(color) & !(pos.king_occupancy(color) | pos.pawn_occupancy(color)) != 0
}

pub(super) fn prune_null_move(
    pos: &mut Position,
    tt: &mut tp::Table,
    kmt: &mut super::killer_moves::Table,
    ply: usize,
    depth: usize,
    beta: Score,
) -> Option<Score> {
    if !can_prune_null_move(pos, depth) {
        return None;
    }

    let ep_sq = pos.get_ep_square();
    pos.play_null_move();
    let score = -super::negamax(pos, tt, kmt, ply + 1, depth - REDUCTION, -beta, -beta + 1);
    pos.undo_null_move(ep_sq);

    ternary!(score >= beta, Some(score), None)
}
