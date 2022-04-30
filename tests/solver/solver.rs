use std::time::Duration;

use watchmaker::{make_random, Solver, WSGenetic, WSGenome};

#[test]
fn adhoc() {
    let mut solver: Solver<WSGenome> = Solver::new(
        Box::new(WSGenetic::new(make_random())),
        0.00,
        1_000,
        100,
        Duration::from_secs(5),
        0.0,
        // Some(Box::new(|epoch, elapsed, best_cost, best_genome| {
        //     eprintln!(
        //         "progress - epoch:{}, elapsed:{:?} best_cost:{} best_genome:{:?}",
        //         epoch, elapsed, best_cost, best_genome
        //     );
        //     true
        // })),
        None,
    );

    let results = solver.solve();
    eprintln!("{:?}", results)
}
