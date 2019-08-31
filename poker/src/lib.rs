#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Hand<'a> {
    score: Poker,
    str: &'a str,
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Poker {
    HighSeven(u8, u8, u8, u8, u8),
    Pair(u8),
    TwoPairs(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: Suit,
}

use std::collections::HashMap;

fn frequencies(vs: &[u8]) -> HashMap<u8, u8> {
    vs.iter().fold(HashMap::new(), |mut acc, &value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    })
}

fn is_straight(ranks: &[u8]) -> Option<u8> {
    for i in 2..=10 {
        let t: Vec<u8> = (i..=i + 4).rev().collect();
        if ranks == &t[..] {
            return Some(i + 4);
        }
    }
    if ranks == [14, 5, 4, 3, 2] {
        return Some(1);
    }
    None
}

fn is_flush(suits: &Vec<Suit>) -> bool {
    suits.iter().all_equal()
}

fn is_straight_flush(ranks: &[u8], suits: &Vec<Suit>) -> Option<u8> {
    if is_flush(suits) {
        is_straight(ranks)
    } else {
        None
    }
}

use std::cmp::{Ordering, Reverse};
use itertools::Itertools;

impl Poker {
    fn new(cards: &[Card]) -> Poker {
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank).collect();
        ranks.sort_by_key(|&x| Reverse(x));
        let ranks = ranks;
        let suits: Vec<Suit> = cards.iter().map(|c| c.suit).collect();

        let freqs = frequencies(&ranks);
        let shape = frequencies(&freqs.values().map(|&x| x).collect::<Vec<u8>>());

        let freqs_to_vals: HashMap<u8, Vec<u8>> = freqs.iter()
            .fold(HashMap::new(), |mut acc, (&k, &v)| {
                acc.entry(v).or_insert(Vec::new()).push(k);
                acc
            });

        let get_val = |k| *freqs_to_vals.get(&k).unwrap().first().unwrap();

        let has_shape = |t1: u8, r1: u8, t2: u8, r2: u8|
            *shape.get(&t1).unwrap_or(&0) == r1 && *shape.get(&t2).unwrap_or(&0) == r2;

        if let Some(high) = is_straight_flush(&ranks, &suits)  {
            Poker::StraightFlush(high)
        } else if has_shape(4, 1, 1, 1) {
            Poker::FourOfAKind(get_val(4), get_val(1))
        } else if has_shape(3, 1, 2, 1) {
            Poker::FullHouse(get_val(3), get_val(2))
        } else if is_flush(&suits) {
            Poker::Flush(ranks[0])
        } else if let Some(high) = is_straight(&ranks) {
            Poker::Straight(high)
        } else if has_shape(3, 1, 1, 2) {
            let mut kickers = freqs_to_vals.get(&1).unwrap().to_owned();
            kickers.sort_by_key(|&x| Reverse(x));
            Poker::ThreeOfAKind(get_val(3), kickers[0], kickers[1])
        } else if has_shape(2, 2, 1, 1) {
            let mut pvs = freqs_to_vals.get(&2).unwrap().to_owned();
            pvs.sort_by_key(|&x| Reverse(x));
            Poker::TwoPairs(pvs[0], pvs[1], get_val(1))
        } else if has_shape(2, 1, 1, 3) {
            Poker::Pair(get_val(2))
        } else {
            Poker::HighSeven(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4])
        }
    }
}

impl<'a> Hand<'a> {
    fn new(str: &str) -> Hand {
        let cards: Vec<Card> = str.split_whitespace().map(|card| {
            Card::new(card)
        }).collect();
        let score = Poker::new(&cards);
        Hand { score, str }
    }
}

use regex::Regex;

impl Card {
    fn new(card: &str) -> Card {
        use Suit::*;
        lazy_static! {
        static ref RE: Regex = Regex::new(r"([\dJQKA]{1,2})([HSCD])").unwrap();
    }
        let caps = RE.captures(card).unwrap();
        let rank: &str = caps.get(1).unwrap().into();
        let suit: &str = caps.get(2).unwrap().into();
        let rank = match rank {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => rank.parse().unwrap(),
        };
        let suit = match suit {
            "H" => Heart,
            "D" => Diamond,
            "C" => Club,
            "S" => Spade,
            _ => panic!("unrecognided suit: {}", suit)
        };
        Card { rank, suit }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<_> = hands.iter().map(|c| Hand::new(c)).collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let f = hands.last().unwrap();
    Some(hands.iter().filter(|&c| c == f).map(|c| c.str).collect())
}
