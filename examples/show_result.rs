use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

// Show how the results work.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        32,
        100,
        0.0,
        1_000,
        Duration::from_secs(5),
    );
    println!("{:?}", result);
}
