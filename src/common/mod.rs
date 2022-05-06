mod failure;
mod math;
mod random;

pub use failure::*;
pub(crate) use math::largest;
pub(crate) use math::mean;
pub use math::round;
pub use random::*;
