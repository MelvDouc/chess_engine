// bits 0-5 = src square
// bits 6-11 = dest square
// bits 12-15 = src piece
// bits 16-19 = captured piece
// bits 20-23 = promoted piece
// bits 24-25 = move kind

use crate::constants::{
    piece::{piece_initial, NONE_PIECE},
    square, wing,
};

const SQUARE_MASK: u32 = (1 << 6) - 1;
const PIECE_MASK: u32 = (1 << 4) - 1;

pub(crate) enum MoveKind {
    Normal,
    EnPassant,
    Promotion,
    Castling,
}

/// Convert move information to a 32-bit unsigned integer.
const fn encode_move(
    src_sq: usize,
    dest_sq: usize,
    src_piece: usize,
    captured: usize,
    promoted: usize,
    kind: MoveKind,
) -> u32 {
    (src_sq
        | dest_sq << 6
        | src_piece << 12
        | captured << 16
        | promoted << 20
        | (kind as usize) << 24) as u32
}

pub(crate) const fn normal_move(
    src_sq: usize,
    dest_sq: usize,
    src_piece: usize,
    captured: usize,
) -> u32 {
    encode_move(
        src_sq,
        dest_sq,
        src_piece,
        captured,
        NONE_PIECE,
        MoveKind::Normal,
    )
}

pub(crate) const fn en_passant_move(
    src_sq: usize,
    dest_sq: usize,
    src_piece: usize,
    captured: usize,
) -> u32 {
    encode_move(
        src_sq,
        dest_sq,
        src_piece,
        captured,
        NONE_PIECE,
        MoveKind::EnPassant,
    )
}

pub(crate) const fn promotion_move(
    src_sq: usize,
    dest_sq: usize,
    src_piece: usize,
    captured: usize,
    promoted: usize,
) -> u32 {
    encode_move(
        src_sq,
        dest_sq,
        src_piece,
        captured,
        promoted,
        MoveKind::Promotion,
    )
}

pub(crate) const fn castling_move(king_src_sq: usize, wing: u8, king: usize) -> u32 {
    let dest_sq = if wing == wing::KING_SIDE {
        king_src_sq + 2
    } else {
        king_src_sq - 2
    };
    encode_move(
        king_src_sq,
        dest_sq,
        king,
        NONE_PIECE,
        NONE_PIECE,
        MoveKind::Castling,
    )
}

pub(crate) const fn decode_src_square(mv: u32) -> usize {
    (mv & SQUARE_MASK) as usize
}

pub(crate) const fn decode_dest_square(mv: u32) -> usize {
    (mv >> 6 & SQUARE_MASK) as usize
}

pub(crate) const fn decode_src_piece(mv: u32) -> usize {
    (mv >> 12 & PIECE_MASK) as usize
}

pub(crate) const fn decode_captured(mv: u32) -> usize {
    (mv >> 16 & PIECE_MASK) as usize
}

pub(crate) const fn decode_promoted(mv: u32) -> usize {
    (mv >> 20 & PIECE_MASK) as usize
}

pub(crate) const fn decode_kind(mv: u32) -> MoveKind {
    match mv >> 24 {
        0 => MoveKind::Normal,
        1 => MoveKind::EnPassant,
        2 => MoveKind::Promotion,
        3 => MoveKind::Castling,
        _ => panic!("Invalid move kind"),
    }
}

pub(crate) fn move_notation(mv: u32) -> String {
    let mut str = String::new();
    str.push_str(square::name_of(decode_src_square(mv)));
    str.push_str(square::name_of(decode_dest_square(mv)));

    if let MoveKind::Promotion = decode_kind(mv) {
        let promoted = decode_promoted(mv);
        str.push(piece_initial(promoted).to_ascii_uppercase());
    }

    str
}
