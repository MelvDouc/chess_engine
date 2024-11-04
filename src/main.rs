#![allow(dead_code)]

use game::fen_string::parse_fen;

mod constants;
mod engine;
mod game;
mod moves;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    _test_fen("k7/8/K1R5/8/8/8/8/8 w - - 0 1"); // KR vs K (mate in 1 ply)

    // _test_fen("4k3/8/5K2/8/8/8/8/5R2 w - - 0 1"); // KR vs K (mate in 3 plies)

    // _test_fen("1Q6/8/2K5/8/k7/8/8/8 w - - 0 1"); // KQ vs K (mate in 3 plies)

    // _test_fen("8/1PK5/k7/8/8/8/8/8 w - - 0 1"); // KP vs K (mate in 5 plies)

    // _test_fen("8/k1P5/2K5/8/8/8/8/8 w - - 0 1"); // KP vs K (mate in 3 plies; rook underpromotion)

    // _test_fen("2r3k1/R4R2/7P/8/8/8/5PK1/8 w - - 0 1"); // 2R vs R (mate in 5 plies)

    // _test_fen("2b2b1r/4qk1p/5P2/3p1Q2/2B1P3/2BP4/5PP1/4K2R w - - 0 1"); // Damiano defense (mate in 7 plies)
}

fn _test_fen(fen: &str) {
    let mut pos = parse_fen(fen);
    let (score, best_moves) = engine::eval(&mut pos);
    println!("--------");
    println!("score={:.2}", score);

    for mv in best_moves {
        print!("{} ", moves::move_encoding::move_notation(mv));
    }

    println!("\n--------");
}
