use std::fmt;

#[derive(Debug, Eq)]
pub struct Clock (i32);


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            0: modulo(hours, 24) * 60 + minutes,
        }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            0: self.0 + minutes,
        }
    }
    fn compute(&self) -> (i32, i32) {
        if self.0 >= 0 {
            (modulo(self.0 / 60, 24), modulo(self.0, 60))
        } else if self.0 >= -59 {
            (23, modulo(self.0, 60))
        } else if modulo(self.0, 60) == 0 {
            (24 - modulo(self.0 / 60, 24), 0)
        } else {
            (modulo(self.0 / 60, 24) - 1, modulo(self.0, 60))
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, b: &Clock) -> bool {
        self.compute() == b.compute()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes) = self.compute();
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

fn modulo(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}