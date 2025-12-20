use crate::{
    game::{
        board::{self, Board, colors, lines, pieces, squares, wings},
        moves::castling::{castling_bit, has_castling_right},
    },
    macros::ternary,
};

const DASH: &str = "-";

pub(super) fn parse_fen(fen: &str) -> Result<ParsedFEN, FENError> {
    let parts = fen.split(" ").collect::<Vec<&str>>();

    if parts.len() == 6 {
        let board = parse_board(parts[0])?;
        let active_color = parse_active_color(parts[1])?;
        let castling_rights = parse_castling_rights(parts[2])?;
        let ep_sq = parse_ep_square(parts[3])?;
        let half_move_clock = parse_half_move_clock(parts[4])?;

        return Ok((board, active_color, castling_rights, ep_sq, half_move_clock));
    }

    Err(FENError::InvalidFormat(fen.to_owned()))
}

pub(super) fn create_fen(
    board: &Board,
    active_color: usize,
    castling_rights: u8,
    ep_sq: usize,
    half_move_clock: u8,
) -> String {
    let b = stringify_board(board);
    let clr = colors::initial_of(active_color);
    let cr = stringify_castling_rights(castling_rights);
    let ep = stringify_ep_square(ep_sq);
    let hmc = half_move_clock;

    format!("{} {} {} {} {}", b, clr, cr, ep, hmc)
}

fn parse_board(str: &str) -> Result<Board, FENError> {
    let mut board = [pieces::NONE; board::NB_SQUARES];

    for (i, row) in str.split("/").enumerate() {
        let rank = squares::rev_coord(i);
        let mut file = lines::FILE_A;

        for ch in row.chars() {
            let piece = pieces::from_initial(ch);

            if piece == pieces::NONE {
                match ch.to_digit(10) {
                    Some(empty) => {
                        file += empty as usize;
                        continue;
                    }
                    None => return Err(FENError::InvalidBoard(str.to_owned(), ch)),
                };
            }

            board[squares::of(rank, file)] = piece;
            file += 1;
        }
    }

    Ok(board)
}

fn stringify_board(board: &Board) -> String {
    let mut rows = Vec::<String>::new();

    for rank in (0..board::NB_RANKS).rev() {
        let mut row = String::new();
        let mut empty: u32 = 0;

        for file in 0..board::NB_FILES {
            let piece = board[squares::of(rank, file)];

            if piece == pieces::NONE {
                empty += 1;
                continue;
            }

            if empty > 0 {
                row.push(char::from_digit(empty, 10).unwrap());
                empty = 0;
            }

            row.push(pieces::initial_of(piece));
        }

        if empty > 0 {
            row.push(char::from_digit(empty, 10).unwrap());
        }

        rows.push(row);
    }

    rows.join("/")
}

fn parse_active_color(str: &str) -> Result<usize, FENError> {
    if str.len() == 1 {
        let initial = str.chars().nth(0).unwrap();

        if let Ok(color) = colors::from_initial(initial) {
            return Ok(color);
        }
    }

    Err(FENError::InvalidColor(str.to_owned()))
}

fn parse_castling_rights(str: &str) -> Result<u8, FENError> {
    if str.eq(DASH) {
        return Ok(0);
    }

    let mut castling_rights: u8 = 0;

    for ch in str.chars() {
        match parse_castling_char(ch) {
            Some(bit) => {
                castling_rights |= bit;
            }
            None => return Err(FENError::InvalidCastlingRights(str.to_owned())),
        };
    }

    Ok(castling_rights)
}

fn parse_castling_char(ch: char) -> Option<u8> {
    match ch {
        'K' => Some(castling_bit(colors::WHITE, wings::KING_SIDE)),
        'Q' => Some(castling_bit(colors::WHITE, wings::QUEEN_SIDE)),
        'k' => Some(castling_bit(colors::BLACK, wings::KING_SIDE)),
        'q' => Some(castling_bit(colors::BLACK, wings::QUEEN_SIDE)),
        _ => None,
    }
}

pub(crate) fn stringify_castling_rights(castling_rights: u8) -> String {
    const INITIALS: [[char; board::NB_WINGS]; board::NB_COLORS] = [['Q', 'K'], ['q', 'k']];

    if castling_rights == 0 {
        return DASH.to_string();
    }

    let mut str = String::new();

    for color in 0..board::NB_COLORS {
        for wing in (0..board::NB_WINGS).rev() {
            if has_castling_right(castling_rights, color, wing) {
                str.push(INITIALS[color][wing]);
            }
        }
    }

    str
}

fn parse_ep_square(str: &str) -> Result<usize, FENError> {
    if str.eq(DASH) {
        return Ok(squares::NONE);
    }

    match squares::from_name(str) {
        Ok(sq) => Ok(sq),
        Err(_) => Err(FENError::InvalidSquare(str.to_owned())),
    }
}

pub(crate) fn stringify_ep_square(ep_sq: usize) -> String {
    ternary!(
        ep_sq == squares::NONE,
        DASH.to_string(),
        squares::name_of(ep_sq)
    )
}

fn parse_half_move_clock(str: &str) -> Result<u8, FENError> {
    match str.parse::<u8>() {
        Ok(x) => Ok(x),
        Err(_) => Err(FENError::InvalidHalfMoveClock(str.to_owned())),
    }
}

type ParsedFEN = (Board, usize, u8, usize, u8);

#[derive(Debug, Clone)]
pub(crate) enum FENError {
    InvalidSquare(String),
    InvalidColor(String),
    InvalidBoard(String, char),
    InvalidCastlingRights(String),
    InvalidHalfMoveClock(String),
    InvalidFormat(String),
}
