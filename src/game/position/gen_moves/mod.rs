mod castling_moves;
mod checks;
mod figure_moves;
mod pawn_moves;
mod pins;

use crate::{
    bit_boards::{bit_mask, set_bits},
    game::{
        board::pieces::{self, piece_types},
        moves::{Move, MoveList, encoding, piece_attacks},
        position::Position,
    },
};

use castling_moves::castling_moves;
use figure_moves::figure_moves;
use pawn_moves::pawn_moves;
use pins::get_pin_mask;

pub(crate) const fn legal_moves(pos: &Position) -> MoveList {
    let color = pos.active_color;
    let enemy_color = pos.inactive_color();
    let own_occ = pos.active_occupancy();
    let king_sq = pos.king_square(color);
    let check_type = checks::CheckType::get(pos, king_sq);
    let check_mask = check_type.get_mask();
    let mut moves = MoveList::new(pos.king_square(enemy_color));

    set_bits!(own_occ, src_sq, {
        let piece = pos.get_piece(src_sq);

        match pieces::type_of(piece) {
            piece_types::PAWN => {
                let pin_check_mask = check_mask & get_pin_mask(pos, king_sq, src_sq, enemy_color);
                pawn_moves(pos, &mut moves, pin_check_mask, piece, src_sq);
            }
            piece_types::KING => {
                let enemy_attacks = color_attacks(pos, enemy_color, king_sq);
                figure_moves(pos, &mut moves, !(own_occ | enemy_attacks), piece, src_sq);

                if check_type.is_none() && pos.can_color_castle(color) {
                    castling_moves(pos, &mut moves, enemy_attacks);
                }
            }
            _ => {
                let attack_mask =
                    !own_occ & check_mask & get_pin_mask(pos, king_sq, src_sq, enemy_color);
                figure_moves(pos, &mut moves, attack_mask, piece, src_sq);
            }
        };
    });

    moves
}

const fn add_move(pos: &Position, moves: &mut MoveList, mut mv: Move) {
    if checks::gives_check(pos, moves.enemy_king_square(), mv) {
        mv = encoding::mark_check(mv);
    }

    moves.push(mv);
}

/// Returns not only the attacked squares but also those X-rayed through the opposing king.
const fn color_attacks(pos: &Position, color: usize, king_sq: usize) -> u64 {
    let occ = pos.full_occupancy() & !bit_mask(king_sq);
    let mut attacks = 0;

    set_bits!(pos.color_occupancy(color), sq, {
        attacks |= piece_attacks(pos.board[sq], sq, occ);
    });

    attacks
}
