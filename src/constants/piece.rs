use crate::constants::board_constants::NB_PIECE_TYPES;

use super::{board_constants::NB_PIECES, Color};

pub(crate) const PTYPE_PAWN: u8 = 0;
pub(crate) const PTYPE_KNIGHT: u8 = 1;
pub(crate) const PTYPE_KING: u8 = 2;
pub(crate) const PTYPE_BISHOP: u8 = 3;
pub(crate) const PTYPE_ROOK: u8 = 4;
pub(crate) const PTYPE_QUEEN: u8 = 5;

pub(crate) const PROMOTION_TYPES: [u8; 4] = [PTYPE_KNIGHT, PTYPE_BISHOP, PTYPE_ROOK, PTYPE_QUEEN];

pub(crate) const WHITE_PAWN: usize = 0;
pub(crate) const BLACK_PAWN: usize = 1;
pub(crate) const WHITE_KNIGHT: usize = 2;
pub(crate) const BLACK_KNIGHT: usize = 3;
pub(crate) const WHITE_KING: usize = 4;
pub(crate) const BLACK_KING: usize = 5;
pub(crate) const WHITE_BISHOP: usize = 6;
pub(crate) const BLACK_BISHOP: usize = 7;
pub(crate) const WHITE_ROOK: usize = 8;
pub(crate) const BLACK_ROOK: usize = 9;
pub(crate) const WHITE_QUEEN: usize = 10;
pub(crate) const BLACK_QUEEN: usize = 11;
pub(crate) const NONE_PIECE: usize = 12;

pub(crate) const fn type_of(piece: usize) -> u8 {
    const PIECES_TYPES: [u8; NB_PIECES] = [
        PTYPE_PAWN,
        PTYPE_PAWN,
        PTYPE_KNIGHT,
        PTYPE_KNIGHT,
        PTYPE_KING,
        PTYPE_KING,
        PTYPE_BISHOP,
        PTYPE_BISHOP,
        PTYPE_ROOK,
        PTYPE_ROOK,
        PTYPE_QUEEN,
        PTYPE_QUEEN,
    ];

    PIECES_TYPES[piece]
}

pub(crate) const fn get_piece(piece_type: u8, color: Color) -> usize {
    const PIECES: [[usize; 2]; NB_PIECE_TYPES] = [
        [WHITE_PAWN, BLACK_PAWN],
        [WHITE_KNIGHT, BLACK_KNIGHT],
        [WHITE_KING, BLACK_KING],
        [WHITE_BISHOP, BLACK_BISHOP],
        [WHITE_ROOK, BLACK_ROOK],
        [WHITE_QUEEN, BLACK_QUEEN],
    ];

    PIECES[piece_type as usize][color as usize]
}

pub(crate) const fn piece_initial(piece: usize) -> char {
    const PIECE_INITIALS: [char; NB_PIECES] =
        ['P', 'p', 'N', 'n', 'K', 'k', 'B', 'b', 'R', 'r', 'Q', 'q'];

    PIECE_INITIALS[piece]
}

pub(crate) const fn from_initial(initial: char) -> usize {
    match initial {
        'P' => WHITE_PAWN,
        'p' => BLACK_PAWN,
        'N' => WHITE_KNIGHT,
        'n' => BLACK_KNIGHT,
        'K' => WHITE_KING,
        'k' => BLACK_KING,
        'B' => WHITE_BISHOP,
        'b' => BLACK_BISHOP,
        'R' => WHITE_ROOK,
        'r' => BLACK_ROOK,
        'Q' => WHITE_QUEEN,
        'q' => BLACK_QUEEN,
        _ => NONE_PIECE,
    }
}
