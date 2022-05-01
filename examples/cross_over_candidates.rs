use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

// Show how the number of cross over candidates affects the results.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    for cross_over_candidates in (1..=8).map(|exponent| 2_usize.pow(exponent)) {
        let result = solve(
            Box::new(WSGenetic::new(make_random())),
            None,
            0.00,
            cross_over_candidates,
            1_000,
            0.0,
            1_000,
            Duration::from_secs(15),
        );
        println!(
            "cross_over_candidates:{:3}, {:?}",
            cross_over_candidates, result
        );
    }
}
