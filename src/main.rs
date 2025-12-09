#![allow(dead_code)]

mod bit_boards;
mod debug;
mod engine;
mod game;
mod macros;
mod errors;

fn main() {
    _main();
}

fn _main() {
    use crate::{debug::print_position, engine::run, game::position::Position};

    let fens = [
        /* simple rook mate */
        // "3k4/4R3/3K4/8/8/8/8/8 w - - 0 1",
        /* ladder mate */
        // "5k2/8/6r1/7r/3K4/8/8/8 b - - 0 1",
        /*  Opera game */
        "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 0 16",
    ];

    for fen in fens {
        let mut pos = Position::from_fen(fen).unwrap();
        print_position(&pos);

        macros::bench!({
            run(&mut pos, 7);
        });

        println!("- - - - - - - - - -\n");
    }
}
