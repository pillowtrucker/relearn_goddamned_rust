#![feature(portable_simd, isqrt, iter_order_by)]
use itertools::Itertools;
use ndarray::{
    parallel::prelude::{IntoParallelRefIterator, ParallelIterator},
    ArcArray1, Array1,
};
use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_till, take_while_m_n},
        streaming::take_while1,
    },
    character::complete::{alpha0, alpha1, alphanumeric1, crlf, multispace0, multispace1, one_of},
    combinator::{cut, eof, map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    number::complete::be_u8,
    sequence::{delimited, preceded, tuple, Tuple},
    *,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead as _, BufReader, Lines},
    num::ParseIntError,
    ops::Index,
    path::Path,
    simd::StdFloat,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let resa: u64 = match read_lines(fname) {
        Ok(lines) => match a(lines) {
            Ok((_, res)) => res,
            Err(_) => 0,
        },
        Err(_) => 0,
    };
    /*
        let resb: u32 = match read_lines(fname) {
            Ok(lines) => b(lines),
            Err(_) => 0,
        };
    */
    println!("part 1: {}, part 2: {}", resa, 0) //, resb);
}
fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn a(lines: Lines<BufReader<File>>) -> IResult<String, u64> {
    let the_body = lines.fold("".to_owned(), |acc: String, l| {
        if let Ok(l) = l {
            format!("{}{}\n", acc, l)
        } else {
            acc
        }
    });
    print!("{}", the_body);
    let res = cards(&the_body);
    println!("{:?}", res);
    match res {
        Ok((input, the_cards)) => {
            let mut thc = the_cards.clone();
            thc.sort();
            println!("sorted: {:?}", thc);
            Ok((
                input.to_owned(),
                thc.iter()
                    .fold((the_cards.len() as u64, 0), |acc, h| {
                        println!("{:?} rank {} score before {}", h, acc.0, acc.1);
                        (acc.0 - 1, acc.1 + h.bid * acc.0)
                    })
                    .1,
            ))
        }
        Err(e) => Err(e.to_owned()),
    }
}
fn cards(input: &str) -> IResult<&str, Vec<Cards>> {
    let (input, (hands, _)) = many_till(cardes, eof).parse(input)?;
    Ok((input, hands))
}
use crate::Hand::*;
fn cards_to_hand(mut cards: Vec<u8>) -> Hand {
    cards.sort();
    let ddwc = cards.iter().dedup_with_count();
    let mut jokers = 0;
    let mut fuuuck: Vec<(usize, &u8)> = ddwc
        .clone()
        .filter_map(|(cn, c)| match c {
            1 => {
                if (cn == 5) {
                    Some((cn, c))
                } else {
                    jokers = cn;
                    None
                }
            }
            _ => Some((cn, c)),
        })
        .collect();
    println!("fuuuck ? {:?}", fuuuck);
    let max_poz = fuuuck
        .iter()
        .position_max_by(|x, y| {
            if (*x.1 == 1) {
                Ordering::Less
            } else if *y.1 == 1 {
                Ordering::Greater
            } else if x.0.cmp(&y.0) == Ordering::Equal {
                x.1.cmp(y.1)
            } else {
                x.0.cmp(&y.0)
            }
        })
        .unwrap();
    let (cn_, c_) = fuuuck[max_poz];
    fuuuck[max_poz] = (cn_ + jokers, c_);
    fuuuck
        .iter()
        .map(|(cn, _)| match cn {
            1 => High,
            2 => Pair,
            3 => Three,
            4 => Chariot,
            5 => Five,
            _ => High,
        })
        .reduce(|h1, h2| match (h1, h2) {
            (High, o) => o,
            (o, High) => o,
            (Five, _) => Five,
            (_, Five) => Five,
            (Chariot, _) => Chariot,
            (_, Chariot) => Chariot,
            (Pair, Three) => FullHouse,
            (Three, Pair) => FullHouse,
            (FullHouse, _) => FullHouse,
            (_, FullHouse) => FullHouse,
            (Three, _) => Three,
            (_, Three) => Three,
            (Pair, Pair) => TwoPair,
            (TwoPair, _) => TwoPair,
            (_, TwoPair) => TwoPair,
        })
        .unwrap()
}
fn cardes(input: &str) -> IResult<&str, Cards> {
    let f: HashMap<char, u8> = [('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)].into();
    let (input, cards) = many1(one_of("23456789TJQKA")).parse(input)?;
    let (input, bid) = preceded(multispace0, take_number).parse(input)?;
    let (input, _) = opt(tag("\n")).parse(input)?;
    let cards = cards
        .iter()
        .map(|c| {
            if f.contains_key(c) {
                f[c]
            } else {
                c.to_digit(10).unwrap() as u8
            }
        })
        .collect::<Vec<u8>>();
    let hand = cards_to_hand(cards.clone());
    Ok((input, Cards { bid, hand, cards }))
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cards {
    cards: Vec<u8>,
    hand: Hand,
    bid: u64,
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum Hand {
    Five,
    Chariot,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High,
}
impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand = self.hand.cmp(&other.hand);
        if (hand == Ordering::Equal) {
            self.cards.iter().cmp_by(&other.cards, |&me, other| {
                println!("comparing {} with {}", me, other);
                if (me < *other) {
                    Ordering::Greater // something is fucked with this but it has to be like that, maybe I'm comparing in the wrong order or my ranks are flipped
                } else if me > *other {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
        } else {
            hand
        }
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn take_number(input: &str) -> IResult<&str, u64> {
    map_res(
        take_while_m_n(1, 20, |c: char| c.is_ascii_digit()),
        |numstring: &str| numstring.parse::<u64>(),
    )
    .parse(input)
}
