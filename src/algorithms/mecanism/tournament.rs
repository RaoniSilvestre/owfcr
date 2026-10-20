use rand::{rngs::StdRng, seq::IndexedRandom};

use crate::types::Solution;

pub fn tournament<'a>(
    population: &'a [Solution],
    rng: &mut StdRng,
    k: usize,
) -> (&'a Solution, &'a Solution) {
    let p1 = selection(population, rng, k);
    let mut p2 = selection(population, rng, k);

    while std::ptr::eq(p1, p2) || p1.mask == p2.mask {
        p2 = selection(population, rng, k);
    }

    (p1, p2)
}

fn selection<'a>(population: &'a [Solution], rng: &mut StdRng, k: usize) -> &'a Solution {
    (0..k)
        .map(|_| population.choose(rng).unwrap())
        .min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap())
        .unwrap()
}
