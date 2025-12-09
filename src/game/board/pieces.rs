use crate::game::board::NB_PIECES;

pub(crate) mod piece_types {
    pub(crate) const PAWN: usize = 0;
    pub(crate) const KNIGHT: usize = 1;
    pub(crate) const BISHOP: usize = 2;
    pub(crate) const ROOK: usize = 3;
    pub(crate) const QUEEN: usize = 4;
    pub(crate) const KING: usize = 5;
}

pub(crate) const WHITE_PAWN: usize = 0;
pub(crate) const BLACK_PAWN: usize = 1;
pub(crate) const WHITE_KNIGHT: usize = 2;
pub(crate) const BLACK_KNIGHT: usize = 3;
pub(crate) const WHITE_BISHOP: usize = 4;
pub(crate) const BLACK_BISHOP: usize = 5;
pub(crate) const WHITE_ROOK: usize = 6;
pub(crate) const BLACK_ROOK: usize = 7;
pub(crate) const WHITE_QUEEN: usize = 8;
pub(crate) const BLACK_QUEEN: usize = 9;
pub(crate) const WHITE_KING: usize = 10;
pub(crate) const BLACK_KING: usize = 11;
pub(crate) const NONE: usize = 12;

pub(crate) const fn of(piece_type: usize, color: usize) -> usize {
    piece_type << 1 | color
}

pub(crate) const fn pawn_of(color: usize) -> usize {
    of(piece_types::PAWN, color)
}

pub(crate) const fn knight_of(color: usize) -> usize {
    of(piece_types::KNIGHT, color)
}

pub(crate) const fn bishop_of(color: usize) -> usize {
    of(piece_types::BISHOP, color)
}

pub(crate) const fn rook_of(color: usize) -> usize {
    of(piece_types::ROOK, color)
}

pub(crate) const fn queen_of(color: usize) -> usize {
    of(piece_types::QUEEN, color)
}

pub(crate) const fn king_of(color: usize) -> usize {
    of(piece_types::KING, color)
}

pub(crate) const fn color_of(piece: usize) -> usize {
    piece & 1
}

pub(crate) const fn rev_color(piece: usize) -> usize {
    piece ^ 1
}

pub(crate) const fn type_of(piece: usize) -> usize {
    piece >> 1
}

pub(crate) const fn is_pawn(piece: usize) -> bool {
    type_of(piece) == piece_types::PAWN
}

pub(crate) const fn is_knight(piece: usize) -> bool {
    type_of(piece) == piece_types::KNIGHT
}

pub(crate) const fn is_bishop(piece: usize) -> bool {
    type_of(piece) == piece_types::BISHOP
}

pub(crate) const fn is_rook(piece: usize) -> bool {
    type_of(piece) == piece_types::ROOK
}

pub(crate) const fn is_queen(piece: usize) -> bool {
    type_of(piece) == piece_types::QUEEN
}

pub(crate) const fn is_king(piece: usize) -> bool {
    type_of(piece) == piece_types::KING
}

pub(crate) const fn is_slider(piece: usize) -> bool {
    let piece_type = type_of(piece);

    piece_type == piece_types::BISHOP
        || piece_type == piece_types::ROOK
        || piece_type == piece_types::QUEEN
}

pub(crate) const fn initial_of(piece: usize) -> char {
    const INITIALS: [char; NB_PIECES] =
        ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k'];

    INITIALS[piece]
}

pub(crate) const fn from_initial(initial: char) -> usize {
    match initial {
        'P' => WHITE_PAWN,
        'N' => WHITE_KNIGHT,
        'B' => WHITE_BISHOP,
        'R' => WHITE_ROOK,
        'Q' => WHITE_QUEEN,
        'K' => WHITE_KING,
        'p' => BLACK_PAWN,
        'n' => BLACK_KNIGHT,
        'b' => BLACK_BISHOP,
        'r' => BLACK_ROOK,
        'q' => BLACK_QUEEN,
        'k' => BLACK_KING,
        _ => NONE,
    }
}
