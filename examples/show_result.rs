use watchmaker::{make_random, search, Settings, WSGenetic};

// Show how the results work.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    let result = search(
        Box::new(WSGenetic::new(make_random())),
        None,
        make_random(),
        &Settings::default(),
    );
    println!("{:?}", result);
}
