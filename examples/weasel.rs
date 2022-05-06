use rand::distributions::Alphanumeric;
use rand::Rng;
use watchmaker::*;

// Show how the number of cross over candidates affects the results.
// The genetic algorithm will search for the string 'weasel'.
fn main() {
    for x in (1..=8).map(|exponent| 2_usize.pow(exponent)) {
        let result = search(
            Box::new(WSGenetic::new(make_random())),
            Box::new(TournamentSelector::new(x).unwrap()),
            None,
            make_random(),
            &SearchSettings::default(),
        );
        println!("cross_over_candidates:{:3}, {:?}", x, result);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub struct WSGenome(pub String);

#[doc(hidden)]
pub static TARGET: &str = "weasel";

#[doc(hidden)]
pub struct WSGenetic {
    random: Random,
}

impl WSGenetic {
    pub fn new(random: Random) -> Self {
        Self { random }
    }
}

impl Genetic<WSGenome> for WSGenetic {
    fn initialize(&mut self) -> WSGenome {
        WSGenome(make_random_string(&mut self.random, TARGET.len()))
    }

    fn evaluate(&mut self, genome: &WSGenome) -> f64 {
        let lhs_str = TARGET;
        let rhs_str = genome.0.as_str();
        lhs_str.chars().zip(rhs_str.chars()).fold(0.0, |acc, x| {
            acc + (x.0 as i16 as f64 - x.1 as i16 as f64).abs()
        })
    }

    fn crossover(&mut self, lhs: &WSGenome, rhs: &WSGenome) -> WSGenome {
        let lhs_str = lhs.0.as_str();
        let rhs_str = rhs.0.as_str();
        let crossover_index = self.random.gen_range(0..lhs_str.len() - 1);
        let result = lhs_str
            .chars()
            .zip(rhs_str.chars())
            .enumerate()
            .map(|(i, (a, b))| if i > crossover_index { a } else { b });
        WSGenome(String::from_iter(result))
    }

    fn mutate(&mut self, original: &WSGenome) -> WSGenome {
        let chars = original.0.as_str();
        let mutate_index = self.random.gen_range(0..chars.len() as usize);
        let chars_result = chars.chars().into_iter().enumerate().map(|(i, c)| {
            if i == mutate_index {
                char::from(self.random.sample(&Alphanumeric))
            } else {
                c
            }
        });
        WSGenome(String::from_iter(chars_result))
    }
}
