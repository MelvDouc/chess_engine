use crate::{
    game::{
        board::{
            NB_WINGS, pieces,
            squares::{self, ep_capture_square},
        },
        moves::{
            Move,
            castling::{
                castling_bit, castling_color_mask, get_wing, rook_dest_square, rook_src_square,
            },
            encoding,
        },
        position::Position,
    },
    macros::{const_while, ternary},
};

pub(super) const fn play_move(pos: &mut Position, mv: Move) {
    match encoding::move_kind(mv) {
        encoding::move_kinds::NORMAL => {
            play_normal_move(pos, mv);
        }
        encoding::move_kinds::EN_PASSANT => {
            play_ep_move(pos, mv);
        }
        encoding::move_kinds::PROMOTION => {
            play_promotion_move(pos, mv);
        }
        encoding::move_kinds::CASTLING => {
            play_castling_move(pos, mv);
        }
        _ => {}
    };
}

pub(super) const fn undo_move(pos: &mut Position, mv: Move) {
    match encoding::move_kind(mv) {
        encoding::move_kinds::NORMAL | encoding::move_kinds::PROMOTION => {
            undo_normal_move(pos, mv);
        }
        encoding::move_kinds::EN_PASSANT => {
            undo_ep_move(pos, mv);
        }
        encoding::move_kinds::CASTLING => {
            undo_castling_move(pos, mv);
        }
        _ => {}
    };
}

pub(super) const fn update_info(pos: &mut Position, mv: Move) {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);
    let src_piece = encoding::src_piece(mv);
    let captured = encoding::captured(mv);
    let is_capture = captured != pieces::NONE;

    if pos.can_color_castle(pos.active_color) {
        if pieces::is_king(src_piece) {
            remove_color_castling_rights(pos, pos.active_color);
        } else if pieces::is_rook(src_piece) {
            remove_wing_castling_rights(pos, src_sq, pos.active_color);
        }
    }

    if pos.can_color_castle(pos.inactive_color()) && is_capture && pieces::is_rook(captured) {
        remove_wing_castling_rights(pos, dest_sq, pos.inactive_color());
    }

    pos.set_ep_square(ternary!(
        pieces::is_pawn(src_piece) && is_double_pawn_move(src_sq, dest_sq),
        (src_sq + dest_sq) / 2,
        squares::NONE
    ));

    pos.half_move_clock = ternary!(
        pieces::is_pawn(src_piece) || is_capture,
        0,
        pos.half_move_clock + 1
    );

    pos.toggle_active_color();
}

const fn play_normal_move(pos: &mut Position, mv: Move) {
    let dest_sq = encoding::dest_square(mv);

    if encoding::is_capture(mv) {
        pos.remove_piece(dest_sq);
    }

    pos.remove_piece(encoding::src_square(mv));
    pos.set_piece(dest_sq, encoding::src_piece(mv));
}

const fn undo_normal_move(pos: &mut Position, mv: Move) {
    let dest_sq = encoding::dest_square(mv);
    let captured = encoding::captured(mv);

    pos.remove_piece(dest_sq);

    if captured != pieces::NONE {
        pos.set_piece(dest_sq, captured);
    }

    pos.set_piece(encoding::src_square(mv), encoding::src_piece(mv));
}

const fn play_ep_move(pos: &mut Position, mv: Move) {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);

    pos.remove_piece(src_sq);
    pos.remove_piece(ep_capture_square(src_sq, dest_sq));
    pos.set_piece(dest_sq, encoding::src_piece(mv));
}

const fn undo_ep_move(pos: &mut Position, mv: Move) {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);

    pos.set_piece(ep_capture_square(src_sq, dest_sq), encoding::captured(mv));
    pos.remove_piece(dest_sq);
    pos.set_piece(src_sq, encoding::src_piece(mv));
}

const fn play_promotion_move(pos: &mut Position, mv: Move) {
    let dest_sq = encoding::dest_square(mv);
    pos.remove_piece(encoding::src_square(mv));

    if encoding::is_capture(mv) {
        pos.remove_piece(dest_sq);
    }

    pos.set_piece(dest_sq, encoding::promoted(mv));
}

const fn play_castling_move(pos: &mut Position, mv: Move) {
    let king = encoding::src_piece(mv);
    let color = pieces::color_of(king);
    let king_src_sq = encoding::src_square(mv);
    let king_dest_sq = encoding::dest_square(mv);
    let wing = get_wing(king_src_sq, king_dest_sq);

    pos.remove_piece(king_src_sq);
    pos.remove_piece(rook_src_square(color, wing));
    pos.set_piece(king_dest_sq, king);
    pos.set_piece(rook_dest_square(color, wing), pieces::rook_of(color));
}

const fn undo_castling_move(pos: &mut Position, mv: Move) {
    let king = encoding::src_piece(mv);
    let color = pieces::color_of(king);
    let king_src_sq = encoding::src_square(mv);
    let king_dest_sq = encoding::dest_square(mv);
    let wing = get_wing(king_src_sq, king_dest_sq);

    pos.remove_piece(king_dest_sq);
    pos.remove_piece(rook_dest_square(color, wing));
    pos.set_piece(king_src_sq, king);
    pos.set_piece(rook_src_square(color, wing), pieces::rook_of(color));
}

const fn is_double_pawn_move(src_sq: usize, dest_sq: usize) -> bool {
    squares::rank_of(dest_sq).abs_diff(squares::rank_of(src_sq)) == 2
}

const fn remove_color_castling_rights(pos: &mut Position, color: usize) {
    let castling_rights = pos.castling_rights & !castling_color_mask(color);
    pos.set_castling_rights(castling_rights);
}

const fn remove_wing_castling_rights(pos: &mut Position, sq: usize, color: usize) {
    const_while!(wing, 0, NB_WINGS, {
        if sq == rook_src_square(color, wing) {
            let castling_rights = pos.castling_rights & !castling_bit(color, wing);
            pos.set_castling_rights(castling_rights);
            break;
        }
    });
}
