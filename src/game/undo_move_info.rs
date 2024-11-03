// bit 0      - active color
// bits 1-4   - castling rights
// bits 5-11  - en passant square
// bits 12-17 - half move clock
// bits 18-26 - full move number

use crate::constants::Color;

use super::Position;

const NBITS_COLOR: u32 = 1;
const NBITS_CASTLING: u32 = 4;
const NBITS_SQUARE: u32 = 7;
const NBITS_HMC: u32 = 6;
const NBITS_FMN: u32 = 9;

const OFFSET_CASTLING: u32 = NBITS_COLOR;
const OFFSET_EP: u32 = OFFSET_CASTLING + NBITS_CASTLING;
const OFFSET_HMC: u32 = OFFSET_EP + NBITS_SQUARE;
const OFFSET_FMN: u32 = OFFSET_HMC + NBITS_HMC;

const MASK_CASTLING: u32 = (1 << NBITS_CASTLING) - 1;
const MASK_SQUARE: u32 = (1 << NBITS_SQUARE) - 1;
const MASK_HMC: u32 = (1 << NBITS_HMC) - 1;

pub(super) fn encode_undo_info(pos: &Position) -> u32 {
    let color = pos.get_active_color() as u32;
    let castling_rights = pos.get_castling_rights() as u32;
    let en_passant_sq = pos.get_en_passant_square() as u32;
    let half_move_clock = pos.half_move_clock as u32;
    let full_move_number = pos.full_move_number as u32;

    color
        | castling_rights << OFFSET_CASTLING
        | en_passant_sq << OFFSET_EP
        | half_move_clock << OFFSET_HMC
        | full_move_number << OFFSET_FMN
}

pub(super) fn reset_from_move_info(pos: &mut Position, undo_info: u32) {
    if undo_info & 1 == 0 {
        pos.set_active_color(Color::White);
    } else {
        pos.set_active_color(Color::Black);
    };

    pos.set_castling_rights((undo_info >> OFFSET_CASTLING & MASK_CASTLING) as u8);
    pos.set_en_passant_square((undo_info >> OFFSET_EP & MASK_SQUARE) as usize);
    pos.half_move_clock = (undo_info >> OFFSET_HMC & MASK_HMC) as u8;
    pos.full_move_number = (undo_info >> OFFSET_FMN) as u16;
}
