use tests::*;
use watchmaker::*;

/// Show how results and progress reporting work.
/// The genetic algorithm will search for the number 100.0.
/// The name comes from the fact that the cost function has its 'peak' (actually its
/// lowest cost) at 100.0.
fn main() {
    println!("This is a very simple example that searches for the number 100.0.");
    let result = search(
        Box::new(PeakGenetic::default()),
        Box::new(TournamentSelector::default()),
        Some(Box::new(|progress| {
            println!("{:?}", progress);
        })),
        &SearchSettings::default(),
    );
    println!("{:?}", result);
}
