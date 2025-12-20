use crate::{engine::MAX_DEPTH, macros::ternary};

pub(crate) type Score = i32;

pub(crate) const MATE_SCORE: Score = 1_000_000;
pub(crate) const DRAW_SCORE: Score = 0;

pub(crate) const fn is_mate_score(score: Score) -> bool {
    score.abs() >= MATE_SCORE - MAX_DEPTH as Score
}

pub(crate) const fn score_to_tt(score: Score, ply: usize) -> Score {
    if score > MATE_SCORE {
        return score + ply as Score;
    }

    if score < -MATE_SCORE {
        return score - ply as Score;
    }

    score
}

pub(crate) const fn score_from_tt(score: Score, ply: usize) -> Score {
    if score > MATE_SCORE {
        return score - ply as Score;
    }

    if score < -MATE_SCORE {
        return score + ply as Score;
    }

    score
}

pub(super) fn stringify_score(score: Score) -> String {
    if score == 0 {
        return "0".to_string();
    }

    let sign = ternary!(score > 0, '+', '-');
    let abs_score = score.abs() as f32;

    // if is_mate_score(score) {
    //     let dist_to_mate = ((MATE_SCORE as f32 - abs_score) / 2.0).ceil();
    //     return format!("{}M{}", sign, dist_to_mate as u8);
    // }

    format!("{}{:.2}", sign, abs_score / 1000.0)
}
