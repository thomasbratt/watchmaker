use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

// Show how population size affects the results.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    for population_size in (1..=16).map(|exponent| 2_usize.pow(exponent)) {
        let result = solve(
            Box::new(WSGenetic::new(make_random())),
            None,
            0.00,
            32,
            100,
            0.0,
            population_size,
            Duration::from_secs(5),
        );
        println!("population:{:5}, {:?}", population_size, result);
    }
}
