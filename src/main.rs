#![allow(dead_code)]

mod benchmarks;
mod bit_boards;
mod engine;
mod game;
mod macros;

fn main() {
    _test_positions();
    // benchmarks::run();
}

fn _test_positions() {
    use crate::{engine, game::position::Position, game::position::debug::print_position};

    let fens = [
        // /* simple rook mate */
        // ("3k4/4R3/3K4/8/8/8/8/8 w - - 0 1", 7),
        // /*  Opera game */
        // ("4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 0 16", 7),
        /* ladder mate (68s) */
        ("5k2/8/6r1/7r/3K4/8/8/8 b - - 0 1", 10),
        // /* immortal bishop sac on g6 */
        // (
        //     "r1b2r2/p3Rpk1/2p1pNpp/1pP1P3/7q/5Q1P/5PP1/1B4K1 w - - 8 53",
        //     8,
        // ),
    ];

    for (fen, max_depth) in fens {
        let mut pos = Position::from_fen(fen).unwrap();
        print_position(&pos);

        macros::bench!({
            engine::run(&mut pos, max_depth, true);
        });

        println!("- - - - - - - - - -\n");
    }
}
