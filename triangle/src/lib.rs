#[derive(Debug)]
pub struct Triangle<T>(T,T,T);

use std::collections::HashSet;
use num::traits::Num;

impl<T> Triangle<T>
where T: Num + PartialOrd + Copy + std::hash::Hash + Eq + Ord {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides == [T::zero(), T::zero(), T::zero()] { return None; }


        Some(Triangle(s[0].clone(),s[1].clone(),s[2].clone()))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.0 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 < self.1 && self.1 < self.2
    }

    pub fn is_isosceles(&self) -> bool {
        let mut s = HashSet::new();
        s.insert(self.0.clone());
        s.insert(self.1.clone());
        s.insert(self.2.clone());
        s.len() == 2
    }
}