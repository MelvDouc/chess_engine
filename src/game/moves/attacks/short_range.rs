use crate::{
    bit_boards::bit_mask,
    game::board::{
        NB_FILES, NB_SQUARES,
        lines::{self, file_mask},
    },
    macros::const_while,
};

type AttackTable = [u64; NB_SQUARES];

const CLEAR_FILE_A: u64 = !file_mask(lines::FILE_A);
const CLEAR_FILE_B: u64 = !file_mask(lines::FILE_B);
const CLEAR_FILE_G: u64 = !file_mask(lines::FILE_G);
const CLEAR_FILE_H: u64 = !file_mask(lines::FILE_H);

macro_rules! create_table {
    ($func: expr) => {{
        let mut table = [0; NB_SQUARES];

        const_while!(sq, 0, NB_SQUARES, {
            table[sq] = $func(bit_mask(sq));
        });

        table
    }};
}

pub(super) const WHITE_PAWN_ATTACKS: AttackTable = create_table!(white_pawn_attacks);
pub(super) const BLACK_PAWN_ATTACKS: AttackTable = create_table!(black_pawn_attacks);
pub(super) const KNIGHT_ATTACKS: AttackTable = create_table!(knight_attacks);
pub(super) const KING_ATTACKS: AttackTable = create_table!(king_attacks);

const fn white_pawn_attacks(mask: u64) -> u64 {
    let nw = mask << (NB_FILES - 1) & CLEAR_FILE_H;
    let ne = mask << (NB_FILES + 1) & CLEAR_FILE_A;
    nw | ne
}

const fn black_pawn_attacks(mask: u64) -> u64 {
    let sw = mask >> (NB_FILES + 1) & CLEAR_FILE_H;
    let se = mask >> (NB_FILES - 1) & CLEAR_FILE_A;
    sw | se
}

const fn knight_attacks(mask: u64) -> u64 {
    let nnw = mask << (NB_FILES * 2 - 1);
    let ssw = mask >> (NB_FILES * 2 + 1);

    let nww = mask << (NB_FILES - 2);
    let sww = mask >> (NB_FILES + 2);

    let nne = mask << (NB_FILES * 2 + 1);
    let sse = mask >> (NB_FILES * 2 - 1);

    let nee = mask << (NB_FILES + 2);
    let see = mask >> (NB_FILES - 2);

    (nnw | ssw) & CLEAR_FILE_H
        | (nww | sww) & CLEAR_FILE_G & CLEAR_FILE_H
        | (nne | sse) & CLEAR_FILE_A
        | (nee | see) & CLEAR_FILE_A & CLEAR_FILE_B
}

const fn king_attacks(mask: u64) -> u64 {
    let n = mask << NB_FILES;
    let s = mask >> NB_FILES;

    let nw = mask << (NB_FILES - 1);
    let w = mask >> 1;
    let sw = mask >> (NB_FILES + 1);

    let ne = mask << (NB_FILES + 1);
    let e = mask << 1;
    let se = mask >> (NB_FILES - 1);

    n | s | (nw | w | sw) & CLEAR_FILE_H | (ne | e | se) & CLEAR_FILE_A
}
