use crate::{
    engine::{check_or_stalemate, score::Score, static_eval::eval_position},
    game::{
        moves::{MoveList, encoding},
        position::Position,
    },
};

pub(crate) fn quiesce(
    pos: &mut Position,
    ply: usize,
    mut alpha: Score,
    beta: Score,
    moves: Option<MoveList>,
) -> Score {
    let mut best_score = eval_position(pos);

    if best_score >= beta {
        return best_score;
    }

    if best_score > alpha {
        alpha = best_score;
    }

    let undo_info = pos.undo_info();
    let mut moves = moves.unwrap_or_else(|| pos.legal_moves());

    if moves.is_empty() {
        return check_or_stalemate(pos.is_check(), ply);
    }

    moves.retain(encoding::is_capture);
    super::move_ordering::sort_captures(&mut moves);

    for &mv in &moves {
        pos.play_move(mv);
        let mv_score = -quiesce(pos, ply + 1, -beta, -alpha, None);
        pos.undo_move(mv, undo_info);

        if mv_score >= beta {
            return mv_score;
        }

        if mv_score > best_score {
            best_score = mv_score;

            if best_score > alpha {
                alpha = best_score;
            }
        }
    }

    best_score
}
