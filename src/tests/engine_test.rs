use crate::{
    constants::Color,
    engine::eval_position::pawns::eval_pawns,
    game::fen_string::{parse_fen, START_FEN},
};

#[test]
fn pawn_eval_start_pos() {
    let pos = parse_fen(START_FEN);
    let white_pawn_eval = eval_pawns(&pos, Color::White);
    assert_eq!(white_pawn_eval, 0i32);
    let black_pawn_eval = eval_pawns(&pos, Color::Black);
    assert_eq!(white_pawn_eval, black_pawn_eval);
}

#[test]
fn passed_pawns_eval() {
    let pos = parse_fen("6k1/P7/1P6/8/8/8/8/6K1 w - - 0 1");
    let ev = eval_pawns(&pos, Color::White);
    assert_eq!(ev, 10 * 6 + 10);
}
