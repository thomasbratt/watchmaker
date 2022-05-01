use std::time::Duration;

use watchmaker::{make_random, solve, Failure, WSGenetic, TARGET};

#[test]
fn search_finds_result_for_simple_test_case() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        Some(Box::new(|epoch, elapsed, best_cost, best_genome| {
            eprintln!(
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

    eprintln!("{:?}", result);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.as_ref().ok().unwrap().best_cost, 0.0);
    assert_eq!(result.as_ref().ok().unwrap().best_genome.0, TARGET);
}

#[test]
fn fails_when_cross_over_candidates_too_low() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        0,
        0,
        0.0,
        1_000,
        Duration::from_secs(5),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_candidates());
}

#[test]
fn fails_when_epoch_limit_too_low() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        3,
        0,
        0.0,
        1_000,
        Duration::from_secs(5),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::epoch_limit());
}

#[test]
fn fails_when_mutation_rate_is_negative() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        3,
        1_000,
        -1.0,
        1_000,
        Duration::from_secs(5),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::mutation_rate());
}

#[test]
fn fails_when_population_size_is_too_small() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        3,
        1_000,
        0.0,
        0,
        Duration::from_secs(5),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::population_size());
}

#[test]
fn fails_when_time_limit_is_zero() {
    let result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.00,
        3,
        10_000,
        0.0,
        1_000,
        Duration::ZERO,
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::time_limit());
}
