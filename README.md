# watchmaker

A [genetic algorithm](https://en.wikipedia.org/wiki/Genetic_algorithm) implementation in Rust.

[![CircleCI](https://circleci.com/gh/thomasbratt/watchmaker/tree/main.svg?style=svg)](https://circleci.com/gh/thomasbratt/watchmaker/tree/main)

## Features

* Developed as an investigation into capabilities and implementation characteristics
* Written in the Rust programming language
* The API aims to be minimal and complete
* Built-in crossover protection, to avoid the common bug where the first genome in a crossover operation is always used for the start of the resulting genome
* Some features are missing (see Roadmap section)

## Usage

* Install Rust using `rustup` <https://rustup.rs/>
* Add a reference to your `Cargo.toml` file from: <https://crates.io/crates/watchmaker>
* Implement the `Genetic` trait for your search problem and call `watchmaker::search`.

```rust
    pub fn search<G>(
        mut genetic: Box<dyn Genetic<G>>,
        mut progress: Option<Progress<G>>,
        mut random: Random,
        settings: &Settings,
    ) -> Result<Success<G>, Failure>
```

Example that searches for an optimal floating point value:

```rust
use watchmaker::*;

fn main() {
    let result = search(
        Box::new(ExampleGenetic::new(make_random())),
        Some(Box::new(|x| {
            println!("progress:{:?}", x);
        })),
        make_random(),
        &Settings::default(),
    );
    println!("{:?}", result);
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExampleGenome(pub f64);

pub const TARGET: f64 = 100.0;

pub struct ExampleGenetic {
    random: Random,
}

impl ExampleGenetic {
    pub fn new(random: Random) -> Self {
        Self { random }
    }
}

impl Genetic<ExampleGenome> for ExampleGenetic {
    fn initialize(&mut self) -> ExampleGenome {
        ExampleGenome(self.random.gen_range(0.0..1_000.0))
    }

    fn evaluate(&mut self, genome: &ExampleGenome) -> f64 {
        (TARGET - genome.0).abs()
    }

    fn crossover(&mut self, lhs: &ExampleGenome, rhs: &ExampleGenome) -> ExampleGenome {
        ExampleGenome((lhs.0 + rhs.0) / 2.0)
    }

    fn mutate(&mut self, original: &ExampleGenome) -> ExampleGenome {
        ExampleGenome(original.0 + self.random.gen_range(-10.0..10.0))
    }
}
```

## Development

* Clone the repository (see below)
* Run `cargo test` or `cargo build`

## Examples

* Run `cargo run --example peak`
* Run `cargo run --example weasel`
* Run `cargo run --example hyperparameter_grid_search`

## Roadmap

Note major version increment with each major release.
API changes will not be backwards compatible between major releases.

- [ ] v4.x.x

* Fifth published version
* Long Term Support
* Update features section
* Will take contributions, bug fixes from this point on.
* 
- [x] v3.x.x

* Fourth published version
* Split out algorithm that produces new generations
* Multithreading
* Unsupported

- [x] v2.x.x

* Better typing around Progress
* Simpler and more complete code example in README.md and crate documentation
* Link to examples, with description and sample output
* Unpublished, unsupported

- [x] v1.x.x

* Second published version (beta quality)
* Randomly swap genome to crossover, to prevent bias towards individual genome
* Builder pattern for search settings
* Rustdoc
* Fix license - does not appear as 'standard' on crates.io
* Unsupported

- [x] v0.1.0

* First published version (beta quality)
* Unsupported

## Alternatives

* The Genetic Algorithm is a very well known technique:
<https://en.wikipedia.org/wiki/Genetic_algorithm>
* Rust Awesome has a list: <https://github.com/awesome-rust-com/awesome-rust#genetic-algorithms>
* Cargo: <https://crates.io/search?q=genetic%20algorithm>

## Contributing

Not accepting pull requests yet :)
See roadmap.

## License

MIT permissive license. See LICENSE for full license details.

## Source Code Repository

<https://github.com/thomasbratt/watchmaker>
