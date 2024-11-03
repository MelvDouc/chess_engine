use crate::constants::{
    board_constants::{square_of, BOARD_WIDTH, RANK_1, RANK_8},
    piece, square, Color,
};

use super::{castling::parse_castling_rights, Position};

pub(crate) const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub(crate) fn parse_fen(fen: &str) -> Position {
    let parts = fen.split(" ").collect::<Vec<&str>>();

    let active_color = parse_active_color(parts[1]);
    let castling_rights = parse_castling_rights(parts[2]);
    let en_passant_square = square::from_name(parts[3]);
    let half_move_clock = parts[4].parse::<u8>().unwrap();
    let full_move_number = parts[5].parse::<u16>().unwrap();

    let mut pos = Position::create(
        active_color,
        castling_rights,
        en_passant_square,
        half_move_clock,
        full_move_number,
    );
    set_pieces(parts[0], &mut pos);
    pos
}

fn set_pieces(piece_str: &str, pos: &mut Position) -> () {
    let rows = piece_str.split("/").collect::<Vec<&str>>();

    for rank in RANK_1..=RANK_8 {
        let row = rows[BOARD_WIDTH - rank - 1];
        let mut file: usize = 0;

        for ch in row.chars() {
            if ch.is_digit(10) {
                file += ch.to_digit(10).unwrap() as usize;
                continue;
            }

            let sq = square_of(rank, file);
            let piece = piece::from_initial(ch);
            pos.set_piece(piece, sq);
            file += 1;
        }
    }
}

fn parse_active_color(color_str: &str) -> Color {
    Color::from_initial(color_str.chars().next().unwrap())
}
