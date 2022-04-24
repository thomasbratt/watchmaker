use crate::common::make_random;
use crate::solver::wsgenome::WSGenome;
use std::time::Duration;
use watchmaker::Solver;

// Test
#[test]
fn adhoc() {
    let solver: Solver<WSGenome> = Solver::new(
        make_random(),
        0.01,
        1_000,
        100_000,
        Duration::from_secs(10),
        0.0,
    );

    solver.solve();
}
