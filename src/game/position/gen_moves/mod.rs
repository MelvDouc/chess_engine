mod castling_moves;
mod check_type;
mod figure_moves;
mod pawn_moves;
mod pins;

use crate::{
    bit_boards::set_bits,
    game::{
        board::pieces::{self, piece_types},
        moves::MoveList,
        position::Position,
    },
};

use castling_moves::castling_moves;
use check_type::CheckType;
use figure_moves::figure_moves;
use pawn_moves::pawn_moves;
use pins::get_pin_mask;

pub(crate) const fn legal_moves(pos: &Position) -> MoveList {
    let color = pos.active_color;
    let enemy_color = pos.inactive_color();
    let own_occ = pos.active_occupancy();
    let king_sq = pos.king_square(color);
    let check_type = CheckType::get(pos);
    let mut moves = MoveList::new();

    let check_mask = match check_type {
        CheckType::None => u64::MAX,
        CheckType::Single(mask) => mask,
        CheckType::Double => 0,
    };

    set_bits!(own_occ, src_sq, {
        let piece = pos.get_piece(src_sq);

        match pieces::type_of(piece) {
            piece_types::PAWN => {
                let pin_check_mask = check_mask
                    & get_pin_mask(
                        &pos.board,
                        pos.full_occupancy(),
                        king_sq,
                        src_sq,
                        enemy_color,
                    );
                pawn_moves(pos, &mut moves, pin_check_mask, piece, src_sq);
            }
            piece_types::KING => {
                let enemy_attacks = pos.color_attacks(enemy_color);
                figure_moves(pos, &mut moves, !(own_occ | enemy_attacks), piece, src_sq);

                if check_type.is_none() && pos.can_color_castle(color) {
                    castling_moves(pos, &mut moves, enemy_attacks);
                }
            }
            _ => {
                let attack_mask = !own_occ
                    & check_mask
                    & get_pin_mask(
                        &pos.board,
                        pos.full_occupancy(),
                        king_sq,
                        src_sq,
                        enemy_color,
                    );
                figure_moves(pos, &mut moves, attack_mask, piece, src_sq);
            }
        };
    });

    moves
}

macro_rules! add_move {
    ($pos: expr, $moves: expr, $mv: expr) => {
        let enemy_king_sq = $pos.king_square($pos.inactive_color());

        if crate::game::position::gen_moves::check_type::gives_check($pos, enemy_king_sq, $mv) {
            $moves.push(crate::game::moves::encoding::mark_check($mv));
        } else {
            $moves.push($mv);
        }
    };
}

pub(self) use add_move;
