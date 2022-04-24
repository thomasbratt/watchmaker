use rand::distributions::Alphanumeric;
use rand::{Rng, RngCore};

pub fn make_random() -> Box<dyn RngCore> {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    Box::new(Xoshiro256PlusPlus::seed_from_u64(82938))
}

pub fn make_random_string(random: &mut Box<dyn RngCore>, length: usize) -> String {
    random
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
        .to_string()
}
