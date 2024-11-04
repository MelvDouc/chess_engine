use crate::{game::fen_string::parse_fen, moves::move_encoding::move_notation};

#[test]
fn consistent_position_hash() {
    let fen = "r1bqk2r/1p1nb1pp/p1n1p3/2ppPp2/3P1P2/2N1BN2/PPPQB1PP/R3K2R w KQkq f6 0 10";
    let mut pos = parse_fen(fen);
    let hash1 = pos.get_hash();
    let legal_moves = pos.legal_moves();
    let undo_info = pos.undo_move_info();

    assert_eq!(hash1, pos.get_hash());

    for mv in legal_moves {
        pos.play_move(mv);

        assert_ne!(hash1, pos.get_hash());

        pos.undo_move(mv, undo_info);

        assert_eq!(hash1, pos.get_hash(), "move: {}", move_notation(mv));
    }
}
