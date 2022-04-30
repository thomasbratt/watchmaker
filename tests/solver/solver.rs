use std::time::Duration;

use watchmaker::{make_random, solve, WSGenetic};

#[test]
fn adhoc() {
    let results = solve(
        Box::new(WSGenetic::new(make_random())),
        // Some(Box::new(|epoch, elapsed, best_cost, best_genome| {
        //     eprintln!(
        //         "progress - epoch:{}, elapsed:{:?} best_cost:{} best_genome:{:?}",
        //         epoch, elapsed, best_cost, best_genome
        //     );
        //     true
        // })),
        None,
        0.00,
        1_000,
        0.0,
        100,
        Duration::from_secs(5),
    );

    eprintln!("{:?}", results)
}
