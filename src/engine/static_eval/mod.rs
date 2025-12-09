mod pawns;

use crate::{
    bit_boards::set_bits,
    engine::score::Score,
    game::{
        board::{
            NB_PIECES,
            pieces::{self, piece_types},
        },
        moves::piece_attacks,
        position::Position,
    },
    macros::const_while,
};

pub(crate) const fn eval_position(pos: &Position) -> Score {
    eval_side(pos, pos.get_active_color()) - eval_side(pos, pos.inactive_color())
}

pub(crate) const fn piece_value(piece: usize) -> Score {
    const PIECE_VALUES: [Score; NB_PIECES + 1] = [
        1000, 1000, 3000, 3000, 3150, 3150, 5000, 5000, 9500, 9500, 4000, 4000, 0,
    ];

    PIECE_VALUES[piece]
}

const fn eval_side(pos: &Position, color: usize) -> Score {
    count_material(pos, color) + eval_mobility(pos, color) + pawns::eval_pawns(pos, color)
}

const fn count_material(pos: &Position, color: usize) -> Score {
    let mut count = 0;

    const_while!(piece_type, 0, piece_types::KING, {
        let piece = pieces::of(piece_type, color);
        count += pos.piece_occupancy(piece).count_ones() as Score * piece_value(piece);
    });

    count
}

const fn eval_mobility(pos: &Position, color: usize) -> Score {
    let full_occ = pos.full_occupancy();
    let occ = pos.knight_occupancy(color)
        | pos.bishop_occupancy(color)
        | pos.rook_occupancy(color)
        | pos.queen_occupancy(color);
    let mut score = 0;

    set_bits!(occ, sq, {
        let piece = pos.get_piece(sq);
        score += 11 * piece_attacks(piece, sq, full_occ).count_ones() as Score;
    });

    score
}
