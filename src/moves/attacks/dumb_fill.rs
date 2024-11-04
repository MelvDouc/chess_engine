use crate::{
    constants::{
        board_constants::{file_of, rank_of, reverse_coord as reverse, BOARD_WIDTH},
        piece::{PTYPE_BISHOP, PTYPE_QUEEN, PTYPE_ROOK},
    },
    utils::{bitboard::set_square, math::usize_min},
};

const BISHOP_DIRECTIONS: [Direction; 4] = [
    Direction::NorthWest,
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::SouthWest,
];

const ROOK_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

const QUEEN_DIRECTIONS: [Direction; 8] = [
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
];

pub(crate) fn sliding_attacks(piece_type: u8, sq: usize, occ: u64) -> u64 {
    match piece_type {
        PTYPE_BISHOP => attacks_in_directions(sq, &BISHOP_DIRECTIONS, occ),
        PTYPE_ROOK => attacks_in_directions(sq, &ROOK_DIRECTIONS, occ),
        PTYPE_QUEEN => attacks_in_directions(sq, &QUEEN_DIRECTIONS, occ),
        _ => panic!("Invalid piece type"),
    }
}

fn attacks_in_directions(sq: usize, directions: &[Direction], occ: u64) -> u64 {
    directions.iter().fold(0u64, |acc, direction| {
        return acc | direction.get_attacks(sq, occ);
    })
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn get_attacks(&self, mut sq: usize, occ: u64) -> u64 {
        let mut attacks = 0u64;

        for _ in 0..self.distance_to_edge(sq) {
            sq = self.next_square(sq);
            set_square(&mut attacks, sq);

            if occ & attacks != 0u64 {
                break;
            }
        }

        attacks
    }

    const fn next_square(&self, sq: usize) -> usize {
        match self {
            Self::North => sq + BOARD_WIDTH,
            Self::South => sq - BOARD_WIDTH,
            Self::East => sq + 1,
            Self::West => sq - 1,
            Self::NorthEast => sq + BOARD_WIDTH + 1,
            Self::NorthWest => sq + BOARD_WIDTH - 1,
            Self::SouthEast => sq - BOARD_WIDTH + 1,
            Self::SouthWest => sq - BOARD_WIDTH - 1,
        }
    }

    const fn distance_to_edge(&self, sq: usize) -> usize {
        let rank = rank_of(sq);
        let file = file_of(sq);

        match self {
            Self::North => reverse(rank),
            Self::South => rank,
            Self::East => reverse(file),
            Self::West => file,
            Self::NorthEast => usize_min(reverse(rank), reverse(file)),
            Self::NorthWest => usize_min(reverse(rank), file),
            Self::SouthEast => usize_min(rank, reverse(file)),
            Self::SouthWest => usize_min(rank, file),
        }
    }
}
