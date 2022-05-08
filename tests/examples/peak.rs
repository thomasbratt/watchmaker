use tests::*;
use watchmaker::*;

// Show how the results and progress reporting work.
// The genetic algorithm will search for the number 100.
/// The name comes from the fact that the cost function has its 'peak' (actually its lowest value)
/// at 100.
fn main() {
    println!("This is a very simple example that searches for the number 100.0.");
    let result = search(
        Box::new(PeakGenetic::default()),
        Box::new(TournamentSelector::default()),
        Some(Box::new(|snapshot| {
            println!("{:?}", snapshot);
        })),
        &SearchSettings::default(),
    );
    println!("{:?}", result);
}
