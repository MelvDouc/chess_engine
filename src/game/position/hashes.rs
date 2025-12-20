use crate::{
    bit_boards::Prng,
    game::{
        board::{NB_COLORS, NB_PIECES, NB_SQUARES},
        moves::castling::NB_BITS_CASTLING_RIGHTS,
    },
    macros::const_while,
};

const NB_PIECE_HASHES: usize = NB_PIECES * NB_SQUARES;
const NB_COLOR_HASHES: usize = NB_COLORS;
const NB_CASTLING_HASHES: usize = 1 << NB_BITS_CASTLING_RIGHTS;

const OFFSET_COLOR: usize = NB_PIECE_HASHES;
const OFFSET_CASTLING: usize = OFFSET_COLOR + NB_COLOR_HASHES;

const TABLE_SIZE: usize = NB_PIECE_HASHES + NB_COLOR_HASHES + NB_CASTLING_HASHES;

/// Zobrist hashes used to get a (mostly) unique representation of a given position.
const HASHES: [u64; TABLE_SIZE] = {
    let mut rng = Prng::new(1234);
    let mut table = [0; TABLE_SIZE];

    const_while!(i, 0, TABLE_SIZE, {
        table[i] = rng.next();
    });

    table
};

pub(super) const fn piece(piece: usize, sq: usize) -> u64 {
    HASHES[piece * NB_SQUARES + sq]
}

pub(super) const fn color(color: usize) -> u64 {
    HASHES[color + OFFSET_COLOR]
}

pub(super) const fn castling(castling_rights: u8) -> u64 {
    HASHES[castling_rights as usize + OFFSET_CASTLING]
}

pub(super) const fn en_passant(sq: usize) -> u64 {
    // reusing piece hashes for simplicity
    HASHES[sq] >> 1
}
