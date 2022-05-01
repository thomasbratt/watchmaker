# watchmaker

A genetic algorithm implementation in Rust.

[![CircleCI](https://circleci.com/gh/thomasbratt/watchmaker/tree/main.svg?style=svg)](https://circleci.com/gh/thomasbratt/watchmaker/tree/main)

## Features

* Developed as an investigation into capabilities and implementation characteristics
* Written in the Rust programming language

## Usage

* Install Rust using rustup <https://rustup.rs/>
* Add a reference to your Cargo.toml file from: <https://crates.io/crates/watchmaker>
* Implement the `Genetic` interface for your search problem:
```rust
    pub trait Genetic<G> {
        fn initialize(&mut self) -> G;
        fn evaluate(&mut self, genome: &G) -> f64;
        fn crossover(&mut self, lhs: &G, rhs: &G) -> G;
        fn mutate(&mut self, original: &G) -> G;
        fn random(&mut self) -> &mut Random;
    }
```
* Call the solver method:
```rust
    let result = solve(
        Box::new(MySearchProblem::new(make_random())),
        None,
        0.00,
        32,
        100,
        0.0,
        1_000,
        Duration::from_secs(5),
    );
    println!("{:?}", result);
```

## Development

* Clone the repository (see below)
* Run `cargo test` or `cargo build`

## Examples

See the examples folder.

## Roadmap

* Rustdoc
* Multithreading

## Alternatives

Genetic Algorithm is a very well known technique:
<https://en.wikipedia.org/wiki/Genetic_algorithm>
There are many alternatives for Rust available through cargo: <https://crates.io/search?q=genetic%20algorithm>

## License

MIT permissive license. See LICENSE for full license details.

## Source Code Repository

<https://github.com/thomasbratt/watchmaker>
