use super::board_constants::{INITIAL_PAWN_RANKS, INITIAL_PIECE_RANKS, NB_PIECES};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Color {
    White,
    Black,
}

impl Color {
    pub(crate) fn piece_color(piece: usize) -> Color {
        BY_PIECE[piece]
    }

    pub(crate) fn from_initial(initial: char) -> Color {
        match initial {
            'w' => Self::White,
            'b' => Self::Black,
            _ => panic!("Invalid color initial"),
        }
    }

    pub(crate) const fn reverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub(crate) const fn direction(&self) -> i8 {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }

    pub(crate) const fn initial(&self) -> char {
        match self {
            Self::White => 'w',
            Self::Black => 'b',
        }
    }

    pub(crate) const fn initial_pawn_rank(self) -> usize {
        INITIAL_PAWN_RANKS[self as usize]
    }

    pub(crate) const fn initial_piece_rank(self) -> usize {
        INITIAL_PIECE_RANKS[self as usize]
    }

    pub(crate) const fn is_white(&self) -> bool {
        match self {
            Color::White => true,
            Color::Black => false,
        }
    }
}

const BY_PIECE: [Color; NB_PIECES] = [
    Color::White,
    Color::Black,
    Color::White,
    Color::Black,
    Color::White,
    Color::Black,
    Color::White,
    Color::Black,
    Color::White,
    Color::Black,
    Color::White,
    Color::Black,
];
