use watchmaker::{make_random, search, SettingsBuilder, Success, WSGenetic, WSGenome};

// Show the effect of search hyperparameters on search times.
// The genetic algorithm will search for the string 'weasel'.
// This example will take a long time to run.
fn main() {
    let mut top: Vec<(u128, usize, usize, Option<Success<WSGenome>>)> =
        (1..=10).map(|_| (u128::MAX, 0, 0, None)).collect();
    for population_size in (1..=16).map(|exponent| 2_usize.pow(exponent)) {
        for cross_over_candidates in (1..=8).map(|exponent| 2_usize.pow(exponent)) {
            let result = search(
                Box::new(WSGenetic::new(make_random())),
                None,
                make_random(),
                &SettingsBuilder::default()
                    .cross_over_candidates(cross_over_candidates)
                    .population_size(population_size)
                    .build()
                    .unwrap(),
            )
            .unwrap();

            println!(
                "(population:{:5}, candidates:{:3}) -> (cost:{:5},elapsed:{:?}) ",
                population_size, cross_over_candidates, result.best_cost, result.elapsed
            );

            if result.best_cost == 0.0 {
                let metric = result.epoch as u128 * result.elapsed.as_millis();
                top.push((metric, population_size, cross_over_candidates, Some(result)));
                top.sort_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());
                top.remove(top.len() - 1);
            }
        }
    }
    println!("Top {} hyperparameters", top.len());
    for item in top {
        println!(
            "population:{:5}, candidates:{:3} -> {:?}",
            item.1,
            item.2,
            item.3.unwrap()
        );
    }
}
