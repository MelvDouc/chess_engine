use crate::{
    constants::{
        board_constants::{
            file_of, rank_of, square_of, BOARD_WIDTH, FILE_A, FILE_D, FILE_F, FILE_H,
        },
        piece::{get_piece, type_of, KING, NONE_PIECE, PAWN, ROOK},
        square::NONE_SQUARE,
        wing, Color,
    },
    moves::move_encoding::*,
    utils::math::usize_abs_diff,
};

use super::{castling::get_castling_right, Position};

pub(super) fn play_move(pos: &mut Position, mv: u32) {
    let src_sq = decode_src_square(mv);
    let dest_sq = decode_dest_square(mv);
    let src_piece = decode_src_piece(mv);
    let piece_type = type_of(src_piece);
    let captured = decode_captured(mv);
    let color = Color::piece_color(src_piece);

    pos.unset_piece(src_piece, src_sq);

    match decode_kind(mv) {
        MoveKind::Normal => {
            pos.set_piece(src_piece, dest_sq);
            update_capture(pos, dest_sq, captured);
        }
        MoveKind::EnPassant => {
            pos.set_piece(src_piece, dest_sq);
            let capture_sq = square_of(rank_of(src_sq), file_of(dest_sq));
            pos.unset_piece(captured, capture_sq);
        }
        MoveKind::Promotion => {
            pos.set_piece(decode_promoted(mv), dest_sq);
            update_capture(pos, dest_sq, captured);
        }
        MoveKind::Castling => {
            let rook = get_piece(ROOK, color);
            let wing = wing::get_wing(src_sq, dest_sq);

            pos.set_piece(src_piece, dest_sq);
            pos.unset_piece(rook, rook_src_square(color, wing));
            pos.set_piece(rook, rook_dest_square(color, wing));
        }
    };

    pos.set_en_passant_square(NONE_SQUARE);
    update_for_piece_type(pos, piece_type, src_sq, dest_sq, color);

    if !color.is_white() {
        if piece_type == PAWN || captured != NONE_PIECE {
            pos.half_move_clock += 1;
        } else {
            pos.half_move_clock = 0;
        }

        pos.full_move_number += 1;
    }
}

fn update_for_piece_type(
    pos: &mut Position,
    piece_type: usize,
    src_sq: usize,
    dest_sq: usize,
    color: Color,
) {
    match piece_type {
        PAWN => {
            if usize_abs_diff(src_sq, dest_sq) == BOARD_WIDTH * 2 {
                pos.set_en_passant_square((src_sq + dest_sq) / 2);
            }
        }
        KING => {
            let unset_rights = get_castling_right(color, wing::QUEEN_SIDE)
                | get_castling_right(color, wing::KING_SIDE);
            unset_castling_rights(pos, unset_rights);
        }
        ROOK => {
            update_castling_rights_for_rook(pos, color, src_sq);
        }
        _ => {}
    };
}

pub(super) fn undo_move(pos: &mut Position, mv: u32) {
    let src_sq = decode_src_square(mv);
    let dest_sq = decode_dest_square(mv);
    let src_piece = decode_src_piece(mv);

    pos.set_piece(src_piece, src_sq);

    match decode_kind(mv) {
        MoveKind::Normal => {
            pos.unset_piece(src_piece, dest_sq);
        }
        MoveKind::EnPassant => {
            let captured = decode_captured(mv);
            let capture_sq = square_of(rank_of(src_sq), file_of(dest_sq));

            pos.unset_piece(src_piece, dest_sq);
            pos.set_piece(captured, capture_sq);
        }
        MoveKind::Promotion => {
            pos.unset_piece(decode_promoted(mv), dest_sq);
        }
        MoveKind::Castling => {
            let color = Color::piece_color(src_piece);
            let rook = get_piece(ROOK, color);
            let wing = wing::get_wing(src_sq, dest_sq);

            pos.unset_piece(src_piece, dest_sq);
            pos.unset_piece(rook, rook_dest_square(color, wing));
            pos.set_piece(rook, rook_src_square(color, wing));
        }
    };
}

fn update_capture(pos: &mut Position, dest_sq: usize, captured: usize) {
    if captured != NONE_PIECE {
        pos.unset_piece(captured, dest_sq);

        if type_of(captured) == ROOK {
            let color = Color::piece_color(captured);
            update_castling_rights_for_rook(pos, color, dest_sq);
        }
    }
}

fn update_castling_rights_for_rook(pos: &mut Position, color: Color, rook_sq: usize) {
    if rank_of(rook_sq) == color.initial_piece_rank() {
        match file_of(rook_sq) {
            FILE_A => {
                unset_castling_rights(pos, get_castling_right(color, wing::QUEEN_SIDE));
            }
            FILE_H => {
                unset_castling_rights(pos, get_castling_right(color, wing::KING_SIDE));
            }
            _ => {}
        }
    }
}

fn unset_castling_rights(pos: &mut Position, unset_rights: u8) {
    let new_castling_rights = pos.get_castling_rights() & !unset_rights;
    pos.set_castling_rights(new_castling_rights);
}

fn rook_src_square(color: Color, wing: usize) -> usize {
    let file = match wing {
        wing::QUEEN_SIDE => FILE_A,
        wing::KING_SIDE => FILE_H,
        _ => panic!("Invalid castling wing"),
    };
    return square_of(color.initial_piece_rank(), file);
}

fn rook_dest_square(color: Color, wing: usize) -> usize {
    let file = match wing {
        wing::QUEEN_SIDE => FILE_D,
        wing::KING_SIDE => FILE_F,
        _ => panic!("Invalid castling wing"),
    };
    return square_of(color.initial_piece_rank(), file);
}
