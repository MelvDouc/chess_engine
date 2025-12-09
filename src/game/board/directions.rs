use crate::{
    bit_boards::{bit_mask, set_bits},
    game::board::{NB_DIRECTIONS, NB_FILES, NB_SQUARES, squares},
    macros::{const_while, ternary},
};

pub(crate) const NORTH: usize = 0;
pub(crate) const SOUTH: usize = 1;
pub(crate) const EAST: usize = 2;
pub(crate) const WEST: usize = 3;
pub(crate) const NORTH_EAST: usize = 4;
pub(crate) const SOUTH_WEST: usize = 5;
pub(crate) const NORTH_WEST: usize = 6;
pub(crate) const SOUTH_EAST: usize = 7;
pub(crate) const NONE: usize = 8;

pub(crate) const fn rev(dir: usize) -> usize {
    dir ^ 1
}

pub(crate) const fn is_orthogonal(dir: usize) -> bool {
    dir <= WEST
}

pub(crate) const fn is_diagonal(dir: usize) -> bool {
    !is_orthogonal(dir)
}

pub(crate) const fn distance_to_edge(sq: usize, dir: usize) -> u8 {
    const EDGE_DISTANCES: [[u8; NB_DIRECTIONS]; NB_SQUARES] = {
        const fn min(x: u8, y: u8) -> u8 {
            ternary!(x < y, x, y)
        }

        let mut table = [[0; NB_DIRECTIONS]; NB_SQUARES];

        const_while!(sq, 0, NB_SQUARES, {
            let rank = squares::rank_of(sq);
            let file = squares::file_of(sq);

            table[sq][NORTH] = squares::rev_coord(rank) as u8;
            table[sq][SOUTH] = rank as u8;
            table[sq][EAST] = squares::rev_coord(file) as u8;
            table[sq][WEST] = file as u8;
            table[sq][NORTH_EAST] = min(table[sq][NORTH], table[sq][EAST]);
            table[sq][SOUTH_WEST] = min(table[sq][SOUTH], table[sq][WEST]);
            table[sq][NORTH_WEST] = min(table[sq][NORTH], table[sq][WEST]);
            table[sq][SOUTH_EAST] = min(table[sq][SOUTH], table[sq][EAST]);
        });

        table
    };

    EDGE_DISTANCES[sq][dir]
}

pub(crate) const fn next_square(sq: usize, dir: usize) -> usize {
    match dir {
        NORTH => sq + NB_FILES,
        SOUTH => sq - NB_FILES,
        EAST => sq + 1,
        WEST => sq - 1,
        NORTH_EAST => sq + NB_FILES + 1,
        SOUTH_WEST => sq - NB_FILES - 1,
        NORTH_WEST => sq + NB_FILES - 1,
        SOUTH_EAST => sq - NB_FILES + 1,
        _ => panic!("Invalid direction"),
    }
}

pub(crate) const fn first_occupied_square(occ: u64, dir: usize) -> usize {
    match dir {
        NORTH | EAST | NORTH_EAST | NORTH_WEST => occ.trailing_zeros() as usize,
        SOUTH | WEST | SOUTH_WEST | SOUTH_EAST => occ.leading_zeros() as usize ^ squares::H8,
        _ => panic!("Invalid direction"),
    }
}

/// A table of bit boards filling the board from every square to every direction.
/// The square isn't part of the ray.
const RAYS: [[u64; NB_DIRECTIONS]; NB_SQUARES] = {
    const fn fill_ray(dir: usize, mut sq: usize) -> u64 {
        let mut dist = distance_to_edge(sq, dir);
        let mut ray = 0;

        while dist > 0 {
            sq = next_square(sq, dir);
            ray |= bit_mask(sq);
            dist -= 1;
        }

        ray
    }

    let mut table = [[0; NB_DIRECTIONS]; NB_SQUARES];

    const_while!(sq, 0, NB_SQUARES, {
        const_while!(dir, 0, NB_DIRECTIONS, {
            table[sq][dir] = fill_ray(dir, sq);
        });
    });

    table
};

pub(crate) const fn ray_of(sq: usize, dir: usize) -> u64 {
    RAYS[sq][dir]
}

pub(crate) const fn get(sq1: usize, sq2: usize) -> usize {
    const SQUARE_DIRECTIONS: [[usize; NB_SQUARES]; NB_SQUARES] = {
        let mut table = [[NONE; NB_SQUARES]; NB_SQUARES];

        const_while!(src_sq, 0, NB_SQUARES, {
            const_while!(dir, 0, NB_DIRECTIONS, {
                set_bits!(ray_of(src_sq, dir), dest_sq, {
                    table[src_sq][dest_sq] = dir;
                });
            });
        });

        table
    };

    SQUARE_DIRECTIONS[sq1][sq2]
}
