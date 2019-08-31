use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    env: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
enum EnvItem {
    Val(i32),
    Symbol(String),
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new(), env: HashMap::new() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        if input.starts_with(":") && !input.ends_with(";") {
            return Err(Error::InvalidWord);
        } else if input.starts_with(":") {

            let tokens = input.split_whitespace().collect::<Vec<&str>>();
            let val = self.eval(tokens[2..tokens.len()-1].join(" ").as_str());
            dbg!(val);
            match val {
               Ok(val)  self.env.insert(tokens[1].to_lowercase(), val)
            }

        } else {
            for t in input.split_whitespace() {
                match t.to_lowercase().as_str() {
                    "+" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else {
                            let z = self.stack.split_off(self.stack.len() - 2).iter().fold(0, |acc, &x| acc + x);
                            self.stack.push(z);
                        }
                    }
                    "-" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else {
                            let vals = self.stack.split_off(self.stack.len() - 2);
                            self.stack.push(vals[0] - vals[1]);
                        }
                    }
                    "*" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else {
                            let vals = self.stack.split_off(self.stack.len() - 2);
                            self.stack.push(vals[0] * vals[1]);
                        }
                    }
                    "/" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else if self.stack[self.stack.len() - 1] == 0 {
                            return Err(Error::DivisionByZero);
                        } else {
                            let vals = self.stack.split_off(self.stack.len() - 2);
                            self.stack.push(vals[0] / vals[1]);
                        }
                    }
                    "dup" => {
                        if self.stack.len() == 0 {
                            return Err(Error::StackUnderflow);
                        } else {
                            self.stack.push(self.stack[self.stack.len() - 1]);
                        }
                    }
                    "drop" => {
                        if self.stack.len() == 0 {
                            return Err(Error::StackUnderflow);
                        } else {
                            self.stack.pop();
                        }
                    }
                    "swap" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else {
                            let vals = self.stack.split_off(self.stack.len() - 2);
                            self.stack.push(vals[1]);
                            self.stack.push(vals[0]);
                        }
                    }
                    "over" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        } else {
                            self.stack.push(self.stack[self.stack.len() - 2]);
                        }
                    }
                    _ => self.stack.push(t.parse().unwrap())
                }
            }
        }
        Ok(())
    }
}
