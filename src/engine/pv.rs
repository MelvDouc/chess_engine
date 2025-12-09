use crate::{
    engine::transposition as tp,
    game::{
        board::{colors, lines, pieces, squares, wings},
        moves::{Move, NULL_MOVE, castling::get_wing, encoding},
        position::Position,
    },
    macros::ternary,
};

type PVNode = (u16, u32);
type PV = Vec<PVNode>;

pub(super) fn stringify(pos: &mut Position, tt: &tp::Table, depth: usize) -> String {
    let pv = collect_pv(pos, tt, depth);
    let mut color = pos.get_active_color();
    let first_mv_black = color == colors::BLACK;
    let mut output = String::new();

    for &(mv_number, mv) in &pv {
        if color == colors::WHITE {
            let string = format!("{}.", mv_number);
            output.push_str(&string);
        }

        output.push_str(&stringify_move(mv));
        output.push(' ');
        color = colors::rev(color);
    }

    if first_mv_black {
        let string = format!("{}...", pv[0].0);
        output.insert_str(0, &string);
    }

    output
}

fn collect_pv(pos: &mut Position, tt: &tp::Table, depth: usize) -> PV {
    let mut pv = PV::new();
    let mut undo_infos = Vec::<u32>::new();
    let mut mv_number = 1;

    for _ in 0..depth {
        if pos.legal_moves().is_empty() {
            break;
        }

        match tp::get_entry(tt, pos.hash()) {
            Some(entry) => {
                if entry.mv == NULL_MOVE {
                    break;
                }

                pv.push((mv_number, entry.mv));
                undo_infos.push(pos.undo_info());
                pos.play_move(entry.mv);

                if pos.get_active_color() == colors::WHITE {
                    mv_number += 1;
                }
            }
            None => {
                break;
            }
        };
    }

    for i in (0..pv.len()).rev() {
        pos.undo_move(pv[i].1, undo_infos[i]);
    }

    pv
}

fn stringify_move(mv: Move) -> String {
    let src_sq = encoding::src_square(mv);
    let dest_sq = encoding::dest_square(mv);

    if encoding::is_castling(mv) {
        let result = ternary!(
            get_wing(src_sq, dest_sq) == wings::QUEEN_SIDE,
            "0-0-0",
            "0-0"
        );
        return result.to_string();
    }

    let src_piece = encoding::src_piece(mv);
    let is_capture = encoding::is_capture(mv);
    let mut result = String::new();

    if !pieces::is_pawn(src_piece) {
        result.push(pieces::initial_of(src_piece).to_ascii_uppercase());

        if is_capture {
            result.push('x');
        }

        result.push_str(&squares::name_of(dest_sq));
        return result;
    }

    if is_capture {
        let src_file = squares::file_of(src_sq);
        result.push(lines::file_name(src_file));
        result.push('x');
    }

    result.push_str(&squares::name_of(dest_sq));

    if encoding::is_promotion(mv) {
        let promoted = encoding::promoted(mv);
        result.push(pieces::initial_of(promoted).to_ascii_uppercase());
    }

    result
}
