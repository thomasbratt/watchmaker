use watchmaker::{make_random, search, SettingsBuilder, WSGenetic};

// Show how the number of cross over candidates affects the results.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    for x in (1..=8).map(|exponent| 2_usize.pow(exponent)) {
        let result = search(
            Box::new(WSGenetic::new(make_random())),
            None,
            make_random(),
            &SettingsBuilder::default()
                .cross_over_candidates(x)
                .build()
                .unwrap(),
        );
        println!("cross_over_candidates:{:3}, {:?}", x, result);
    }
}
