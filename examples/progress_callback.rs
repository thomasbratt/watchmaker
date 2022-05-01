use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

// Show how the progress callback works.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    let _ = solve(
        Box::new(WSGenetic::new(make_random())),
        Some(Box::new(|epoch, elapsed, best_cost, best_genome| {
            println!(
                "progress - epoch:{}, elapsed:{:?} best_cost:{} best_genome:{:?}",
                epoch, elapsed, best_cost, best_genome
            );
            true
        })),
        0.00,
        32,
        100,
        0.0,
        1_000,
        Duration::from_secs(5),
    );
}
