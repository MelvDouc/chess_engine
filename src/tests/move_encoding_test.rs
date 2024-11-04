use rand::{thread_rng, Rng};

use crate::{
    constants::{
        piece::{get_piece, BLACK_PAWN, NONE_PIECE, PTYPE_KNIGHT, PTYPE_QUEEN, WHITE_PAWN},
        square::{A1, A5, A6, H5, H6, H8},
        Color,
    },
    moves::move_encoding::{
        decode_captured, decode_dest_square, decode_kind, decode_promoted, decode_src_piece,
        decode_src_square, en_passant_move, normal_move, promotion_move, MoveKind,
    },
};

#[test]
fn random_move_encoding() {
    let mut rng = thread_rng();
    let src_sq = rng.gen_range(A1..=H8);
    let dest_sq = rng.gen_range(A1..=H8);
    let src_piece = rng.gen_range(WHITE_PAWN..NONE_PIECE);
    let captured = rng.gen_range(WHITE_PAWN..=NONE_PIECE);
    let is_promotion = rng.gen_bool(25.0 / 100.0);

    let mv = if is_promotion {
        let promoted_type = rng.gen_range(PTYPE_KNIGHT..=PTYPE_QUEEN);
        let promoted = get_piece(promoted_type, Color::White);
        promotion_move(src_sq, dest_sq, src_piece, captured, promoted)
    } else {
        normal_move(src_sq, dest_sq, src_piece, captured)
    };

    assert_eq!(decode_src_square(mv), src_sq);
    assert_eq!(decode_dest_square(mv), dest_sq);
    assert_eq!(decode_src_piece(mv), src_piece);
    assert_eq!(decode_captured(mv), captured);

    match decode_kind(mv) {
        MoveKind::Normal => assert!(!is_promotion),
        MoveKind::Promotion => assert!(is_promotion),
        _ => assert!(false),
    }
}

#[test]
fn random_en_passant() {
    let mut rng = thread_rng();
    let src_sq = rng.gen_range(A5..=H5);
    let dest_sq = rng.gen_range(A6..=H6);
    let src_piece = WHITE_PAWN;
    let captured = BLACK_PAWN;

    let mv = en_passant_move(src_sq, dest_sq, src_piece, captured);

    assert_eq!(decode_src_square(mv), src_sq);
    assert_eq!(decode_dest_square(mv), dest_sq);
    assert_eq!(decode_src_piece(mv), src_piece);
    assert_eq!(decode_captured(mv), captured);
    assert_eq!(decode_promoted(mv), NONE_PIECE);

    match decode_kind(mv) {
        MoveKind::EnPassant => assert!(true),
        _ => assert!(false),
    }
}
