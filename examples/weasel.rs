use std::time::Duration;
use watchmaker::*;

// Show progress reporting.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    println!("This is example that searches for the string 'METHINKS IT IS LIKE A WEASEL'.");
    let result = search(
        Box::new(WSGenetic::default()),
        Box::new(TournamentSelector::new(8).unwrap()),
        Some(Box::new(|snapshot| {
            println!("{:?}", snapshot);
        })),
        &SearchSettingsBuilder::default()
            .time_limit(Duration::from_secs(60))
            .epoch_limit(32_768)
            .build()
            .unwrap(),
    );
    println!("{:?}", result);
}
