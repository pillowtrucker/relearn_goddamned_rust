#![feature(portable_simd, isqrt, iter_order_by, slice_as_chunks)]
use itertools::Itertools;
use ndarray::{
    parallel::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    ArcArray, ArcArray1, ArcArray2, Array1, Axis, Ix2,
};
use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_till, take_till1, take_while_m_n},
        streaming::take_while1,
    },
    character::complete::{
        alpha0, alpha1, alphanumeric1, crlf, multispace0, multispace1, newline, one_of,
    },
    combinator::{cut, eof, map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    number::complete::be_u8,
    sequence::{delimited, preceded, terminated, tuple, Tuple},
    *,
};
use polynomial::Polynomial;
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
    let res = numberwang(&the_body);
    println!("{:?}", res);
    match res {
        Ok((input, the_series)) => {
            the_series.axis_iter(Axis(0)).into_par_iter().for_each(|t| {
                if (t.iter().all(|e| *e == 0.0)) {
                } else {
                    let xs: Vec<i64> = (0..t.len()).map(|el| (el as i64)).collect_vec();
                    let ys = t.map(|el| *el as i64);
                    let p = Polynomial::lagrange(&xs, ys.as_slice().unwrap()).unwrap();
                    println!("ys {ys:?}");
                    println!("poly {}", p.pretty("x"));
                    println!("polydata {p:?}");
                    println!("row {t:?}");
                }
            });
            //            println!("sorted: {:?}", thc);
            Ok((input.to_owned(), 0.0))
        }
        Err(e) => Err(e.to_owned()),
    }
}
fn numberwang(input: &str) -> IResult<&str, ArcArray2<f64>> {
    let (input, series): (&str, Vec<Vec<f64>>) =
        many1(terminated(many1(numberspace), newline)).parse(input)?;
    //    let (y, x) = series.as_chunks();
    let mut ok = ArcArray2::<f64>::from_elem((series.len(), series[0].len()), 0.0);
    for (i, r) in series.iter().enumerate() {
        for (j, n) in r.iter().enumerate() {
            ok[[i, j]] = *n;
        }
    }
    Ok((input, ok))
}
fn numberspace(input: &str) -> IResult<&str, f64> {
    let (input, numbar): (&str, f64) = preceded(opt(tag(" ")), take_number).parse(input)?;
    Ok((input, numbar))
}
fn take_number(input: &str) -> IResult<&str, f64> {
    map_res(
        take_while_m_n(1, 20, |c: char| {
            c.is_ascii_digit() || c.eq_ignore_ascii_case(&'-')
        }),
        |numstring: &str| numstring.parse::<f64>(),
    )
    .parse(input)
}
