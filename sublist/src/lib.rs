#![feature(test)]
extern crate test;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
use std::fmt::Debug;
// assumes second list is the longer one
fn is_sub<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> bool {
    assert!(second_list.len() > first_list.len());
     for i in 0..=(second_list.len() - first_list.len()){
       'inner: for j in 0..second_list.len() {
            if j == first_list.len() {
                return true;
            } else if first_list[j] == second_list[j + i] {
                continue;
            } else {
                break 'inner;
            }
        }
    }
    false
}

fn is_super<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> bool {
    is_sub(second_list, first_list)
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_sub(first_list, second_list) {
        Comparison::Sublist
    } else if first_list.len() > second_list.len() && is_super(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn sublist2<T: PartialEq + Debug>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if a.windows(n).any(|v| v == b) { Superlist } else { Unequal },
        (m, n) if m < n => if b.windows(m).any(|v| v == a) { Sublist } else { Unequal },
        (_, _) => if a == b { Equal } else { Unequal },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_superlist_early_in_huge_list_1(b: &mut Bencher) {
        b.iter(|| {
            let huge: Vec<u32> = (1..1000000).collect();
            sublist(&huge, &[3, 4, 5]);
        });
    }

    #[bench]
    fn bench_superlist_early_in_huge_list_2(b: &mut Bencher) {
        b.iter(|| {
            let huge: Vec<u32> = (1..1000000).collect();
            sublist2(&huge, &[3, 4, 5]);
        });
    }
    #[bench]
    fn bench_superlist_late_in_huge_list_1(b: &mut Bencher) {
        b.iter(|| {
            let huge: Vec<u32> = (1..1000007).collect();
            sublist(&huge, &[1000003, 1000004, 1000005]);
        });
    }

    #[bench]
    fn bench_superlist_late_in_huge_list_2(b: &mut Bencher) {
        b.iter(|| {
            let huge: Vec<u32> = (1..1000007).collect();
            sublist2(&huge, &[1000003, 1000004, 1000005]);
        });
    }
#[bench]
fn bench_superlist_huge_lists_1(b: &mut Bencher) {
    b.iter(|| {
        let huge: Vec<u32> = (1..1000020).collect();
        let huge2: Vec<u32> = (10..1000000).collect();
        sublist(&huge, &huge2);
    });
}

    #[bench]
    fn bench_superlist_huge_lists_2(b: &mut Bencher) {
        b.iter(|| {
            let huge: Vec<u32> = (1..1000020).collect();
            let huge2: Vec<u32> = (10..1000000).collect();
            sublist(&huge, &huge2);
        });
    }

    #[bench]
    fn recurring_values_unequal_1(b: &mut Bencher) {
        b.iter(|| sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1]));
    }
    #[bench]
    fn recurring_values_unequal_2(b: &mut Bencher) {
        b.iter(|| sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1]));
    }


}