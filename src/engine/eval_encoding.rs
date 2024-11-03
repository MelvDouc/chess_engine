// bits 0-19  = score abs value
// bits 20-25 = depth
// bit 31     = score sign

/// The number of bits in the score's absolute value.
const NBITS_SCORE: i32 = 20;
const NBITS_DEPTH: i32 = 6;

const OFFSET_DEPTH: i32 = NBITS_SCORE;
const MASK_DEPTH: i32 = ((1 << NBITS_DEPTH) - 1) << OFFSET_DEPTH;

pub(super) const fn encode_eval(score: i32, depth: usize) -> i32 {
    score | (depth as i32) << OFFSET_DEPTH
}

pub(super) const fn decode_score(ev: i32) -> i32 {
    ev & !MASK_DEPTH
}

pub(super) const fn decode_depth(ev: i32) -> usize {
    (ev & MASK_DEPTH) as usize
}
