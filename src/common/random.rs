use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::RngCore;

/// Syntax sugar for a source of randomness chosen at runtime.
pub type Random = Box<dyn RngCore>;

/// Create a source of randomness for use in the genetic algorithm.
/// This is provided as a convenience and different sources of randomness can be used, as long as they conform to [`Random`].
pub fn make_random() -> Random {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    Box::new(Xoshiro256PlusPlus::from_entropy())
}

/// Create a source of randomness with a seed for use in the genetic algorithm.
/// This is provided as a convenience and different sources of randomness can be used, as long as
/// they conform to [`Random`].
pub fn make_random_from_seed(seed: u64) -> Random {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    Box::new(Xoshiro256PlusPlus::seed_from_u64(seed))
}

/// Make a random string.
/// This is a convenience function used by some examples.
pub fn make_random_string(random: &mut Random, length: usize) -> String {
    random
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}
