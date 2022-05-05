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
* Implement the `Genetic` trait for your search problem:
```rust
    pub trait Genetic<G> {
        fn initialize(&mut self) -> G;
        fn evaluate(&mut self, genome: &G) -> f64;
        fn crossover(&mut self, lhs: &G, rhs: &G) -> G;
        fn mutate(&mut self, original: &G) -> G;
    }
```
Example:
```rust

type MyGenome = String;

pub struct MySearchProblem {
    random: Random,
}

impl MySearchProblem {
    pub fn new(random: Random) -> Self {
        Self { random }
    }
}

impl Genetic<MyGenome> for MySearchProblem {
    fn initialize(&mut self) -> MyGenome {
        unimplemented!();
    }

    fn evaluate(&mut self, genome: &MyGenome) -> f64 {
        unimplemented!();
    }

    fn crossover(&mut self, lhs: &MyGenome, rhs: &MyGenome) -> MyGenome {
        unimplemented!();
    }

    fn mutate(&mut self, original: &MyGenome) -> MyGenome {
        unimplemented!();
    }
}
```
See `WSGenome` and `WSGenetic` for a fully implemented, working example that finds a target string.
* Call the `solver` method:
```rust
    pub fn search<G>(
        mut genetic: Box<dyn Genetic<G>>,
        mut progress: Option<Progress<G>>,
        mut random: Random,
        settings: &Settings,
    ) -> Result<Success<G>, Failure>
```
Example:
```rust
    let result = solve(
        mut genetic: Box::new(MySearchProblem::new(make_random())),
        mut progress: None,
        make_random(),
        &Settings::default(),
    );
    println!("{:?}", result);
```

## Development

* Clone the repository (see below)
* Run `cargo test` or `cargo build`

## Examples

See the `examples` folder.

## Roadmap

Note major version increment with each major release.
API changes will not be backwards compatible between major releases.

- [ ] v3.x.x

* Fourth published version
* Long Term Support
* Will take contributions, bug fixes from this point on.

- [ ] v2.x.x

* Third published version (beta quality)
* Multithreading
* Split out algorithm that produces new generations
* Better typing around Progress
* Simpler code example in README.md and crate documentation
* Link to examples, with description and sample output
* Update features section
* Unsupported

- [ ] v1.x.x

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
