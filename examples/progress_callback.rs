use watchmaker::{make_random, search, Settings, WSGenetic};

// Show how the progress callback works.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    let _ = search(
        Box::new(WSGenetic::new(make_random())),
        Some(Box::new(|epoch, elapsed, best_cost, best_genome| {
            println!(
                "progress - epoch:{}, elapsed:{:?} best_cost:{} best_genome:{:?}",
                epoch, elapsed, best_cost, best_genome
            );
            true
        })),
        make_random(),
        &Settings::default(),
    );
}
