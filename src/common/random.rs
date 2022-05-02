use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::RngCore;

pub type Random = Box<dyn RngCore>;

pub fn make_random() -> Random {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    Box::new(Xoshiro256PlusPlus::from_entropy())
}

pub fn make_random_from_seed(seed: u64) -> Random {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    Box::new(Xoshiro256PlusPlus::seed_from_u64(seed))
}

pub fn make_random_string(random: &mut Random, length: usize) -> String {
    random
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}
