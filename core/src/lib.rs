// Does not work with backticks.
// #![doc = include_str!("../README.md")]

//! See [README.md](https://github.com/thomasbratt/watchmaker/blob/main/core/README.md) for a description
mod common;
mod genetic;
mod search;
mod selector;
mod settings;

pub use common::*;
pub use genetic::*;
pub use search::*;
pub use selector::*;
pub use settings::*;
