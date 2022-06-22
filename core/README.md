# watchmaker

A [genetic algorithm](https://en.wikipedia.org/wiki/Genetic_algorithm) library implementation in Rust.

[![CircleCI](https://circleci.com/gh/thomasbratt/watchmaker/tree/main.svg?style=svg)](https://circleci.com/gh/thomasbratt/watchmaker/tree/main)

## Features

* The API aims to be minimal and complete but allow extension with new genomes, selection operators and genetic operators
* Results include the best genome found, reason for stopping the search and some basic statistics
* Optional progress reporting
* Optional multithreaded evaluation of genomes using [Rayon](https://github.com/rayon-rs/rayon)
* Extensible cross-over partner selection via `Selector` trait.
* Diversity protection. Each genome is selected at least once; cross over partner chosen by `Selector` trait.
* Provided `TourniquetSelector` implementation simply selects the least cost partner from a random sample.
* Crossover
* Built-in crossover protection, to avoid the common bug where the first genome in a crossover operation is always used for the start of the resulting genome

## Usage

* Install Rust using [rustup](https://rustup.rs)
* Add an entry to the `[dependencies]` section of your `Cargo.toml` file from: <https://crates.io/crates/watchmaker>
* Implement the `Genetic` trait for your search problem and call `watchmaker::search`.

A complete example that searches for a specific floating point value:

```rust
    use rand::Rng;
    use watchmaker::*;
    
    pub const TARGET: f64 = 100.0;

    fn main() {
        println!("This is a very simple example that searches for the number 100.0.");
        let result = search(
            Box::new(PeakGenetic::default()),
            Box::new(TourniquetSelector::default()),
            Some(Box::new(|progress| {
                println!("{:?}", progress);
            })),
            &SearchSettings::default(),
        );
        println!("{:?}", result);
    }
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct PeakGenome(pub f64);
    
    #[derive(Default)]
    pub struct PeakGenetic {}
    
    impl Genetic<PeakGenome> for PeakGenetic {
        fn initialize(&self) -> PeakGenome {
            PeakGenome(rand::thread_rng().gen_range(0.0..1_000.0))
        }
    
        fn evaluate(&self, genome: &PeakGenome) -> f64 {
            (TARGET - genome.0).abs()
        }
    
        fn crossover(&self, lhs: &PeakGenome, rhs: &PeakGenome) -> PeakGenome {
            PeakGenome((lhs.0 + rhs.0) / 2.0)
        }
    
        fn mutate(&self, original: &PeakGenome) -> PeakGenome {
            PeakGenome(original.0 + rand::thread_rng().gen_range(-10.0..10.0))
        }
    }
```

## Development

* Clone the repository (see below)
* Run `cargo test` or `cargo build`

## Tests and Examples

* Run `cargo run --example peak`
* Run `cargo run --example weasel`
* Run `cargo run --example tsp`
* Run `cargo run --example hyper_weasel`
* Run `cargo run --example hyper_tsp`

Tests and examples are in a separate [tests](../tests) crate.
This arrangement allows code reuse between tests, examples and benches without affecting the [core](core) crate.

## Roadmap

Note major version increment with each major release.
API changes will not be backwards compatible between major releases.

- [ ] v3.x.x

* Fourth published version; Long Term Support
* Will take contributions, bug fixes from this point on.

- [ ] Unpublished; unsupported; beta quality

* Add hyperparameter detection feature
* Update examples
* Update benchmarks
* Update API docs
* Document `DetectConcurrencySettings`, `Selector` and `TournamentSelector` 
* Update README.md features 
* Update README.md examples
* Add crate level docs

- [x] v2.0.0

* Third published version; unsupported; beta quality
* Split tests, benches, examples into separate crate + workspace
* Add tests for `TournamentSelector`

- [x] Unpublished; unsupported; beta quality

* Split out algorithm that produces new generations
* Add Travelling Salesperson Problem
* Multithreading
* Improve benchmarking
* Fix bug with non-monotonic best cost / genomes

- [x] Unpublished; unsupported; beta quality

* Better typing around Progress
* Simpler and more complete code example in README.md and crate documentation
* Link to examples, with description and sample output

- [x] v1.x.x

* Second published version; unsupported; beta quality
* Randomly swap genome to crossover, to prevent bias towards individual genome
* Builder pattern for search settings
* Rustdoc
* Fix license - does not appear as 'standard' on crates.io

- [x] v0.1.0

* First published version; unsupported; beta quality

## Alternatives

* The Genetic Algorithm is a very well known technique:
<https://en.wikipedia.org/wiki/Genetic_algorithm>
* Rust Awesome has a list: <https://github.com/awesome-rust-com/awesome-rust#genetic-algorithms>
* Cargo: <https://crates.io/search?q=genetic%20algorithm>

## Contributing

Not accepting pull requests yet :)
See roadmap.

## License

MIT permissive license. See [LICENSE](LICENSE) for full license details.

## Source Code Repository

<https://github.com/thomasbratt/watchmaker>
