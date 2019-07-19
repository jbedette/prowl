use rand;
use rand::Rng;

pub fn random_range(low: usize, hi: usize) -> usize {
    rand::thread_rng().gen_range(low, hi)
}
