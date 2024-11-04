use core::f64;
use std::collections::HashMap;

use crate::game::Position;

use super::eval_position::evaluate_position;

const MAX_DEPTH: usize = 7;
const MAX_SCORE: f64 = 1_000_000.0;
const MIN_SCORE: f64 = -MAX_SCORE;
const FACTORS: [i32; 2] = [1, -1];

pub(crate) fn eval(pos: &mut Position) -> EvalTuple {
    let mut memo = HashMap::<u64, f64>::new();
    let moves: MoveVector = Vec::new();
    let ev = eval_recursive(pos, 0, MIN_SCORE, MAX_SCORE, &mut memo, moves);
    ev
}

fn eval_recursive(
    pos: &mut Position,
    depth: usize,
    mut alpha: f64,
    beta: f64,
    memo: &mut HashMap<u64, f64>,
    moves: MoveVector,
) -> EvalTuple {
    let key = pos.get_hash();

    if memo.contains_key(&key) {
        let score = *memo.get(&key).unwrap();
        return (score, moves);
    }

    let active_color = pos.get_active_color();
    let legal_moves = pos.legal_moves();

    if legal_moves.is_empty() {
        let score = if pos.is_check() { MIN_SCORE } else { 0.0 };
        memo.insert(key, score);
        return (score, moves);
    }

    if depth == MAX_DEPTH {
        let mp_score = evaluate_position(pos) * FACTORS[active_color as usize];
        return ((mp_score as f64) / 1000.0, moves);
    }

    let undo_info = pos.undo_move_info();
    let mut best_score = f64::NEG_INFINITY;
    let mut best_moves_opt: Option<MoveVector> = None;

    for mv in legal_moves {
        pos.play_move(mv);

        let (mv_score, line) =
            eval_recursive(pos, depth + 1, -beta, -alpha, memo, add_move(&moves, mv));
        let mv_score = adjust_move_score(mv_score, depth);

        pos.undo_move(mv, undo_info);

        if mv_score > best_score {
            best_score = mv_score;
            best_moves_opt = Some(line);

            if best_score > alpha {
                alpha = best_score;
            }

            if alpha >= beta {
                break;
            }
        }
    }

    (best_score, best_moves_opt.unwrap())
}

fn add_move(moves: &MoveVector, mv: u32) -> MoveVector {
    let mut clone = moves.clone();
    clone.push(mv);
    clone
}

fn adjust_move_score(mv_score: f64, depth: usize) -> f64 {
    if mv_score == MIN_SCORE {
        return -mv_score - depth as f64;
    }

    -mv_score
}

type MoveVector = Vec<u32>;
type EvalTuple = (f64, MoveVector);
