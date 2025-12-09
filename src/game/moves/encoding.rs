use crate::game::{board::pieces, moves::Move};

type MoveKind = Move;

pub(crate) mod move_kinds {
    use super::MoveKind;

    pub(crate) const NORMAL: MoveKind = 0;
    pub(crate) const EN_PASSANT: MoveKind = 1;
    pub(crate) const PROMOTION: MoveKind = 2;
    pub(crate) const CASTLING: MoveKind = 3;
}

const NB_BITS_MOVE_KIND: usize = 2;
const NB_BITS_SQUARE: usize = 6;
const NB_BITS_PIECE: usize = 4;

const OFFSET_SRC_SQUARE: usize = NB_BITS_MOVE_KIND;
const OFFSET_DEST_SQUARE: usize = OFFSET_SRC_SQUARE + NB_BITS_SQUARE;
const OFFSET_SRC_PIECE: usize = OFFSET_DEST_SQUARE + NB_BITS_SQUARE;
const OFFSET_CAPTURED: usize = OFFSET_SRC_PIECE + NB_BITS_PIECE;
const OFFSET_PROMOTED: usize = OFFSET_CAPTURED + NB_BITS_PIECE;
const OFFSET_GIVES_CHECK: usize = OFFSET_PROMOTED + NB_BITS_PIECE;

const MASK_MOVE_KIND: Move = (1 << NB_BITS_MOVE_KIND) - 1;
const MASK_SQUARE: Move = (1 << NB_BITS_SQUARE) - 1;
const MASK_PIECE: Move = (1 << NB_BITS_PIECE) - 1;

pub(crate) const fn normal_move(
    src_sq: usize,
    dest_sq: usize,
    src_piece: usize,
    captured: usize,
) -> Move {
    (src_sq << OFFSET_SRC_SQUARE
        | dest_sq << OFFSET_DEST_SQUARE
        | src_piece << OFFSET_SRC_PIECE
        | captured << OFFSET_CAPTURED) as Move
}

pub(crate) const fn en_passant_move(
    src_sq: usize,
    dest_sq: usize,
    pawn: usize,
    captured: usize,
) -> Move {
    move_kinds::EN_PASSANT | normal_move(src_sq, dest_sq, pawn, captured)
}

pub(crate) const fn promotion_move(
    src_sq: usize,
    dest_sq: usize,
    pawn: usize,
    captured: usize,
    promoted: usize,
) -> Move {
    move_kinds::PROMOTION
        | normal_move(src_sq, dest_sq, pawn, captured)
        | (promoted << OFFSET_PROMOTED) as Move
}

pub(crate) const fn castling_move(src_sq: usize, dest_sq: usize, king: usize) -> Move {
    move_kinds::CASTLING | normal_move(src_sq, dest_sq, king, pieces::NONE)
}

pub(crate) const fn move_kind(mv: Move) -> MoveKind {
    mv & MASK_MOVE_KIND
}

pub(crate) const fn src_square(mv: Move) -> usize {
    (mv >> OFFSET_SRC_SQUARE & MASK_SQUARE) as usize
}

pub(crate) const fn dest_square(mv: Move) -> usize {
    (mv >> OFFSET_DEST_SQUARE & MASK_SQUARE) as usize
}

pub(crate) const fn src_piece(mv: Move) -> usize {
    (mv >> OFFSET_SRC_PIECE & MASK_PIECE) as usize
}

pub(crate) const fn captured(mv: Move) -> usize {
    (mv >> OFFSET_CAPTURED & MASK_PIECE) as usize
}

pub(crate) const fn promoted(mv: Move) -> usize {
    (mv >> OFFSET_PROMOTED & MASK_PIECE) as usize
}

pub(crate) const fn gives_check(mv: Move) -> bool {
    (mv & 1 << OFFSET_GIVES_CHECK) != 0
}

pub(crate) const fn mark_check(mv: Move) -> Move {
    mv | 1 << OFFSET_GIVES_CHECK
}

pub(crate) const fn is_capture(mv: Move) -> bool {
    captured(mv) != pieces::NONE
}

pub(crate) const fn is_en_passant(mv: Move) -> bool {
    move_kind(mv) == move_kinds::EN_PASSANT
}

pub(crate) const fn is_promotion(mv: Move) -> bool {
    move_kind(mv) == move_kinds::PROMOTION
}

pub(crate) const fn is_castling(mv: Move) -> bool {
    move_kind(mv) == move_kinds::CASTLING
}

#[cfg(test)]
mod tests {
    use crate::game::board::{NB_COLORS, NB_PIECE_TYPES, NB_SQUARES, colors, pieces};

    use rand::random_range;

    fn random_color() -> usize {
        random_range(0..NB_COLORS)
    }

    fn random_piece(color: usize) -> usize {
        pieces::of(random_range(0..NB_PIECE_TYPES), color)
    }

    fn random_square() -> usize {
        random_range(0..NB_SQUARES)
    }

    #[test]
    fn normal_move() {
        let src_sq = random_square();
        let dest_sq = random_square();
        let color = random_color();
        let src_piece = random_piece(color);
        let captured = random_piece(colors::rev(color));
        let mv = super::normal_move(src_sq, dest_sq, src_piece, captured);

        assert_eq!(super::move_kind(mv), super::move_kinds::NORMAL);
        assert_eq!(super::src_square(mv), src_sq);
        assert_eq!(super::dest_square(mv), dest_sq);
        assert_eq!(super::src_piece(mv), src_piece);
        assert_eq!(super::captured(mv), captured);
    }

    #[test]
    fn en_passant_move() {
        let src_sq = random_square();
        let dest_sq = random_square();
        let src_piece = pieces::pawn_of(random_color());
        let captured = pieces::rev_color(src_piece);
        let mv = super::en_passant_move(src_sq, dest_sq, src_piece, captured);

        assert_eq!(super::move_kind(mv), super::move_kinds::EN_PASSANT);
        assert_eq!(super::src_square(mv), src_sq);
        assert_eq!(super::dest_square(mv), dest_sq);
        assert_eq!(super::src_piece(mv), src_piece);
        assert_eq!(super::captured(mv), captured);
    }

    #[test]
    fn promotion() {
        let src_sq = random_square();
        let dest_sq = random_square();
        let color = random_color();
        let pawn = pieces::pawn_of(color);
        let captured = random_piece(colors::rev(color));
        let promoted = pawn + (random_range(0..4) + 1) * 2;
        let mv = super::promotion_move(src_sq, dest_sq, pawn, captured, promoted);

        assert_eq!(super::move_kind(mv), super::move_kinds::PROMOTION);
        assert_eq!(super::src_square(mv), src_sq);
        assert_eq!(super::dest_square(mv), dest_sq);
        assert_eq!(super::src_piece(mv), pawn);
        assert_eq!(super::captured(mv), captured);
        assert_eq!(super::promoted(mv), promoted);
    }

    #[test]
    fn castling() {
        let src_sq = random_square();
        let dest_sq = random_square();
        let king = pieces::king_of(random_color());
        let mv = super::castling_move(src_sq, dest_sq, king);

        assert_eq!(super::move_kind(mv), super::move_kinds::CASTLING);
        assert_eq!(super::src_square(mv), src_sq);
        assert_eq!(super::dest_square(mv), dest_sq);
        assert_eq!(super::src_piece(mv), king);
        assert_eq!(super::captured(mv), pieces::NONE);
    }
}
