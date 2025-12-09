use crate::game::{
    board::{pieces, squares},
    moves::encoding,
    position::tests::filter_move_kind,
};

#[test]
fn blocked_pawn() {
    let pos = super::from_fen("K7/8/8/8/8/k7/P7/8 w - - 0 1");
    let mut moves = pos.legal_moves();
    moves.retain(|mv| encoding::src_piece(mv) == pieces::WHITE_PAWN);
    assert!(moves.is_empty());
}

#[test]
fn single_push() {
    let pos = super::from_fen("K7/8/8/8/k7/8/P7/8 w - - 0 1");
    let moves = pos.legal_moves();

    let src_sq = squares::A2;
    let src_piece = pieces::WHITE_PAWN;
    let captured = pieces::NONE;
    let push1 = encoding::normal_move(src_sq, squares::A3, src_piece, captured);
    let push2 = encoding::normal_move(src_sq, squares::A4, src_piece, captured);

    assert!(moves.contains(push1));
    assert!(!moves.contains(push2));
}

#[test]
/// Test situation when only a double pawn move is legal.
fn double_push() {
    let pos = super::from_fen("8/8/k7/8/K6r/7r/1P6/8 w - - 0 1");
    let moves = pos.legal_moves();
    assert_eq!(moves.len(), 1);

    let mv = moves[0];
    assert_eq!(encoding::src_piece(mv), pieces::WHITE_PAWN);
    assert_eq!(encoding::dest_square(mv), squares::B4);
}

#[test]
fn promotion_to_empty_square() {
    let mut pos = super::from_fen("7k/P7/8/8/8/8/8/K7 w - - 0 1");
    let promotions = filter_move_kind(&mut pos, encoding::move_kinds::PROMOTION);

    assert_eq!(promotions.len(), 4);
    assert_eq!(encoding::promoted(promotions[0]), pieces::WHITE_QUEEN);
    assert_eq!(encoding::promoted(promotions[1]), pieces::WHITE_KNIGHT);
    assert_eq!(encoding::promoted(promotions[2]), pieces::WHITE_ROOK);
    assert_eq!(encoding::promoted(promotions[3]), pieces::WHITE_BISHOP);
}

#[test]
fn promotion_capture() {
    let mut pos = super::from_fen("nn5k/P7/8/8/8/8/8/K7 w - - 0 1");

    let promotions = filter_move_kind(&mut pos, encoding::move_kinds::PROMOTION);
    assert_eq!(promotions.len(), 4);

    let undo_info = pos.undo_info();
    let mv = promotions[0];
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let src_piece = encoding::src_piece(mv);
    let captured = encoding::captured(mv);
    let promoted = encoding::promoted(mv);

    pos.play_move(mv);
    assert_eq!(pos.get_piece(src_sq), pieces::NONE);
    assert_eq!(pos.get_piece(dest_sq), promoted);

    pos.undo_move(mv, undo_info);
    assert_eq!(pos.get_piece(src_sq), src_piece);
    assert_eq!(pos.get_piece(dest_sq), captured);
}

#[test]
fn en_passant() {
    let mut pos = super::from_fen("rnbqkbnr/ppp2ppp/8/3Pp3/8/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3");

    let ep_moves = filter_move_kind(&mut pos, encoding::move_kinds::EN_PASSANT);
    assert_eq!(ep_moves.len(), 1);

    let undo_info = pos.undo_info();
    let mv = ep_moves[0];
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let src_piece = encoding::src_piece(mv);
    let captured = encoding::captured(mv);

    assert_eq!(captured, pieces::rev_color(src_piece));

    pos.play_move(mv);
    assert_eq!(pos.get_piece(src_sq), pieces::NONE);
    assert_eq!(pos.get_piece(src_sq + 1), pieces::NONE);
    assert_eq!(pos.get_piece(dest_sq), src_piece);

    pos.undo_move(mv, undo_info);
    assert_eq!(pos.get_piece(src_sq), src_piece);
    assert_eq!(pos.get_piece(src_sq + 1), captured);
    assert_eq!(pos.get_piece(dest_sq), pieces::NONE);
}

#[test]
fn en_passant_pin() {
    let mut pos = super::from_fen("6k1/b7/8/2Pp4/8/8/8/6K1 w - d6 0 1");
    let ep_moves = filter_move_kind(&mut pos, encoding::move_kinds::EN_PASSANT);

    assert!(ep_moves.is_empty());
}
