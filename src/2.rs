use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::streaming::take_while_m_n,
    combinator::{eof, map_res, opt},
    error::ErrorKind,
    multi::{many_m_n, many_till},
    sequence::delimited,
    *,
};
use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead as _, BufReader, Lines},
    num::ParseIntError,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    println!(
        "{:?} <= {:?} == {:?}",
        Ballsack::Blue(20),
        Ballsack::Blue(10),
        Ballsack::Blue(20) <= Ballsack::Blue(10)
    );
    if let [Ok(max_r), Ok(max_g), Ok(max_b)] = &args[2..5]
        .iter()
        .map(|a| a.parse::<u32>())
        .collect::<Vec<Result<u32, ParseIntError>>>()[..]
    {
        let max_bag = Bag {
            ballsack_red: Ballsack::Red(*max_r),
            ballsack_green: Ballsack::Green(*max_g),
            ballsack_blue: Ballsack::Blue(*max_b),
        };
        let resa: u32 = match read_lines(fname) {
            Ok(lines) => a(lines, max_bag),
            Err(_) => 0,
        };

        let resb: u32 = match read_lines(fname) {
            Ok(lines) => b(lines),
            Err(_) => 0,
        };

        println!("part 1: {}, part 2: {}", resa, resb);
    }
}
fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Bag {
    ballsack_red: Ballsack,
    ballsack_green: Ballsack,
    ballsack_blue: Ballsack,
}
impl Ord for Bag {
    fn cmp(&self, other: &Self) -> Ordering {
        let r = self.ballsack_red.cmp(&other.ballsack_red);
        let g = self.ballsack_green.cmp(&other.ballsack_green);
        let b = self.ballsack_blue.cmp(&other.ballsack_blue);
        match (r, g, b) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Greater, _, _) => Ordering::Greater,
            (_, Ordering::Greater, _) => Ordering::Greater,
            (_, _, Ordering::Greater) => Ordering::Greater,
            (Ordering::Less, Ordering::Less, Ordering::Less) => Ordering::Less,
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn b(lines: Lines<BufReader<File>>) -> u32 {
    let games: Vec<Game> = lines
        .map(|l| -> IResult<String, Game> {
            match l {
                Ok(input) => match game(&input) {
                    Ok((input, game)) => Ok((input.to_owned(), game)),
                    Err(e) => Err(e.to_owned()),
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
    games.iter().fold(0, |acc, g| {
        println!("{:?}", g);
        let mut max_red = Ballsack::Red(0);
        let mut max_green = Ballsack::Green(0);
        let mut max_blue = Ballsack::Blue(0);
        g.bags.iter().for_each(|b| {
            if b.ballsack_green > max_green {
                max_green = b.ballsack_green;
            }
            if b.ballsack_blue > max_blue {
                max_blue = b.ballsack_blue;
            }
            if b.ballsack_red > max_red {
                max_red = b.ballsack_red;
            }
        });
        match (max_red, max_green, max_blue) {
            (Ballsack::Red(r), Ballsack::Green(g), Ballsack::Blue(b)) => acc + r * g * b,
            _ => 0,
        }
    })
}

fn a(lines: Lines<BufReader<File>>, max_bag: Bag) -> u32 {
    println!("max bag {:?}", max_bag);
    let games: Vec<Game> = lines
        .map(|l| -> IResult<String, Game> {
            match l {
                Ok(input) => match game(&input) {
                    Ok((input, game)) => Ok((input.to_owned(), game)),
                    Err(e) => Err(e.to_owned()),
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
    games.iter().fold(0, |acc, g| {
        println!("{:?}", g);

        if g.bags.iter().all(|b| {
            println!("{:?} <= {:?} == {:?}", *b, max_bag, (*b <= max_bag));
            *b <= max_bag
        }) {
            acc + g.gid
        } else {
            acc
        }
    })
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, gid) = game_id(&input)?;
    let (input, bags) = bags(&input)?;
    Ok((input, Game { gid, bags }))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    gid: u32,
    bags: Vec<Bag>,
}

fn bags(input: &str) -> IResult<&str, Vec<Bag>> {
    let (input, (bags, _)) = many_till(bag, eof).parse(input)?;
    Ok((input, bags))
}

fn game_id(input: &str) -> IResult<&str, u32> {
    let (input, gid) = delimited(tag("Game "), take_number, tag(":")).parse(input)?;
    Ok((input, gid))
}
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Ballsack {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn balls(input: &str) -> IResult<&str, Ballsack> {
    let (input, num_balls) = delimited(tag(" "), take_number, tag(" ")).parse(input)?;
    let (input, colour) = take_colour(input)?;
    let (input, _) = opt(tag(",")).parse(input)?;
    if let Some(ballsack) = match colour {
        "red" => Some(Ballsack::Red(num_balls)),
        "green" => Some(Ballsack::Green(num_balls)),
        "blue" => Some(Ballsack::Blue(num_balls)),
        &_ => None,
    } {
        Ok((input, ballsack))
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input,
            code: ErrorKind::Eof,
        }))
    }
}

fn bag(input: &str) -> IResult<&str, Bag> {
    let (input, ballsacks) = many_m_n(1, 3, balls).parse(input)?;
    let (input, _) = opt(tag(";")).parse(input)?;
    let mut ballsack_red = Ballsack::Red(0);
    let mut ballsack_green = Ballsack::Green(0);
    let mut ballsack_blue = Ballsack::Blue(0);
    ballsacks.iter().for_each(|bs| match bs {
        Ballsack::Blue(_) => ballsack_blue = bs.to_owned(),
        Ballsack::Red(_) => ballsack_red = bs.to_owned(),
        Ballsack::Green(_) => ballsack_green = bs.to_owned(),
    });
    Ok((
        input,
        Bag {
            ballsack_red,
            ballsack_green,
            ballsack_blue,
        },
    ))
}

fn take_number(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 6, |c: char| c.is_ascii_digit()),
        |numstring| u32::from_str_radix(numstring, 10),
    )
    .parse(input)
}
fn take_colour(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue"))).parse(input)
}
