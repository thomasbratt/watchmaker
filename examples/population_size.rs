use watchmaker::{make_random, search, SettingsBuilder, WSGenetic};

// Show how population size affects the results.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    for x in (1..=16).map(|exponent| 2_usize.pow(exponent)) {
        let result = search(
            Box::new(WSGenetic::new(make_random())),
            None,
            make_random(),
            &SettingsBuilder::default()
                .population_size(x)
                .build()
                .unwrap(),
        );
        println!("population:{:5}, {:?}", x, result);
    }
}
