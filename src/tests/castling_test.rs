use crate::{
    constants::{piece, square},
    game::{fen_string::parse_fen, Position},
    moves::move_encoding::{
        decode_dest_square, decode_kind, decode_src_piece, decode_src_square, MoveKind,
    },
};

#[test]
fn two_castling_moves() {
    let mut pos = parse_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let castling_moves = get_castling_moves(&mut pos);
    assert_eq!(castling_moves.len(), 2);
}

#[test]
fn castling_through_check() {
    let mut pos = parse_fen("4k3/8/8/8/8/8/2p5/R3K2R w KQ - 0 1");
    let castling_moves = get_castling_moves(&mut pos);

    assert_eq!(castling_moves.len(), 1);

    let mv = castling_moves[0];

    assert_eq!(decode_src_piece(mv), piece::WHITE_KING);
    assert_eq!(decode_src_square(mv), square::E1);
    assert_eq!(decode_dest_square(mv), square::G1);
}

#[test]
fn castling_through_occupied_square() {
    let mut pos = parse_fen("4k3/8/8/8/8/8/8/R3KB1R w KQ - 0 1");
    let castling_moves = get_castling_moves(&mut pos);

    assert_eq!(castling_moves.len(), 1);

    let mv = castling_moves[0];

    assert_eq!(decode_src_piece(mv), piece::WHITE_KING);
    assert_eq!(decode_src_square(mv), square::E1);
    assert_eq!(decode_dest_square(mv), square::C1);
}

fn get_castling_moves(pos: &mut Position) -> Vec<u32> {
    let mut legal_moves = pos.legal_moves();
    legal_moves.retain(|mv| match decode_kind(*mv) {
        MoveKind::Castling => true,
        _ => false,
    });
    legal_moves
}
