#![feature(future_join)]
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
    character::complete::{alpha0, alpha1, alphanumeric1, crlf, multispace0, multispace1},
    combinator::{cut, eof, map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    sequence::{delimited, preceded},
    *,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    future::join,
    io::{BufRead as _, BufReader, Lines},
    num::ParseIntError,
    ops::Index,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let resa: f64 = match read_lines(fname) {
        Ok(lines) => match a(lines) {
            Ok((_, res)) => res,
            Err(_) => 0.0,
        },
        Err(_) => 0.0,
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

fn a(lines: Lines<BufReader<File>>) -> IResult<String, f64> {
    let the_body = lines.fold("".to_owned(), |acc: String, l| {
        if let Ok(l) = l {
            format!("{}{}\n", acc, l)
        } else {
            acc
        }
    });

    print!("{}", the_body);
    let res = almanach(&the_body);
    println!("{:?}", res);
    match res {
        Ok((input, the_almanach)) => {
            //            println!("{:?}", the_almanach);
            Ok((input.to_owned(), 0.0))
        }
        Err(e) => Err(e.to_owned()),
    }
}
fn seeds(input: &str) -> IResult<&str, ArcArray1<f64>> {
    let (input, _) = tag("seeds:").parse(input)?;
    let (input, seeds) = many1(preceded(multispace0, take_number)).parse(input)?;
    Ok((input, ArcArray1::from_vec(seeds)))
}
fn mappe(input: &str) -> IResult<&str, Mappe> {
    let (input, (src_name, dst_name)) =
        nom::sequence::separated_pair(alpha1, tag("-to-"), alpha1).parse(input)?;
    let (input, _) = tag(" map:\n").parse(input)?;
    let (input, translations) = many1(translation).parse(input)?;
    Ok((
        input,
        Mappe {
            src_name: src_name.to_owned(),
            dst_name: dst_name.to_owned(),
            translations: ArcArray1::from_vec(translations),
        },
    ))
}
fn translation(input: &str) -> IResult<&str, Translation> {
    let (input, dst) = take_number(input)?;
    let (input, _) = multispace0(input)?;
    let (input, src) = take_number(input)?;
    let (input, _) = multispace0(input)?;
    let (input, span) = take_number(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Translation { dst, src, span }))
}
fn almanach(input: &str) -> IResult<&str, Almanach> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = tag("\n\n").parse(input)?;
    let (input, (mappes, _)) = many_till(mappe, eof).parse(input)?;
    let mut h = HashMap::<String, Mappe>::new();
    mappes.iter().for_each(|m| {
        h.insert(m.dst_name.to_owned(), m.to_owned());
    });
    Ok((input, Almanach { seeds, mappes: h }))
}

#[derive(Clone, Debug)]
struct Almanach {
    seeds: ArcArray1<f64>,
    mappes: HashMap<String, Mappe>,
}
#[derive(Clone, Debug)]
struct Mappe {
    src_name: String,
    dst_name: String,
    translations: ArcArray1<Translation>,
}
#[derive(Copy, Clone, Debug)]
struct Translation {
    dst: f64,
    src: f64,
    span: f64,
}
fn take_number(input: &str) -> IResult<&str, f64> {
    map_res(
        take_while_m_n(1, 20, |c: char| c.is_ascii_digit()),
        |numstring: &str| numstring.parse::<f64>(),
    )
    .parse(input)
}
