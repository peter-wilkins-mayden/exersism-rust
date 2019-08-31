#[macro_use]
extern crate unic_char_range;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn is_pangram(sentence: &str) -> bool {
    let alf: HashSet<char> = HashSet::from_iter(chars!('a'..='z'));

    let t :HashSet<char> = sentence.chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| alf.contains(c))
        .collect();

    t == alf
}
