use rand::distributions::Distribution;
use rand::Rng;

pub static CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ";

/// Make a random string.
/// This is a convenience function used by some examples.
pub fn make_random_string(length: usize) -> String {
    let distribution = RandomStringDistribution::new(CHARACTERS);
    rand::thread_rng()
        .sample_iter(&distribution)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

pub struct RandomStringDistribution {
    bytes: Vec<u8>,
}

impl RandomStringDistribution {
    pub fn new(chars: &str) -> Self {
        Self {
            bytes: Vec::from(chars.as_bytes()),
        }
    }
}

impl Distribution<u8> for RandomStringDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let i = rng.gen_range(0..self.bytes.len());
        self.bytes[i]
    }
}
