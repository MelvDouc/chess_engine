#![allow(dead_code)]

mod constants;
mod engine;
mod game;
mod moves;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    let mut diagonal = 0u64;

    for i in 0..8 {
        diagonal |= utils::bitboard::bitboard_from_square(i, i);
    }

    println!("{}", diagonal);
    utils::debug::print_bitboard(diagonal);
}

fn _test() {
    let fen = "k7/8/K1R5/8/8/8/8/8 w - - 0 1";
    // let fen = "4k3/8/5K2/8/8/8/8/5R2 w - - 0 1";
    // let fen = "8/7p/3p1k2/2p5/p1P1bN1P/P5P1/1P3K2/8 w - - 0 40";
    // let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut pos = game::fen_string::parse_fen(fen);
    let (score, depth) = engine::eval(&mut pos);
    println!("score: {}", (score as f64) / 1000.0);
    println!("depth: {}", depth);
}
