use crate::game::{
    board::{colors, pieces, squares, wings},
    moves::{
        MoveList,
        castling::{castling_bit, has_castling_right},
        encoding,
    },
    position::Position,
};

macro_rules! assert_castling_right {
    ($castling_rights: expr, $color: ident, $wing: ident) => {
        assert!(has_castling_right(
            $castling_rights,
            colors::$color,
            wings::$wing
        ))
    };
}

#[test]
fn simple_castling() {
    let pos = super::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    let castling_rights = pos.get_castling_rights();

    assert_castling_right!(castling_rights, WHITE, QUEEN_SIDE);
    assert_castling_right!(castling_rights, WHITE, KING_SIDE);
    assert_castling_right!(castling_rights, BLACK, QUEEN_SIDE);
    assert_castling_right!(castling_rights, BLACK, KING_SIDE);
}

#[test]
fn queen_side_castling() {
    let mut pos = super::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    let moves = get_castling_moves(&mut pos);

    pos.play_move(moves[0]);
    assert_eq!(pos.get_piece(squares::A1), pieces::NONE);
    assert_eq!(pos.get_piece(squares::E1), pieces::NONE);
    assert_eq!(pos.get_piece(squares::C1), pieces::WHITE_KING);
    assert_eq!(pos.get_piece(squares::D1), pieces::WHITE_ROOK);
}

#[test]
fn king_side_castling() {
    let mut pos = super::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
    let moves = get_castling_moves(&mut pos);

    pos.play_move(moves[1]);
    assert_eq!(pos.get_piece(squares::E8), pieces::NONE);
    assert_eq!(pos.get_piece(squares::H8), pieces::NONE);
    assert_eq!(pos.get_piece(squares::G8), pieces::BLACK_KING);
    assert_eq!(pos.get_piece(squares::F8), pieces::BLACK_ROOK);
}

#[test]
fn cannot_castle_in_check() {
    let mut pos = super::from_fen("1k2r2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    let moves = get_castling_moves(&mut pos);
    assert_eq!(moves.len(), 0);
}

#[test]
fn cannot_castle_though_check() {
    let mut pos = super::from_fen("2r1k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    let moves = get_castling_moves(&mut pos);
    assert_eq!(moves.len(), 1);
}

#[test]
fn cannot_castle_though_occupied_square() {
    let mut pos = super::from_fen("r3k2r/8/8/8/8/8/8/R3K1NR w KQkq - 0 1");
    let moves = get_castling_moves(&mut pos);
    assert_eq!(moves.len(), 1);
}

#[test]
fn unset_rights_on_king_move() {
    let mut pos = super::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let mv = encoding::normal_move(squares::E1, squares::E2, pieces::WHITE_KING, pieces::NONE);
    pos.play_move(mv);

    assert_eq!(pos.get_castling_rights(), 0);
}

#[test]
fn unset_rights_on_rook_move() {
    let mut pos = super::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let mv = encoding::normal_move(squares::A1, squares::A8, pieces::WHITE_ROOK, pieces::NONE);
    pos.play_move(mv);

    assert_eq!(
        pos.get_castling_rights(),
        castling_bit(colors::WHITE, wings::KING_SIDE)
    );
}

#[test]
fn unset_rights_on_rook_capture() {
    let mut pos = super::from_fen("r3k3/8/8/8/8/8/8/R3K2R b KQ - 0 1");
    let mv = encoding::normal_move(
        squares::A8,
        squares::A1,
        pieces::BLACK_ROOK,
        pieces::WHITE_ROOK,
    );
    pos.play_move(mv);

    assert_eq!(
        pos.get_castling_rights(),
        castling_bit(colors::WHITE, wings::KING_SIDE)
    );
}

fn get_castling_moves(pos: &mut Position) -> MoveList {
    super::filter_move_kind(pos, encoding::move_kinds::CASTLING)
}
