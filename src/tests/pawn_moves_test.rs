use crate::{
    constants::{
        board_constants::BOARD_WIDTH,
        piece::{type_of, BLACK_PAWN, PTYPE_PAWN, WHITE_PAWN},
        square::{D5, E5, E6},
    },
    game::fen_string::{parse_fen, START_FEN},
    moves::move_encoding::{
        decode_dest_square, decode_kind, decode_src_piece, decode_src_square, en_passant_move,
        MoveKind,
    },
};

#[test]
fn set_en_passant_square() {
    let mut pos = parse_fen(START_FEN);
    let legal_moves = pos.legal_moves();
    let mut double_pawn_move_count = 0;
    let undo_info = pos.undo_move_info();

    for mv in legal_moves {
        if let MoveKind::Normal = decode_kind(mv) {
            let src_sq = decode_src_square(mv);
            let dest_sq = decode_dest_square(mv);
            let src_piece = decode_src_piece(mv);

            if type_of(src_piece) == PTYPE_PAWN && dest_sq == src_sq + BOARD_WIDTH * 2 {
                double_pawn_move_count += 1;
                pos.play_move(mv);
                assert_eq!(pos.get_en_passant_square(), src_sq + 8);
                pos.undo_move(mv, undo_info);
            }
        }
    }

    assert_eq!(double_pawn_move_count, BOARD_WIDTH);
}

#[test]
fn play_en_passant_capture() {
    let mut pos = parse_fen("rnbqkbnr/ppp2ppp/8/3Pp3/8/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3");

    assert_eq!(pos.get_en_passant_square(), E6);

    let ep_move = en_passant_move(D5, E6, WHITE_PAWN, BLACK_PAWN);
    let legal_moves1 = pos.legal_moves();
    let undo_info = pos.undo_move_info();

    assert!(legal_moves1.contains(&ep_move));

    pos.play_move(ep_move);

    assert!(!pos.has_piece_on(WHITE_PAWN, D5));
    assert!(!pos.has_piece_on(BLACK_PAWN, E5));
    assert!(pos.has_piece_on(WHITE_PAWN, E6));

    pos.undo_move(ep_move, undo_info);

    let legal_moves2 = pos.legal_moves();
    assert_eq!(legal_moves1.len(), legal_moves2.len());
}
