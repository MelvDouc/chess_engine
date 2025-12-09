use crate::{
    game::{
        board::{NB_FILES, NB_PIECES, NB_RANKS, colors, lines, pieces, squares},
        position::{Position, fen},
    },
    macros::ternary,
};

use colored::Colorize;

pub(crate) fn print_position(pos: &Position) {
    const _PIECE_UNICODE: [char; NB_PIECES] =
        ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔'];

    for rank in (0..NB_RANKS).rev() {
        print!("{} ", lines::rank_name(rank));

        for file in 0..NB_FILES {
            let sq = squares::of(rank, file);
            let piece = pos.get_piece(sq);
            let ch = ternary!(piece == pieces::NONE, ' ', pieces::initial_of(piece));
            let str = format!(" {} ", ch);
            let str = ternary!(squares::is_dark(sq), str.on_green(), str.on_white());

            print!("{}", str.black());
        }

        print!("\n");
    }

    print!("  ");

    for file in 0..NB_FILES {
        print!(" {} ", lines::file_name(file).to_ascii_uppercase());
    }

    println!(
        " ({} {} {} {})\n",
        colors::initial_of(pos.get_active_color()),
        fen::stringify_castling_rights(pos.get_castling_rights()),
        fen::stringify_ep_square(pos.get_ep_square()),
        pos.half_move_clock()
    );
}
