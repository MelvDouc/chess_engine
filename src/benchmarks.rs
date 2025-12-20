use colored::Colorize;

use crate::{engine, game::position::Position};

pub(crate) fn run() {
    println!("Running benchmarks...\n");
    bench_move_generation();
    bench_analyze_start_pos();
}

fn benchmark(name: &str, iterations: usize, mut func: impl FnMut() -> ()) {
    let mut counts = Vec::<u128>::new();
    println!("- {}", name.green());

    for _ in 0..iterations {
        let instant = std::time::Instant::now();
        func();
        let elapsed = instant.elapsed().as_millis();
        counts.push(elapsed);
    }

    let max = *counts.iter().max().unwrap();
    let min = *counts.iter().min().unwrap();
    let total: u128 = counts.iter().sum();
    let average = total as f32 / iterations as f32;

    println!("  * average time: {:.2} ms", average);
    println!("  * min: {} ms, max: {} ms", min, max);
}

fn bench_move_generation() {
    let fen = "3Q4/1Q4Q1/4Q3/2Q4R/Q4Q2/3Q4/1Q4Rp/1K1BBNNk w - - 0 1";
    let pos = Position::from_fen(fen).unwrap();

    benchmark("move generation", 100, || {
        pos.legal_moves();
    });
}

fn bench_analyze_start_pos() {
    let mut pos = Position::from_fen(Position::START_FEN).unwrap();
    let max_depth = 4;
    let name = format!("analyze start position (d={})", max_depth);

    benchmark(&name, 10, || {
        engine::run(&mut pos, max_depth, false);
    });
}
