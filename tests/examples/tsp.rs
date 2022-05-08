use std::time::Duration;
use tests::*;
use watchmaker::*;

/// This example will search for solutions to a well-known TSP problem using various concurrency settings.
fn main() {
    println!("This example will search for solutions to a well-known TSP problem ('Berlin 52') using various concurrency settings.");
    run(ConcurrencySettings::SingleThreaded);
    run(ConcurrencySettings::MultiThreaded);
}

fn run(concurrency: ConcurrencySettings) {
    let result = search(
        Box::new(TspGenetic::default()),
        Box::new(TournamentSelector::new(15).unwrap()),
        None,
        &SearchSettingsBuilder::default()
            .concurrency(concurrency)
            .population_size(32_768)
            .mutation_probability(0.01)
            .time_limit(Duration::from_secs(60))
            .epoch_limit(1_000_000)
            .build()
            .unwrap(),
    )
    .unwrap();
    println!("{:?} {:?}", concurrency, result);
}
