use rand::{Rng, rng};

use crate::game::{
    board::{pieces, squares},
    moves::{Move, MoveList, encoding},
    position::{Position, undo_info::UndoInfo},
};

mod castling;
mod pawn_moves;

pub(self) fn filter_move_kind(pos: &mut Position, mv_kind: u32) -> MoveList {
    let mut moves = pos.legal_moves();
    moves.retain(|mv| encoding::move_kind(mv) == mv_kind);
    moves
}

pub(self) fn from_fen(fen: &str) -> Position {
    Position::from_fen(fen).unwrap()
}

#[test]
fn detect_checkmate() {
    let pos = from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3");
    let moves = pos.legal_moves();

    assert!(moves.is_empty());
    assert!(pos.is_check());
}

#[test]
fn detect_stalemate() {
    let pos = from_fen("5bnr/4p1pq/4Qpkr/7p/2P4P/8/PP1PPPP1/RNB1KBNR b - - 0 1");
    let moves = pos.legal_moves();

    assert!(moves.is_empty());
    assert!(!pos.is_check());
}

#[test]
fn detect_50_move_rule() {
    let mut pos = from_fen("K1k5/8/8/8/8/8/8/8 w - - 0 1");
    let mut rng = rng();

    for _ in 0..50 {
        let moves = pos.legal_moves();
        let mv = moves[rng.random_range(0..moves.len())];
        pos.play_move(mv);
    }

    assert_eq!(pos.half_move_clock(), 50);
}

#[test]
fn escape_double_check() {
    let pos = from_fen("k7/5P2/6P1/3K4/8/1R2n2Q/5N2/7b w - - 0 1");
    let moves = pos.legal_moves();
    assert_eq!(moves.len(), 5);

    for &mv in &moves {
        assert_eq!(encoding::src_piece(mv), pieces::WHITE_KING);
    }
}

#[test]
fn detect_move_gives_check() {
    let mut pos = from_fen("6N1/8/8/3k4/6B1/2P5/2R1R3/7K w - - 0 1");
    let moves = pos.legal_moves();
    let undo_info = pos.undo_info();

    for &mv in &moves {
        pos.play_move(mv);
        let gives_check = pos.is_check();
        pos.undo_move(mv, undo_info);

        assert_eq!(encoding::gives_check(mv), gives_check);
    }
}

#[test]
fn castles_checkmate() {
    let mut pos = from_fen("8/8/8/8/8/5BB1/PPP2P2/R3K1k1 w Q - 0 1");

    let mv = encoding::castling_move(squares::E1, squares::C1, pieces::WHITE_KING);
    let mv = encoding::mark_check(mv);
    assert!(pos.legal_moves().contains(mv));

    pos.play_move(mv);
    assert!(pos.is_check());
    assert!(pos.legal_moves().is_empty());
}

#[test]
fn many_moves1() {
    let pos = from_fen("3Q4/1Q4Q1/4Q3/2Q4R/Q4Q2/3Q4/1Q4Rp/1K1BBNNk w - - 0 1");
    let moves = pos.legal_moves();

    assert_eq!(moves.len(), 218);
}

#[test]
fn many_moves2() {
    let pos = from_fen("8/PPPPPPPP/6k1/2BB4/5Q2/2NN3K/1R6/R7 w - - 0 1");
    let moves = pos.legal_moves();

    assert_eq!(moves.len(), 111);
}

#[test]
fn consistent_hash() {
    let mut pos = from_fen(Position::START_FEN);
    let mut undo = Vec::<(Move, UndoInfo, u64)>::new();
    let mut rng = rng();

    for _ in 0..50 {
        let moves = pos.legal_moves();

        if moves.is_empty() {
            break;
        }

        let mv = moves[rng.random_range(0..moves.len())];
        undo.push((mv, pos.undo_info(), pos.hash));
        pos.play_move(mv);
    }

    for &(mv, undo_info, hash) in undo.iter().rev() {
        pos.undo_move(mv, undo_info);
        assert_eq!(pos.hash, hash);
    }
}

#[test]
fn triple_repetition() {
    use squares::{A3, B1, G8, H6};

    let mut pos = from_fen(Position::START_FEN);
    let white_piece = pieces::WHITE_KNIGHT;
    let black_piece = pieces::BLACK_KNIGHT;
    let captured = pieces::NONE;

    let white_mv1 = encoding::normal_move(B1, A3, white_piece, captured);
    let white_mv2 = encoding::normal_move(A3, B1, white_piece, captured);
    let black_mv1 = encoding::normal_move(G8, H6, black_piece, captured);
    let black_mv2 = encoding::normal_move(H6, G8, black_piece, captured);

    pos.play_move(white_mv1);
    pos.play_move(black_mv1);

    pos.play_move(white_mv2);
    pos.play_move(black_mv2);

    pos.play_move(white_mv1);
    pos.play_move(black_mv1);

    pos.play_move(white_mv2);
    pos.play_move(black_mv2);

    assert_eq!(pos.rep_count(), 3);
}
