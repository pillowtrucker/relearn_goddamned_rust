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
    character::{
        complete::{alpha0, alpha1, alphanumeric1, crlf, multispace0, multispace1, one_of},
        is_alphabetic,
    },
    combinator::{cut, eof, map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    number::complete::be_u8,
    sequence::{delimited, preceded, separated_pair, tuple, Tuple},
    *,
};
use num::{integer::lcm, Integer};
use petgraph::{csr::Csr, graphmap::DiGraphMap, visit::IntoEdgeReferences, Directed};
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
    let res = peepee(&the_body);
    println!("{:?}", res);
    match res {
        Ok((input, caca)) => {
            let mut results = Vec::<(usize, [u8; 3], [u8; 3])>::new();
            results = caca
                .starts
                .par_iter()
                .map(|start| {
                    let mut cur = *start;
                    let mut final_steps = 0;
                    while cur[2] != b'Z' {
                        for dir in caca.caca.clone().into_iter() {
                            final_steps += 1;
                            let ok = caca.poopoo.edges(cur);
                            let hng: Vec<_> = ok.collect();
                            let checkdupes = hng.len();
                            let next = if checkdupes < 2 {
                                hng.first().unwrap().clone().to_owned()
                            } else {
                                hng.iter()
                                    .find(|shitto| *shitto.2 == dir)
                                    .unwrap()
                                    .clone()
                                    .to_owned()
                            };
                            if (final_steps % 1000 == 0) {
                                println!("step {final_steps:?} start {start:?} dir {dir:?} cur {cur:?} next {next:?} edges {hng:?}");
                            }
                            cur = next.1;
                            if (cur[2] == b'Z') {
                                break;
                            }
                        }
                    }
                    (final_steps, cur.to_owned(), start.to_owned())
                })
                .collect();
            println!("results {results:?}");
            let probably_result = results.iter().fold(1, |acc, rr| {
                let frr = rr.0 as u64;
                lcm(acc, frr)
            });

            Ok((input.to_owned(), probably_result))
        }
        Err(e) => Err(e.to_owned()),
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum LR {
    L,
    R,
}
use crate::LR::*;
#[derive(Debug, Clone)]
struct Peepee {
    caca: Vec<LR>,
    poopoo: DiGraphMap<Ingot, LR>,
    starts: Vec<Ingot>,
}
#[derive(Debug, Clone)]
struct RawPoopoo {
    ingot: Ingot,
    snakes: (Ingot, Ingot),
}
fn peepee(input: &str) -> IResult<&str, Peepee> {
    let mut poopoo = DiGraphMap::<Ingot, LR>::with_capacity(512, 512);
    let (input, caca) = many1(one_of("LR")).parse(input)?;
    let (input, _) = tag("\n\n").parse(input)?;
    let (input, raw_poopoos) = many1(raw_poopoo).parse(input)?;
    for raw_poopoo in raw_poopoos.clone() {
        poopoo.add_node(raw_poopoo.ingot);
        poopoo.add_edge(raw_poopoo.ingot, raw_poopoo.snakes.0, L);
        poopoo.add_edge(raw_poopoo.ingot, raw_poopoo.snakes.1, R);
    }
    let starts = raw_poopoos
        .iter()
        .filter_map(|raw_poo| {
            if raw_poo.ingot[2] == b'A' {
                Some(raw_poo.ingot)
            } else {
                None
            }
        })
        .collect();
    Ok((
        input,
        Peepee {
            caca: caca
                .iter()
                .map(|c| match c {
                    'L' => L,
                    'R' => R,
                    _ => R,
                })
                .collect(),
            poopoo,
            starts,
        },
    ))
}
fn raw_poopoo(input: &str) -> IResult<&str, RawPoopoo> {
    let (input, the_ingot) = ingot(input)?;
    let (input, _) = tag(" = ").parse(input)?;
    let (input, snakes) =
        delimited(tag("("), separated_pair(ingot, tag(", "), ingot), tag(")")).parse(input)?;
    let (input, _) = opt(tag("\n")).parse(input)?;
    Ok((
        input,
        RawPoopoo {
            ingot: the_ingot,
            snakes,
        },
    ))
}
fn mangle(input: &str) -> [u8; 3] {
    let mut buf = [0u8; 3];
    let len = 3.min(input.len());
    buf[..len].copy_from_slice(&input.as_bytes()[..len]);
    buf
}
type Ingot = [u8; 3];
fn ingot(input: &str) -> IResult<&str, Ingot> {
    let (input, ingot) = take_while_m_n(3, 3, |c: char| c.is_ascii_uppercase()).parse(input)?;
    Ok((input, mangle(ingot)))
}
