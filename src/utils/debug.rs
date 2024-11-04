use crate::{
    constants::{
        board_constants::{file_of, square_of, FILE_A, FILE_H, RANK_1, RANK_8},
        piece::{piece_initial, NONE_PIECE, WHITE_PAWN},
        square,
    },
    game::{castling::stringify_castling_rights, Position},
};

use super::bitboard::bitboard_of;

pub(crate) fn print_binary_string(bb: u64) {
    for sq in (square::A1..=square::H8).rev() {
        print!("{}", (bb & bitboard_of(sq)) >> sq);

        if file_of(sq) == 0 {
            print!(" ");
        }
    }
}

pub(crate) fn print_bitboard(bb: u64) {
    for rank in (RANK_1..=RANK_8).rev() {
        for file in FILE_A..=FILE_H {
            let sq = square_of(rank, file);
            let square_bb = bitboard_of(sq);
            print!("{} ", (bb & square_bb) >> sq);
        }

        print!("\n");
    }

    print!("\n");
}

pub(crate) fn print_board(pos: &Position) {
    print!("\n");

    for rank in (RANK_1..=RANK_8).rev() {
        for file in FILE_A..=FILE_H {
            let sq = square_of(rank, file);
            let mut ch = '.';

            for piece in WHITE_PAWN..NONE_PIECE {
                if pos.has_piece_on(piece, sq) {
                    ch = piece_initial(piece);
                    break;
                }
            }

            print!("{} ", ch);
        }

        print!("\n");
    }

    let castling_rights = stringify_castling_rights(pos.get_castling_rights());
    println!("--------");
    println!("color: {}", pos.get_active_color().initial());
    println!("castling rights: {}", castling_rights);
    println!("e.p.: {}", square::name_of(pos.get_en_passant_square()));
}
