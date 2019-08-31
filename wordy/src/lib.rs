use std::collections::VecDeque;
use regex::Regex;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn minus(x: i32, y: i32) -> i32 {
    x - y
}

fn mult(x: i32, y: i32) -> i32 {
    x * y
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}
use std::convert::TryInto;
fn pow(x: i32, y: i32) -> i32 {
    (x as u32).pow(y.try_into().unwrap()) as i32
}

pub fn answer(command: &str) -> Option<i32> {
    let mut ops: VecDeque<fn(i32, i32) -> i32> = VecDeque::new();
    let mut vals: VecDeque<i32> = VecDeque::new();

    let re = Regex::new(r"^What is (.*)\?$").unwrap();
    let caps = re.captures(command);

    let mut s = "";
    if caps.is_some() {
        s = caps.unwrap().get(1).unwrap().as_str();
    } else {
        return None;
    }

    dbg!(s);
    let mut expect = "val";
    for token in s.split_whitespace() {
        dbg!(token);
        match expect {
            "val" => {
                match token.parse() {
                    Ok(v) => {
                        vals.push_back(v);
                        expect = "op";
                    }
                    _ => { return None; }
                }
            }
            "op" => {
                match token {
                    "plus" => {
                        ops.push_back(add);
                        expect = "val";
                    }
                    "minus" => {
                        ops.push_back(minus);
                        expect = "val";
                    }
                    "multiplied" => {
                        ops.push_back(mult);
                        expect = "by";
                    }
                    "divided" => {
                        ops.push_back(div);
                        expect = "by";
                    }
                    "raised" => {
                        ops.push_back(pow);
                        expect = "raised";
                    }
                    _ => { return None; }
                }
            }
            "by" => {
                if token != "by" { return None; };
                expect = "val";
            }
            "raised" => { assert_eq!(token, "to"); expect = "to"; }
            "to" => { assert_eq!(token, "the"); expect = "the"; }
            "the" => {
                let v: Vec<i32> = token.chars().take(1).map(|v| v.to_string().parse().unwrap()).collect();
                vals.push_back(v[0]);
                expect = "power";
            }
            "power" => {}
            _ => unreachable!()
        }
    };
    if ops.len() != vals.len() - 1 {
        None
    } else if vals.len() == 1 {
        Some(vals.pop_front().unwrap())
    } else {
        for op in ops.iter() {
            let x = vals.pop_front().unwrap();
            let y = vals.pop_front().unwrap();
            vals.push_front(op(x, y));
        }
        Some(vals.pop_front().unwrap())
    }
}
