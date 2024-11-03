use crate::{
    constants::{
        board_constants::{file_of, rank_of, square_of, FILE_E, NB_PIECES, NB_SQUARES},
        piece::{get_piece, KING, NONE_PIECE, PAWN, PROMOTION_TYPES, QUEEN, WHITE_PAWN},
        wing, Color,
    },
    moves::{
        get_pawn_pushes,
        move_encoding::{castling_move, en_passant_move, normal_move, promotion_move},
        piece_attacks,
    },
    utils::bitboard::{bitboard_of, clear_square, pop_right, set_square},
};

use super::{
    castling::{are_castling_squares_ok, get_castling_right},
    play_moves::{play_move, undo_move},
    position_hash::*,
    undo_move_info::*,
};

pub(crate) struct Position {
    occupancies: [u64; NB_PIECES + 2],
    hash: u64,
    active_color: Color,
    castling_rights: u8,
    en_passant_square: usize,
    pub(crate) half_move_clock: u8,
    pub(crate) full_move_number: u16,
}

impl Position {
    pub(crate) fn create(
        active_color: Color,
        castling_rights: u8,
        en_passant_square: usize,
        half_move_clock: u8,
        full_move_number: u16,
    ) -> Self {
        let occupancies: [u64; NB_PIECES + 2] = [0; NB_PIECES + 2];
        let hash = color_hash()
            ^ castling_hash(castling_rights)
            ^ en_passant_hash(file_of(en_passant_square));
        Position {
            occupancies,
            hash,
            active_color,
            castling_rights,
            en_passant_square,
            half_move_clock,
            full_move_number,
        }
    }

    pub(crate) const fn get_active_color(&self) -> Color {
        self.active_color
    }

    pub(crate) const fn inactive_color(&self) -> Color {
        self.active_color.reverse()
    }

    pub(crate) fn set_active_color(&mut self, color: Color) {
        if color != self.active_color {
            self.hash ^= color_hash();
            self.active_color = color;
        }
    }

    pub(crate) fn is_white_to_move(&self) -> bool {
        self.active_color.is_white()
    }

    pub(crate) fn has_castling(&self, color: Color, wing: usize) -> bool {
        self.castling_rights & get_castling_right(color, wing) != 0
    }

    pub(crate) fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub(crate) fn set_castling_rights(&mut self, castling_rights: u8) {
        if castling_rights != self.castling_rights {
            self.hash ^= castling_hash(self.castling_rights) ^ castling_hash(castling_rights);
            self.castling_rights = castling_rights;
        }
    }

    pub(crate) fn get_en_passant_square(&self) -> usize {
        self.en_passant_square
    }

    pub(crate) fn set_en_passant_square(&mut self, en_passant_sq: usize) {
        if en_passant_sq != self.en_passant_square {
            self.hash ^= en_passant_hash(self.en_passant_square) ^ en_passant_hash(en_passant_sq);
            self.en_passant_square = en_passant_sq;
        }
    }

    pub(crate) const fn get_hash(&self) -> u64 {
        self.hash
    }

    pub(crate) const fn has_piece_on(&self, piece: usize, sq: usize) -> bool {
        self.occupancies[piece] & bitboard_of(sq) != 0
    }

    pub(crate) fn set_piece(&mut self, piece: usize, sq: usize) {
        set_square(&mut self.occupancies[piece], sq);
        set_square(
            &mut self.occupancies[NB_PIECES + Color::piece_color(piece) as usize],
            sq,
        );
        self.hash ^= piece_hash(piece, sq);
    }

    pub(crate) fn unset_piece(&mut self, piece: usize, sq: usize) {
        clear_square(&mut self.occupancies[piece], sq);
        clear_square(
            &mut self.occupancies[NB_PIECES + Color::piece_color(piece) as usize],
            sq,
        );
        self.hash ^= piece_hash(piece, sq);
    }

    /// Returns a U64 where every 4 digits is the number of pieces at the corresponding index.
    pub(crate) fn piece_count(&self) -> u64 {
        let mut piece_count = 0u64;

        for piece in WHITE_PAWN..NONE_PIECE {
            let count = self.occupancies[piece].count_ones() as u64;
            piece_count |= count << (4 * piece);
        }

        piece_count
    }

    pub(crate) fn color_attacks(&self, color: Color) -> u64 {
        let occupancy = self.full_occupancy();
        let mut attacks = 0u64;

        for piece_type in PAWN..=QUEEN {
            let piece = get_piece(piece_type, color);
            let mut occ = self.occupancies[piece];

            while occ != 0u64 {
                let sq = pop_right(&mut occ);
                attacks |= piece_attacks(piece, sq, occupancy);
            }
        }

        attacks
    }

    pub(crate) fn is_check(&self) -> bool {
        let king_bb = self.piece_occupancy(KING, self.active_color);
        let occupancy = self.full_occupancy();

        for piece_type in PAWN..=QUEEN {
            let piece = get_piece(piece_type, self.inactive_color());
            let mut occ = self.occupancies[piece];

            while occ != 0u64 {
                let sq = pop_right(&mut occ);

                if piece_attacks(piece, sq, occupancy) & king_bb != 0u64 {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn legal_moves(&mut self) -> Vec<u32> {
        let mut legal_moves = self.pseudo_legal_moves();
        let undo_info = self.undo_move_info();
        legal_moves.retain(|&mv| {
            self.play_move(mv, false);
            let is_check = self.is_check();
            self.undo_move(mv, undo_info);
            !is_check
        });
        self.add_castling_moves(&mut legal_moves);
        legal_moves
    }

    fn active_occupancy(&self) -> u64 {
        self.occupancies[NB_PIECES + self.active_color as usize]
    }

    fn inactive_occupancy(&self) -> u64 {
        self.occupancies[NB_PIECES + self.inactive_color() as usize]
    }

    fn full_occupancy(&self) -> u64 {
        self.active_occupancy() | self.inactive_occupancy()
    }

    fn piece_occupancy(&self, piece_type: usize, color: Color) -> u64 {
        self.occupancies[get_piece(piece_type, color)]
    }

    fn pseudo_legal_moves(&self) -> Vec<u32> {
        let mut moves = Vec::<u32>::new();
        let not_active_occ = !self.active_occupancy();
        let occupancy = self.full_occupancy();
        let inactive_occ_array = self.inactive_occ_array();

        self.add_pawn_moves(&mut moves, occupancy, inactive_occ_array);
        self.add_piece_moves(&mut moves, occupancy, not_active_occ, inactive_occ_array);

        moves
    }
}

impl Position {
    fn inactive_occ_array(&self) -> [usize; NB_SQUARES] {
        let mut arr = [NONE_PIECE; NB_SQUARES];

        for piece_type in PAWN..=QUEEN {
            let piece = get_piece(piece_type, self.active_color);
            let mut occ = self.occupancies[piece];

            while occ != 0u64 {
                let sq = pop_right(&mut occ);
                arr[sq] = piece;
            }
        }

        arr
    }

    fn add_pawn_pushes(&self, moves: &mut Vec<u32>, pawn: usize, src_sq: usize, occupancy: u64) {
        let mut pushes = get_pawn_pushes(src_sq, self.active_color, occupancy);

        while pushes != 0u64 {
            let dest_sq = pop_right(&mut pushes);

            if rank_of(dest_sq) == self.inactive_color().initial_piece_rank() {
                self.add_promotions(moves, pawn, src_sq, dest_sq, NONE_PIECE);
                continue;
            }

            let mv = normal_move(src_sq, dest_sq, pawn, NONE_PIECE);
            moves.push(mv);
        }
    }

    fn add_pawn_captures(
        &self,
        moves: &mut Vec<u32>,
        pawn: usize,
        src_sq: usize,
        inactive_occ: [usize; NB_SQUARES],
    ) {
        let occ_mask = self.inactive_occupancy() | bitboard_of(self.en_passant_square);
        let mut attacks = piece_attacks(pawn, src_sq, self.full_occupancy()) & occ_mask;

        while attacks != 0u64 {
            let dest_sq = pop_right(&mut attacks);

            if rank_of(dest_sq) == self.inactive_color().initial_piece_rank() {
                self.add_promotions(moves, pawn, src_sq, dest_sq, inactive_occ[dest_sq]);
                continue;
            }

            if dest_sq == self.en_passant_square {
                let captured = get_piece(PAWN, self.inactive_color());
                let mv = en_passant_move(src_sq, dest_sq, pawn, captured);
                moves.push(mv);
                continue;
            }

            let mv = normal_move(src_sq, dest_sq, pawn, inactive_occ[dest_sq]);
            moves.push(mv);
        }
    }

    fn add_promotions(
        &self,
        moves: &mut Vec<u32>,
        pawn: usize,
        src_sq: usize,
        dest_sq: usize,
        captured: usize,
    ) {
        for piece_type in PROMOTION_TYPES {
            let promoted = get_piece(piece_type, self.active_color);
            let mv = promotion_move(src_sq, dest_sq, pawn, captured, promoted);
            moves.push(mv);
        }
    }

    fn add_pawn_moves(
        &self,
        moves: &mut Vec<u32>,
        occupancy: u64,
        inactive_occ_array: [usize; NB_SQUARES],
    ) {
        let pawn = get_piece(PAWN, self.active_color);
        let mut occ = self.occupancies[pawn];

        while occ != 0u64 {
            let src_sq = pop_right(&mut occ);
            self.add_pawn_pushes(moves, pawn, src_sq, occupancy);
            self.add_pawn_captures(moves, pawn, src_sq, inactive_occ_array);
        }
    }

    fn add_piece_moves(
        &self,
        moves: &mut Vec<u32>,
        occupancy: u64,
        not_active_occ: u64,
        inactive_occ_array: [usize; NB_SQUARES],
    ) {
        for piece_type in PAWN + 1..=QUEEN {
            let src_piece = get_piece(piece_type, self.active_color);
            let mut occ = self.occupancies[src_piece];

            while occ != 0u64 {
                let src_sq = pop_right(&mut occ);
                let mut attacks = piece_attacks(src_piece, src_sq, occupancy) & not_active_occ;

                while attacks != 0u64 {
                    let dest_sq = pop_right(&mut attacks);
                    let mv = normal_move(src_sq, dest_sq, src_piece, inactive_occ_array[dest_sq]);
                    moves.push(mv);
                }
            }
        }
    }

    fn add_castling_moves(&self, legal_moves: &mut Vec<u32>) {
        let rank = self.active_color.initial_piece_rank();
        let king_src_sq = square_of(rank, FILE_E);
        let king_bb = self.piece_occupancy(KING, self.active_color);

        if king_bb != bitboard_of(king_src_sq) {
            return;
        }

        let enemy_attacks = self.color_attacks(self.inactive_color());

        if enemy_attacks & king_bb != 0u64 {
            return;
        }

        let king = get_piece(KING, self.active_color);

        for wing in wing::WINGS {
            if self.has_castling(self.active_color, wing)
                && are_castling_squares_ok(wing, rank, self.full_occupancy(), enemy_attacks)
            {
                let mv = castling_move(king_src_sq, wing, king);
                legal_moves.push(mv);
            }
        }
    }
}

impl Position {
    /// Move the pieces according to the move parameters.
    /// It also updates the <u>castling rights</u> and <u>en passant square</u>.
    /// The active is not automatically updated in order to test for check.
    pub(crate) fn play_move(&mut self, mv: u32, toggle_color: bool) {
        play_move(self, mv);

        if toggle_color {
            self.set_active_color(self.inactive_color());
        }
    }

    /// Put the pieces back where they were before playing the move.
    /// It also resets the position parameters like castling rights or the en passant square.
    pub(crate) fn undo_move(&mut self, mv: u32, undo_info: u32) {
        undo_move(self, mv);
        reset_from_move_info(self, undo_info);
    }

    pub(crate) fn undo_move_info(&self) -> u32 {
        encode_undo_info(self)
    }
}
