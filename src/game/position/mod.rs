pub(crate) mod fen;
mod gen_moves;
mod hashes;
mod play_move;
mod repetitions;
mod undo_info;

#[cfg(test)]
mod tests;

use crate::{
    bit_boards::{clear_bit, set_bit, set_bits},
    errors::FENError,
    game::{
        board::{Board, NB_COLORS, NB_PIECE_TYPES, NB_PIECES, NB_SQUARES, colors, pieces, squares},
        moves::{Move, MoveList, castling::castling_color_mask, piece_attacks},
    },
    macros::const_while,
};

use self::repetitions as reps;

pub(crate) struct Position {
    board: Board,
    piece_occupancies: [u64; NB_PIECES],
    color_occupancies: [u64; NB_COLORS],
    active_color: usize,
    castling_rights: u8,
    en_passant_sq: usize,
    half_move_clock: u8,
    hash: u64,
    rep_table: reps::Table,
}

impl Position {
    pub(crate) const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub(crate) fn from_fen(fen: &str) -> Result<Self, FENError> {
        #[rustfmt::skip]
        let (
            board,
            active_color,
            castling_rights,
            en_passant_sq,
            half_move_clock
        ) = fen::parse_fen(fen)?;

        let mut pos = Self {
            board: [pieces::NONE; NB_SQUARES],
            active_color,
            castling_rights,
            en_passant_sq,
            half_move_clock,
            piece_occupancies: [0; NB_PIECES],
            color_occupancies: [0; NB_COLORS],
            rep_table: reps::create(),
            hash: 0,
        };

        for (sq, &piece) in board.iter().enumerate() {
            if piece != pieces::NONE {
                pos.set_piece(sq, piece);
            }
        }

        pos.hash ^= hashes::color(active_color);
        pos.hash ^= hashes::castling(castling_rights);
        pos.hash ^= hashes::en_passant(en_passant_sq);
        reps::increment(&mut pos.rep_table, pos.hash);

        Ok(pos)
    }

    pub(crate) fn to_fen(&self) -> String {
        fen::create_fen(
            &self.board,
            self.active_color,
            self.castling_rights,
            self.en_passant_sq,
            self.half_move_clock,
        )
    }
}

impl Position {
    pub(crate) const fn get_active_color(&self) -> usize {
        self.active_color
    }

    pub(self) const fn set_active_color(&mut self, color: usize) {
        self.hash ^= hashes::color(self.active_color);
        self.hash ^= hashes::color(color);
        self.active_color = color;
    }

    pub(crate) const fn inactive_color(&self) -> usize {
        colors::rev(self.active_color)
    }

    pub(self) const fn toggle_active_color(&mut self) {
        self.set_active_color(self.inactive_color());
    }

    pub(crate) const fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub(self) const fn set_castling_rights(&mut self, castling_rights: u8) {
        if castling_rights != self.castling_rights {
            self.hash ^= hashes::castling(self.castling_rights);
            self.hash ^= hashes::castling(castling_rights);
            self.castling_rights = castling_rights;
        }
    }

    pub(self) const fn can_color_castle(&self, color: usize) -> bool {
        self.castling_rights & castling_color_mask(color) != 0
    }

    pub(crate) const fn get_ep_square(&self) -> usize {
        self.en_passant_sq
    }

    pub(self) const fn set_ep_square(&mut self, sq: usize) {
        if sq != self.en_passant_sq {
            self.hash ^= hashes::en_passant(self.en_passant_sq);
            self.hash ^= hashes::en_passant(sq);
            self.en_passant_sq = sq;
        }
    }

    pub(crate) const fn half_move_clock(&self) -> u8 {
        self.half_move_clock
    }

    pub(crate) const fn hash(&self) -> u64 {
        self.hash
    }

    pub(crate) const fn color_occupancy(&self, color: usize) -> u64 {
        self.color_occupancies[color]
    }

    pub(crate) const fn active_occupancy(&self) -> u64 {
        self.color_occupancy(self.active_color)
    }

    pub(crate) const fn inactive_occupancy(&self) -> u64 {
        self.color_occupancy(self.inactive_color())
    }

    pub(crate) const fn piece_occupancy(&self, piece: usize) -> u64 {
        self.piece_occupancies[piece]
    }

    pub(crate) const fn piece_occupancy2(&self, piece_type: usize, color: usize) -> u64 {
        self.piece_occupancy(pieces::of(piece_type, color))
    }

    pub(crate) const fn full_occupancy(&self) -> u64 {
        self.color_occupancy(colors::WHITE) | self.color_occupancy(colors::BLACK)
    }

    pub(crate) const fn pawn_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::pawn_of(color))
    }

    pub(crate) const fn knight_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::knight_of(color))
    }

    pub(crate) const fn bishop_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::bishop_of(color))
    }

    pub(crate) const fn rook_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::rook_of(color))
    }

    pub(crate) const fn queen_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::queen_of(color))
    }

    pub(crate) const fn king_occupancy(&self, color: usize) -> u64 {
        self.piece_occupancy(pieces::king_of(color))
    }

    pub(crate) const fn king_square(&self, color: usize) -> usize {
        self.king_occupancy(color).trailing_zeros() as usize
    }

    pub(crate) const fn piece_count(&self) -> u8 {
        self.full_occupancy().count_ones() as u8
    }

    pub(crate) const fn undo_info(&self) -> undo_info::UndoInfo {
        undo_info::encode(
            self.castling_rights,
            self.en_passant_sq,
            self.half_move_clock,
        )
    }

    pub(crate) const fn rep_count(&self) -> u8 {
        reps::read(&self.rep_table, self.hash)
    }

    pub(crate) const fn reset_reps(&mut self) {
        reps::reset(&mut self.rep_table);
    }

    pub(crate) const fn get_piece(&self, sq: usize) -> usize {
        self.board[sq]
    }

    pub(self) const fn set_piece(&mut self, sq: usize, piece: usize) {
        self.board[sq] = piece;
        set_bit!(self.piece_occupancies[piece], sq);
        set_bit!(self.color_occupancies[pieces::color_of(piece)], sq);
        self.hash ^= hashes::piece(piece, sq);
    }

    /// Assumes `piece != pieces::NONE`.
    pub(self) const fn remove_piece(&mut self, sq: usize) {
        let piece = self.board[sq];

        self.board[sq] = pieces::NONE;
        clear_bit!(self.piece_occupancies[piece], sq);
        clear_bit!(self.color_occupancies[pieces::color_of(piece)], sq);
        self.hash ^= hashes::piece(piece, sq);
    }

    pub(crate) const fn is_check(&self) -> bool {
        let color = self.active_color;
        let opp_color = self.inactive_color();
        let king_sq = self.king_square(color);

        if piece_attacks(pieces::pawn_of(color), king_sq, 0) & self.pawn_occupancy(opp_color) != 0 {
            return true;
        }

        let full_occ = self.full_occupancy();

        const_while!(piece_type, 1, NB_PIECE_TYPES, {
            let piece = pieces::of(piece_type, opp_color);

            if piece_attacks(piece, king_sq, full_occ) & self.piece_occupancy(piece) != 0 {
                return true;
            }
        });

        false
    }

    /// Returns not only the attacked squares but also those X-rayed through the opposing king.
    const fn color_attacks(&self, color: usize) -> u64 {
        let occ = self.full_occupancy() & !self.king_occupancy(colors::rev(color));
        let mut attacks = 0;

        set_bits!(self.color_occupancy(color), sq, {
            attacks |= piece_attacks(self.board[sq], sq, occ);
        });

        attacks
    }

    pub(crate) const fn legal_moves(&self) -> MoveList {
        gen_moves::legal_moves(self)
    }

    pub(crate) const fn play_move(&mut self, mv: Move) {
        play_move::play_move(self, mv);
        play_move::update_info(self, mv);
        reps::increment(&mut self.rep_table, self.hash);
    }

    pub(crate) const fn undo_move(&mut self, mv: Move, undo_info: undo_info::UndoInfo) {
        reps::decrement(&mut self.rep_table, self.hash);

        play_move::undo_move(self, mv);
        self.toggle_active_color();
        self.set_castling_rights(undo_info::castling_rights(undo_info));
        self.set_ep_square(undo_info::ep_square(undo_info));
        self.half_move_clock = undo_info::half_move_clock(undo_info);
    }

    pub(crate) const fn play_null_move(&mut self) {
        self.toggle_active_color();
        self.set_ep_square(squares::NONE);
    }

    pub(crate) const fn undo_null_move(&mut self, ep_sq: usize) {
        self.toggle_active_color();
        self.set_ep_square(ep_sq);
    }
}
