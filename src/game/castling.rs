use crate::{
    constants::{
        board_constants::{FILE_B, FILE_C, FILE_D, FILE_F, FILE_G},
        wing, Color,
    },
    utils::bitboard::bitboard_from_square as bb_from_sq,
};

const NO_CASTLING: u8 = 0u8;
const WHITE_KING_SIDE: u8 = 1;
const WHITE_QUEEN_SIDE: u8 = 1 << 1;
const BLACK_KING_SIDE: u8 = 1 << 2;
const BLACK_QUEEN_SIDE: u8 = 1 << 3;

const INITIALS: [(char, u8); 4] = [
    ('K', WHITE_KING_SIDE),
    ('Q', WHITE_QUEEN_SIDE),
    ('k', BLACK_KING_SIDE),
    ('q', BLACK_QUEEN_SIDE),
];

pub(crate) const fn get_castling_right(color: Color, wing: u8) -> u8 {
    const BY_COLOR_AND_WING: [[u8; 2]; 2] = [
        [WHITE_KING_SIDE, WHITE_QUEEN_SIDE],
        [BLACK_KING_SIDE, BLACK_QUEEN_SIDE],
    ];

    BY_COLOR_AND_WING[color as usize][wing as usize]
}

pub(super) fn parse_castling_rights(str: &str) -> u8 {
    if str.eq("-") {
        return NO_CASTLING;
    }

    let mut castling_rights = NO_CASTLING;

    for ch in str.chars() {
        for (initial, value) in INITIALS {
            if ch == initial {
                castling_rights |= value;
                break;
            }
        }
    }

    castling_rights
}

pub(crate) fn stringify_castling_rights(castling_rights: u8) -> String {
    if castling_rights == NO_CASTLING {
        return String::from("-");
    }

    let mut str = String::new();

    for (initial, value) in INITIALS {
        if castling_rights & value != NO_CASTLING {
            str.push(initial);
        }
    }

    str
}

pub(super) fn are_castling_squares_ok(
    wing: u8,
    rank: usize,
    occupancy: u64,
    enemy_attacks: u64,
) -> bool {
    match wing {
        wing::QUEEN_SIDE => {
            let attack_mask = bb_from_sq(rank, FILE_C) | bb_from_sq(rank, FILE_D);
            let occ_mask = attack_mask | bb_from_sq(rank, FILE_B);
            (enemy_attacks & attack_mask == 0u64) && (occupancy & occ_mask == 0u64)
        }
        wing::KING_SIDE => {
            let mask = bb_from_sq(rank, FILE_F) | bb_from_sq(rank, FILE_G);
            (enemy_attacks & mask == 0u64) && (occupancy & mask == 0u64)
        }
        _ => panic!("Invalid castling wing"),
    }
}
