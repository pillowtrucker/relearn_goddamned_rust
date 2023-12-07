use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_till, take_while_m_n},
        streaming::take_while1,
    },
    character::complete::{crlf, multispace0, multispace1},
    combinator::{eof, map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    sequence::{delimited, preceded},
    *,
};
use std::{
    cmp::Ordering,
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead as _, BufReader, Lines},
    num::ParseIntError,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let resa: u32 = match read_lines(fname) {
        Ok(lines) => a(lines),
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

fn a(lines: Lines<BufReader<File>>) -> u32 {
    let cards: Vec<Card> = lines
        .map(|l| -> IResult<String, Card> {
            match l {
                Ok(input) => match card(&input) {
                    Ok((input, card)) => Ok((input.to_owned(), card)),
                    Err(e) => {
                        println!("{:?}", e);
                        Err(e.to_owned())
                    }
                },
                Err(e) => Err(nom::Err::Failure(nom::error::Error {
                    input: e.to_string(),
                    code: ErrorKind::Eof,
                })),
            }
        })
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|h| h.1)
        .collect();
    cards.iter().fold(0, |acc, crd| {
        let exponent = crd.present.intersection(&crd.winning).count();
        println!("{:?}", crd);
        if (exponent > 0) {
            acc + u32::pow(2, exponent as u32 - 1)
        } else {
            acc
        }
    })
}
#[derive(Debug)]
struct Card {
    cid: u32,
    winning: HashSet<u32>,
    present: HashSet<u32>,
}
fn card(input: &str) -> IResult<&str, Card> {
    let (input, cid) = card_id(input)?;
    let (input, (winning_, _)) =
        many_till(delimited(multispace0, take_number, multispace0), tag("|")).parse(input)?;
    let winning: HashSet<u32> = HashSet::from_iter(winning_);
    let (input, present_) = many1(preceded(multispace0, take_number)).parse(input)?;
    let present: HashSet<u32> = HashSet::from_iter(present_);
    Ok((
        input,
        Card {
            cid,
            winning,
            present,
        },
    ))
}
fn card_id(input: &str) -> IResult<&str, u32> {
    let (input, cid) =
        delimited(preceded(tag("Card"), multispace1), take_number, tag(":")).parse(input)?;
    Ok((input, cid))
}

fn take_number(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 6, |c: char| c.is_ascii_digit()),
        |numstring: &str| numstring.parse::<u32>(),
    )
    .parse(input)
}
