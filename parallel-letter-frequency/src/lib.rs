use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input.par_iter()
        .map(|text: &&str| {
            let mut m = HashMap::new();
            text.chars()
                .flat_map(|c| c.to_lowercase())
                .filter(|c| c.is_alphabetic())
                .for_each(|c| {
                    *m.entry(c).or_insert(0) += 1;
                });
            m
        }).reduce(|| HashMap::new(), |mut res, m| {
        for (&k, &v) in m.iter() {
            *res.entry(k).or_insert(0) += v;
        }
        res
    })
}