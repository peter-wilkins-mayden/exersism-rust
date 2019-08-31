#[macro_use]
extern crate maplit;

use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

fn merge_with<K: Hash + Eq, V>(maps: &[HashMap<K, V>]) -> HashMap<K, V> {
    let n = maps.len();
    match n {
        0 => HashMap::new(),
        1 => maps[0].clone(),
        _ => {
            let mut  acc= maps[0].clone();
            maps[1..].iter().fold(acc, |m1, m2| {
                for(k, v) in m2.iter() {
                    *acc.entry(k).or_insert(v) += v;
                };
                acc
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_with;

    #[test]
    fn it_works() {
        let m1 = hashmap! {"a" => 1,"b" => 2};
        let m2 = hashmap! {"a" => 1,"b" => 2};

        assert_eq!(hashmap! {"a" => 2,"b" => 4}, merge_with(&vec![m1, m2]))
    }
}
