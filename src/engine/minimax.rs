use core::f64;
use std::collections::HashMap;

use crate::{game::Position, moves::move_encoding::move_notation};

use super::{
    eval_board::evaluate_board,
    eval_encoding::{decode_depth, decode_score, encode_eval},
};

const MAX_SCORE: i32 = 1_000_000;
const MIN_SCORE: i32 = -MAX_SCORE;
const MAX_DEPTH: usize = 6;

const DEFAULT_SCORES: [i32; 2] = [MIN_SCORE, MAX_SCORE];

pub(crate) fn eval(pos: &mut Position) -> (i32, usize) {
    let mut memo = HashMap::<u64, i32>::new();
    let alpha = f64::NEG_INFINITY;
    let beta = f64::INFINITY;

    let ev = minimax(pos, 0, alpha, beta, &mut memo);
    let score = decode_score(ev);
    let depth = decode_depth(ev);

    println!("memo size: {}", memo.len());
    (score, depth)
}

fn minimax(
    pos: &mut Position,
    depth: usize,
    mut alpha: f64,
    mut beta: f64,
    memo: &mut HashMap<u64, i32>,
) -> i32 {
    let key = pos.get_hash();

    if memo.contains_key(&key) {
        let score = memo.get(&key).unwrap();
        return encode_eval(*score, depth);
    }

    let legal_moves = pos.legal_moves();

    if legal_moves.len() == 0 {
        let score = if pos.is_check() {
            DEFAULT_SCORES[pos.get_active_color() as usize]
        } else {
            0
        };
        return encode_eval(score, depth);
    }

    if depth == MAX_DEPTH {
        let score = evaluate_board(pos);
        return encode_eval(score, depth);
    }

    let maximizing = pos.is_white_to_move();
    let undo_info = pos.undo_move_info();
    let mut best_score = DEFAULT_SCORES[pos.get_active_color() as usize];
    let mut best_depth = MAX_DEPTH;
    let mut best_move = 0;

    for mv in legal_moves {
        pos.play_move(mv, true);
        let move_ev = minimax(pos, depth + 1, alpha, beta, memo);
        pos.undo_move(mv, undo_info);

        let move_score = decode_score(move_ev);
        let move_depth = decode_depth(move_ev);

        if maximizing && move_score > best_score
            || !maximizing && move_score < best_score
            || move_score == best_score && move_depth < best_depth
        {
            best_score = move_score;
            best_depth = move_depth;
            best_move = mv;
            update_alpha_beta(&mut alpha, &mut beta, best_score, maximizing);

            if alpha >= beta {
                break;
            }
        }
    }

    println!("{} (d={})", move_notation(best_move), depth);
    memo.insert(key, best_score);
    encode_eval(best_score, best_depth)
}

fn update_alpha_beta(alpha: &mut f64, beta: &mut f64, best_score: i32, maximizing: bool) {
    if maximizing && best_score > (*alpha) as i32 {
        *alpha = best_score as f64;
        return;
    }

    if !maximizing && best_score < (*beta) as i32 {
        *beta = best_score as f64;
    }
}
