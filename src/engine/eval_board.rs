use crate::{
    constants::{
        board_constants::{file_of, rank_of, BOARD_WIDTH},
        piece::{get_piece, BISHOP, KNIGHT, PAWN, QUEEN, ROOK},
        Color,
    },
    game::Position,
    utils::bitboard::pop_right,
};

const MASK4: u64 = (1 << 4) - 1;
const PIECE_TYPE_VALUES: [u64; 6] = [1, 3, 0, 3, 5, 10];

pub(super) fn evaluate_board(pos: &Position) -> i32 {
    if pos.half_move_clock >= 50 {
        return 0;
    }

    let active_color = pos.get_active_color();
    let inactive_color = pos.inactive_color();
    let piece_count = pos.piece_count();

    if !has_material_to_mate(active_color, inactive_color, piece_count)
        && !has_material_to_mate(inactive_color, active_color, piece_count)
    {
        return 0;
    }

    let white_material_count = material_count(piece_count, Color::White);
    let white_square_control = square_control(pos.color_attacks(Color::White));
    let white_score = white_material_count * 1000 + white_square_control * 100;

    let black_material_count = material_count(piece_count, Color::Black);
    let black_square_control = square_control(pos.color_attacks(Color::Black));
    let black_score = black_material_count * 1000 + black_square_control * 100;

    white_score - black_score
}

const fn get_count(piece_count: PieceCount, piece_type: usize, color: Color) -> u64 {
    let piece = get_piece(piece_type, color);
    piece_count >> (4 * piece) & MASK4
}

fn material_count(piece_count: PieceCount, color: Color) -> i32 {
    let mut m_count = 0;

    for piece_type in PAWN..=QUEEN {
        m_count += get_count(piece_count, piece_type, color) * PIECE_TYPE_VALUES[piece_type];
    }

    m_count.try_into().unwrap()
}

/// Returns whether a player has at least a pawn, rook or queen.
fn has_lone_mating_piece(color: Color, piece_count: PieceCount) -> bool {
    get_count(piece_count, PAWN, color) > 0
        || get_count(piece_count, ROOK, color) > 0
        || get_count(piece_count, QUEEN, color) > 0
}

fn has_non_king_piece(color: Color, piece_count: PieceCount) -> bool {
    has_lone_mating_piece(color, piece_count)
        || get_count(piece_count, KNIGHT, color) > 0
        || get_count(piece_count, BISHOP, color) > 0
}

fn has_material_to_mate(color: Color, inactive_color: Color, piece_count: PieceCount) -> bool {
    let nb_bishops = get_count(piece_count, BISHOP, color);

    if has_lone_mating_piece(color, piece_count) || nb_bishops > 1 {
        return true;
    }

    match get_count(piece_count, KNIGHT, color) {
        0 => false,
        1 => nb_bishops > 0,
        2 => nb_bishops > 0 || has_non_king_piece(inactive_color, piece_count),
        _ => true,
    }
}

fn square_control(color_attacks: u64) -> i32 {
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

type PieceCount = u64;
