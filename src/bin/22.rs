use std::str::FromStr;

use anyhow::anyhow;
use num::{traits::Euclid, BigInt, One};

advent_of_code::solution!(22);

#[derive(Debug)]
struct Deck {
    cards: Vec<u16>,
}

#[derive(Debug, Clone)]
struct AdvancedDeck {
    size: BigInt,
    techniques: Vec<Technique>,
}

impl AdvancedDeck {
    fn new(size: BigInt, techniques: Vec<Technique>) -> Self {
        Self { size, techniques }
    }

    fn reverse_deal(&self, i: &BigInt) -> BigInt {
        &self.size - BigInt::one() - i
    }

    fn reverse_cut(&self, i: &BigInt, n: BigInt) -> BigInt {
        (i + n).rem_euclid(&self.size)
    }

    fn reverse_increment(&self, i: &BigInt, n: BigInt) -> BigInt {
        (n.modinv(&self.size).unwrap() * i).rem_euclid(&self.size)
    }

    fn f(&self, i: &BigInt) -> BigInt {
        let mut i = i.clone();
        for technique in &self.techniques {
            i = match *technique {
                Technique::Cut(n) => self.reverse_cut(&i, n.into()),
                Technique::Deal(n) => self.reverse_increment(&i, n.into()),
                Technique::NewStack => self.reverse_deal(&i),
            };
        }
        i
    }
}

#[derive(Debug, Clone, Copy)]
enum Technique {
    Cut(isize),
    Deal(usize),
    NewStack,
}

impl FromStr for Technique {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next().unwrap() {
            "cut" => {
                let by = words.next().unwrap().parse()?;
                Ok(Self::Cut(by))
            }
            "deal" => {
                if words.next().unwrap() == "into" {
                    Ok(Self::NewStack)
                } else {
                    let inc = words.nth(1).unwrap().parse()?;
                    Ok(Self::Deal(inc))
                }
            }
            _ => Err(anyhow!("Invalid technique {s}")),
        }
    }
}

impl Deck {
    fn new(size: usize) -> Self {
        let cards = (0..size).map(|n| n as u16).collect();
        Self { cards }
    }

    fn cut(&mut self, amount: isize) {
        let am = amount.unsigned_abs() % self.cards.len();
        if amount < 0 {
            self.cards.rotate_right(am);
        } else {
            self.cards.rotate_left(am);
        }
    }

    fn deal(&mut self, increment: usize) {
        let len = self.cards.len();
        let mut new_cards = vec![0; len];
        let mut index = 0;
        for &card in &self.cards {
            new_cards[index] = card;
            index = (index + increment) % len;
        }

        self.cards = new_cards;
    }

    fn new_stack(&mut self) {
        self.cards.reverse();
    }

    fn process<'a>(&mut self, techniques: impl IntoIterator<Item = &'a Technique>) {
        for t in techniques {
            match *t {
                Technique::Cut(by) => self.cut(by),
                Technique::Deal(inc) => self.deal(inc),
                Technique::NewStack => self.new_stack(),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut deck = Deck::new(10007);
    let techniques = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    deck.process(&techniques);
    deck.cards.iter().position(|c| *c == 2019)
}

pub fn part_two(input: &str) -> Option<BigInt> {
    let deck_size: BigInt = 119315717514047u128.into();
    let mut techniques = input
        .lines()
        .map(|l| l.parse::<Technique>().unwrap())
        .collect::<Vec<_>>();
    techniques.reverse();
    let deck = AdvancedDeck::new(deck_size.clone(), techniques);
    let x: BigInt = 2020.into();
    let y = deck.f(&x);
    let z = deck.f(&y);
    let a = ((&y - &z) * (&x - &y).modinv(&deck_size).unwrap()).rem_euclid(&deck_size);
    let b = (y - &a * &x).rem_euclid(&deck_size);
    let num_times: BigInt = 101741582076661u128.into();
    let one = BigInt::one();
    Some(
        (a.modpow(&num_times, &deck_size) * x
            + ((a.modpow(&num_times, &deck_size) - &one)
                * (a - one).modinv(&deck_size).unwrap()
                * b))
            .rem_euclid(&deck_size),
    )
}
