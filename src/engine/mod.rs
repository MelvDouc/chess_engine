use crate::{
    engine::{null_move_pruning::prune_null_move, score::*, transposition as tp},
    game::{
        moves::{Move, NULL_MOVE},
        position::Position,
    },
    macros::ternary,
};

mod killer_moves;
mod move_ordering;
mod null_move_pruning;
mod pv;
mod quiescence;
mod score;
mod static_eval;
mod transposition;

const MAX_DEPTH: usize = 255;
const DELTA: Score = 500;

pub(crate) fn run(pos: &mut Position, max_depth: usize, print_pv: bool) {
    let mut tt = tp::create_table();
    let mut kmt = killer_moves::create_table();
    let mut prev_score = 0;
    let mut delta = 250;

    for depth in 1..=max_depth {
        let score = ternary!(
            depth <= 4,
            analyze(pos, &mut tt, &mut kmt, 0, depth, -MATE_SCORE, MATE_SCORE),
            analyze_aspiration_windows(
                pos,
                &mut tt,
                &mut kmt,
                depth,
                prev_score - delta,
                prev_score + delta
            )
        );
        prev_score = score;

        if depth % 4 == 0 {
            delta += 250;
        }

        if print_pv {
            println!(
                "{} {} {}",
                depth,
                stringify_score(score),
                pv::stringify(pos, &tt, depth)
            );
        }
    }
}

fn analyze(
    pos: &mut Position,
    tt: &mut tp::Table,
    kmt: &mut killer_moves::Table,
    ply: usize,
    depth: usize,
    alpha: Score,
    beta: Score,
) -> Score {
    let score = negamax(pos, tt, kmt, ply, depth, alpha, beta);
    pos.reset_reps();
    score
}

fn analyze_aspiration_windows(
    pos: &mut Position,
    tt: &mut tp::Table,
    kmt: &mut killer_moves::Table,
    depth: usize,
    mut alpha: Score,
    mut beta: Score,
) -> Score {
    let mut score: Score = 0;

    for _ in 0..2 {
        score = analyze(pos, tt, kmt, 0, depth, alpha, beta);

        if score <= alpha {
            alpha = -MATE_SCORE;
        } else if score >= beta {
            beta = MATE_SCORE;
        } else {
            break;
        }
    }

    score
}

macro_rules! set_exact {
    ($tt: expr, $hash: expr, $depth: expr, $score: expr) => {
        let entry = tp::Entry::exact($hash, $score, $depth, NULL_MOVE);
        tp::set_entry($tt, entry);
        return entry.score;
    };
}

fn negamax(
    pos: &mut Position,
    tt: &mut tp::Table,
    kmt: &mut killer_moves::Table,
    ply: usize,
    depth: usize,
    mut alpha: Score,
    mut beta: Score,
) -> Score {
    let old_alpha = alpha;
    let hash = pos.hash();

    if let Some(score) = tp::cached_score(tt, hash, depth, ply, &mut alpha, &mut beta) {
        return score;
    }

    if pos.half_move_clock() >= 50 || pos.rep_count() >= 2 || pos.piece_count() == 2 {
        set_exact!(tt, hash, depth, DRAW_SCORE);
    }

    let mut moves = pos.legal_moves();

    if moves.is_empty() {
        let score = ternary!(pos.is_check(), -MATE_SCORE, DRAW_SCORE);
        set_exact!(tt, hash, depth, score_to_tt(score, ply));
    }

    if depth == 0 {
        set_exact!(
            tt,
            hash,
            depth,
            quiescence::quiesce(pos, alpha, beta, Some(moves))
        );
    }

    if let Some(score) = prune_null_move(pos, tt, kmt, ply, depth, beta) {
        return score;
    }

    move_ordering::sort_moves(&mut moves, &kmt[depth]);
    let undo_info = pos.undo_info();
    let mut best_score = Score::MIN;
    let mut best_mv = NULL_MOVE;

    for &mv in &moves {
        pos.play_move(mv);
        let mv_score = move_score(pos, tt, kmt, ply, depth, alpha, beta, mv);
        pos.undo_move(mv, undo_info);

        if mv_score <= best_score {
            continue;
        }

        best_score = mv_score;
        best_mv = mv;

        if move_ordering::is_quiet_move(mv) {
            killer_moves::update(kmt, mv, depth);
        }

        if best_score > alpha {
            alpha = best_score;

            if alpha >= beta {
                break;
            }
        }
    }

    let flag = tp::flags::get_flag(old_alpha, beta, best_score);
    let entry = tp::Entry::new(flag, hash, best_score, depth, best_mv);
    tp::set_entry(tt, entry);
    best_score
}

fn move_score(
    pos: &mut Position,
    tt: &mut tp::Table,
    kmt: &mut killer_moves::Table,
    ply: usize,
    depth: usize,
    alpha: Score,
    beta: Score,
    mv: Move,
) -> Score {
    if depth >= 2 && move_ordering::is_quiet_move(mv) {
        let mv_score = -negamax(pos, tt, kmt, ply + 1, depth - 2, -alpha - 1, -alpha);

        if mv_score <= alpha {
            return mv_score;
        }
    }

    -negamax(pos, tt, kmt, ply + 1, depth - 1, -beta, -alpha)
}
