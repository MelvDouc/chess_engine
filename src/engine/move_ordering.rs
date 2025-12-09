use crate::{
    engine::{score::Score, static_eval::piece_value},
    game::moves::{Move, MoveList, encoding},
};

pub(crate) fn sort_moves(moves: &mut MoveList, km: &super::killer_moves::Pair) {
    moves
        .as_mut_slice()
        .sort_by(|&a, &b| move_score(b, km).cmp(&move_score(a, km)));
}

pub(crate) fn sort_captures(moves: &mut MoveList) {
    moves
        .as_mut_slice()
        .sort_by(|&a, &b| move_score_raw(b).cmp(&move_score_raw(a)));
}

pub(crate) const fn is_quiet_move(mv: Move) -> bool {
    !encoding::gives_check(mv) && !encoding::is_capture(mv) && !encoding::is_promotion(mv)
}

const fn move_score(mv: Move, km: &super::killer_moves::Pair) -> Score {
    let mut score = move_score_raw(mv);

    if encoding::gives_check(mv) {
        score += 5_000_000;
    }

    if encoding::is_promotion(mv) {
        score += 1_000_000;
    }

    if score < 1_000_000 && (mv == km.0 || mv == km.1) {
        score += 500_000;
    }

    score
}

const fn move_score_raw(mv: Move) -> Score {
    10_000 - piece_value(encoding::src_piece(mv)) + 10 * piece_value(encoding::captured(mv))
}

#[cfg(test)]
mod tests {
    use crate::game::{moves::NULL_MOVE, position::Position};

    use super::*;

    #[test]
    fn sort_moves() {
        let pos = Position::from_fen("r3k3/1P6/8/4pP2/8/2Q5/8/4K2R w K e6 0 1").unwrap();
        let mut moves = pos.legal_moves();
        super::sort_moves(&mut moves, &(NULL_MOVE, NULL_MOVE));

        for i in 0..(moves.len() - 1) {
            let mv1 = moves[i];
            let mv2 = moves[i + 1];

            if !encoding::gives_check(mv1) {
                assert!(!encoding::gives_check(mv2));
            }
        }
    }
}
