use crate::{
    constants::{
        board_constants::{rank_of, BOARD_WIDTH, NB_SQUARES},
        square, Color,
    },
    utils::bitboard::bitboard_of,
};

const PAWN_PUSHES: [[u64; NB_SQUARES]; 2] = create_pushes();

const fn create_pushes() -> [[u64; NB_SQUARES]; 2] {
    let mut pawn_pushes: [[u64; NB_SQUARES]; 2] = [[0u64; NB_SQUARES]; 2];
    let mut c = 0;

    while c <= 1 {
        let color = if c == 0 { Color::White } else { Color::Black };
        c += 1;

        let mut sq = square::A2;

        while sq <= square::H7 {
            let push = shift_push(bitboard_of(sq), color);

            if rank_of(sq) == color.initial_pawn_rank() {
                pawn_pushes[color as usize][sq] = push | shift_push(push, color);
            } else {
                pawn_pushes[color as usize][sq] = push;
            }

            sq += 1;
        }
    }

    pawn_pushes
}

const fn shift_push(push: u64, color: Color) -> u64 {
    match color {
        Color::White => push << BOARD_WIDTH,
        Color::Black => push >> BOARD_WIDTH,
    }
}

pub(crate) const fn get_pawn_pushes(sq: usize, color: Color, occupancy: u64) -> u64 {
    // A push is never possible when the first dest square is occupied.
    if occupancy & shift_push(bitboard_of(sq), color) != 0u64 {
        return 0u64;
    }

    PAWN_PUSHES[color as usize][sq]
}
