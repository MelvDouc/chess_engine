use crate::{
    game::board::{NB_FILES, NB_PIECES, NB_RANKS, colors, lines, pieces, squares},
    macros::ternary,
};

use colored::Colorize;

pub(crate) fn print_position(pos: &super::Position) {
    const _PIECE_UNICODE: [char; NB_PIECES] =
        ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔'];

    for rank in (0..NB_RANKS).rev() {
        print!("{} ", lines::rank_name(rank));

        for file in 0..NB_FILES {
            let sq = squares::of(rank, file);
            let piece = pos.get_piece(sq);
            let ch = ternary!(piece == pieces::NONE, ' ', pieces::initial_of(piece));
            let str = format!(" {} ", ch);
            let str = ternary!(squares::is_dark(sq), str.on_cyan(), str.on_white());

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
        colors::initial_of(pos.active_color),
        super::fen::stringify_castling_rights(pos.castling_rights),
        super::fen::stringify_ep_square(pos.en_passant_sq),
        pos.half_move_clock
    );
}
