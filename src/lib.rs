// Does not work with backticks.
// #![doc = include_str!("../README.md")]

mod common;
mod genetic;
mod search;
mod settings;

pub use common::*;
pub use genetic::*;
pub use search::*;
pub use settings::*;
