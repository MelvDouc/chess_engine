use crate::{
    constants::board_constants::{file_of, rank_of, BOARD_WIDTH},
    utils::bitboard::pop_right,
};

pub(super) fn square_control(color_attacks: u64) -> i32 {
    const SQUARE_VALUES: [[i32; BOARD_WIDTH]; BOARD_WIDTH] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 4, 4, 4, 4, 2, 1],
        [1, 2, 4, 6, 6, 4, 2, 1],
        [1, 2, 4, 6, 6, 4, 2, 1],
        [1, 2, 4, 4, 4, 4, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut control = 0i32;
    let mut bb = color_attacks;

    while bb > 0u64 {
        let sq = pop_right(&mut bb);
        control += SQUARE_VALUES[rank_of(sq)][file_of(sq)];
    }

    control
}
