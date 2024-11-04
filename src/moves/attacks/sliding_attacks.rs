use std::cmp::min;

use lazy_static::lazy_static;

use crate::{
    constants::{
        board_constants::{file_of, rank_of, reverse_coord, NB_SQUARES},
        piece::{PTYPE_BISHOP, PTYPE_QUEEN, PTYPE_ROOK},
    },
    utils::bitboard::set_square,
};

const DIRECTION_NORTH_EAST: i8 = 7;
const DIRECTION_NORTH: i8 = 8;
const DIRECTION_NORTH_WEST: i8 = 9;
const DIRECTION_WEST: i8 = 1;
const DIRECTION_SOUTH_WEST: i8 = -7;
const DIRECTION_SOUTH: i8 = -8;
const DIRECTION_SOUTH_EAST: i8 = -9;
const DIRECTION_EAST: i8 = -1;

const DIRECTIONS: [[i8; 4]; 2] = [
    [
        DIRECTION_NORTH_EAST,
        DIRECTION_NORTH_WEST,
        DIRECTION_SOUTH_WEST,
        DIRECTION_SOUTH_EAST,
    ],
    [
        DIRECTION_NORTH,
        DIRECTION_WEST,
        DIRECTION_SOUTH,
        DIRECTION_EAST,
    ],
];

const SHIFTS: [[u8; NB_SQUARES]; 2] = [
    [
        6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7,
        5, 5, 5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5,
        5, 5, 5, 6,
    ],
    [
        12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10,
        11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10,
        10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
    ],
];

const PRECOMPUTED_MAGICS: [[u64; NB_SQUARES]; 2] = [
    [
        9368648609924554880,
        9009475591934976,
        4504776450605056,
        1130334595844096,
        1725202480235520,
        288516396277699584,
        613618303369805920,
        10168455467108368,
        9046920051966080,
        36031066926022914,
        1152925941509587232,
        9301886096196101,
        290536121828773904,
        5260205533369993472,
        7512287909098426400,
        153141218749450240,
        9241386469758076456,
        5352528174448640064,
        2310346668982272096,
        1154049638051909890,
        282645627930625,
        2306405976892514304,
        11534281888680707074,
        72339630111982113,
        8149474640617539202,
        2459884588819024896,
        11675583734899409218,
        1196543596102144,
        5774635144585216,
        145242600416216065,
        2522607328671633440,
        145278609400071184,
        5101802674455216,
        650979603259904,
        9511646410653040801,
        1153493285013424640,
        18016048314974752,
        4688397299729694976,
        9226754220791842050,
        4611969694574863363,
        145532532652773378,
        5265289125480634376,
        288239448330604544,
        2395019802642432,
        14555704381721968898,
        2324459974457168384,
        23652833739932677,
        282583111844497,
        4629880776036450560,
        5188716322066279440,
        146367151686549765,
        1153170821083299856,
        2315697107408912522,
        2342448293961403408,
        2309255902098161920,
        469501395595331584,
        4615626809856761874,
        576601773662552642,
        621501155230386208,
        13835058055890469376,
        3748138521932726784,
        9223517207018883457,
        9237736128969216257,
        1127068154855556,
    ],
    [
        612498416294952992,
        2377936612260610304,
        36037730568766080,
        72075188908654856,
        144119655536003584,
        5836666216720237568,
        9403535813175676288,
        1765412295174865024,
        3476919663777054752,
        288300746238222339,
        9288811671472386,
        146648600474026240,
        3799946587537536,
        704237264700928,
        10133167915730964,
        2305983769267405952,
        9223634270415749248,
        10344480540467205,
        9376496898355021824,
        2323998695235782656,
        9241527722809755650,
        189159985010188292,
        2310421375767019786,
        4647717014536733827,
        5585659813035147264,
        1442911135872321664,
        140814801969667,
        1188959108457300100,
        288815318485696640,
        758869733499076736,
        234750139167147013,
        2305924931420225604,
        9403727128727390345,
        9223970239903959360,
        309094713112139074,
        38290492990967808,
        3461016597114651648,
        181289678366835712,
        4927518981226496513,
        1155212901905072225,
        36099167912755202,
        9024792514543648,
        4611826894462124048,
        291045264466247688,
        83880127713378308,
        1688867174481936,
        563516973121544,
        9227888831703941123,
        703691741225216,
        45203259517829248,
        693563138976596032,
        4038638777286134272,
        865817582546978176,
        13835621555058516608,
        11541041685463296,
        288511853443695360,
        283749161902275,
        176489098445378,
        2306124759338845321,
        720584805193941061,
        4977040710267061250,
        10097633331715778562,
        325666550235288577,
        1100057149646,
    ],
];

lazy_static! {
    static ref BISHOP_GENERATOR: AttackGenerator = AttackGenerator::create(PTYPE_BISHOP);
    static ref ROOK_GENERATOR: AttackGenerator = AttackGenerator::create(PTYPE_ROOK);
}

pub(super) fn sliding_attacks(piece_type: usize, sq: usize, occ: u64) -> u64 {
    match piece_type {
        PTYPE_BISHOP => BISHOP_GENERATOR.attack_bitboard(sq, occ),
        PTYPE_ROOK => ROOK_GENERATOR.attack_bitboard(sq, occ),
        PTYPE_QUEEN => {
            sliding_attacks(PTYPE_ROOK, sq, occ) | sliding_attacks(PTYPE_BISHOP, sq, occ)
        }
        _ => panic!("Invalid piece type: {}", piece_type),
    }
}

fn distance_to_edge(sq: usize, direction: i8) -> usize {
    let rank = rank_of(sq);
    let file = file_of(sq);

    match direction {
        DIRECTION_EAST => file,
        DIRECTION_WEST => reverse_coord(file),
        DIRECTION_SOUTH => rank,
        DIRECTION_NORTH => reverse_coord(rank),
        DIRECTION_SOUTH_EAST => min(rank, file),
        DIRECTION_NORTH_WEST => min(reverse_coord(rank), reverse_coord(file)),
        DIRECTION_SOUTH_WEST => min(rank, reverse_coord(file)),
        DIRECTION_NORTH_EAST => min(reverse_coord(rank), file),
        _ => panic!("Invalid direction '{}'", direction),
    }
}

struct AttackGenerator {
    shifts: [u8; NB_SQUARES],
    masks: [u64; NB_SQUARES],
    magic_numbers: [u64; NB_SQUARES],
    offsets: [u64; NB_SQUARES],
    attack_table: Vec<u64>,
}

impl AttackGenerator {
    pub(super) fn create(piece_type: usize) -> Self {
        let pt_index = match piece_type {
            PTYPE_BISHOP => 0,
            PTYPE_ROOK => 1,
            _ => panic!("Invalid piece type: {}", piece_type),
        };
        let directions = DIRECTIONS[pt_index];
        let shifts = SHIFTS[pt_index];
        let magic_numbers = PRECOMPUTED_MAGICS[pt_index];
        let mut masks = [0u64; NB_SQUARES];
        let mut offsets = [0u64; NB_SQUARES];
        let mut attack_table = Vec::<u64>::new();

        for sq in 0..NB_SQUARES {
            masks[sq] = directions.iter().fold(0u64, |acc, &direction| {
                return acc | Self::mask_bits(sq, direction);
            });
            offsets[sq] = attack_table.len().try_into().unwrap();
            Self::fill_attack_table(
                &directions,
                sq,
                shifts[sq],
                magic_numbers[sq],
                offsets[sq] as u64,
                &mut attack_table,
            );
        }

        AttackGenerator {
            shifts,
            masks,
            magic_numbers,
            offsets,
            attack_table,
        }
    }

    fn add_direction(sq: usize, direction: i8) -> usize {
        ((sq as i8) + direction) as usize
    }

    fn mask_bits(mut sq: usize, direction: i8) -> u64 {
        let mut mask = 0u64;

        for _ in 1..distance_to_edge(sq, direction) {
            sq = Self::add_direction(sq, direction);
            set_square(&mut mask, sq);
        }

        mask
    }

    fn line_attack(mut sq: usize, direction: i8, occ: u64) -> u64 {
        let mut attack = 0u64;

        for _ in 0..distance_to_edge(sq, direction) {
            sq = Self::add_direction(sq, direction);
            set_square(&mut attack, sq);

            if occ & attack != 0u64 {
                break;
            }
        }

        attack
    }

    fn line_occupancies(sq: usize, direction: i8) -> Vec<u64> {
        let distance = distance_to_edge(sq, direction);
        let mut bbv = Vec::<u64>::new();

        if distance < 1 {
            return bbv;
        }

        let nb_occupancies = 1u64 << (distance - 1);

        for occ in 0..nb_occupancies {
            let mut bitboard = 0u64;
            let mut bit_mask = 1u64;
            let mut sq2 = sq;

            while bit_mask <= occ {
                sq2 = Self::add_direction(sq2, direction);
                bitboard |= (occ & bit_mask) >> bit_mask;
                bitboard <<= sq2;
                bit_mask <<= 1;
            }

            bbv.push(bitboard);
        }

        bbv
    }

    fn all_occupancies(sq: usize, directions: &[i8]) -> Vec<u64> {
        let mut occupancies = Vec::<u64>::new();

        for &direction in directions {
            let bbv = Self::line_occupancies(sq, direction);

            if bbv.is_empty() {
                continue;
            }

            if occupancies.is_empty() {
                occupancies = bbv;
                continue;
            }

            let mut tmp = Vec::<u64>::new();

            for bb in bbv {
                for &occ in &occupancies {
                    tmp.push(bb | occ);
                }
            }

            occupancies = tmp;
        }

        occupancies
    }

    fn fill_attack_table(
        directions: &[i8],
        sq: usize,
        shift: u8,
        magic_nb: u64,
        offset: u64,
        attack_table: &mut Vec<u64>,
    ) {
        let occupancies = Self::all_occupancies(sq, directions);

        for occ in occupancies {
            let occ_attacks = directions.iter().fold(0u64, |acc, &direction| {
                return acc | Self::line_attack(sq, direction, occ);
            });
            attack_table.push(occ_attacks);

            let index = Self::get_index(occ, magic_nb, shift, offset);
            assert_eq!(attack_table[index], occ_attacks);
        }
    }

    fn get_index(occ: u64, magic_nb: u64, shift: u8, offset: u64) -> usize {
        let index = occ.wrapping_mul(magic_nb) >> ((NB_SQUARES as u8) - shift);
        (index + offset) as usize
    }
}

impl AttackGenerator {
    pub(super) fn attack_bitboard(&self, sq: usize, occ: u64) -> u64 {
        self.attack_table[self.attack_table_index(sq, occ)]
    }

    fn attack_table_index(&self, sq: usize, occ: u64) -> usize {
        Self::get_index(
            occ,
            self.magic_numbers[sq],
            self.shifts[sq],
            self.offsets[sq],
        )
    }
}
