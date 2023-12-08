#![feature(portable_simd, isqrt)]
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
    sequence::{delimited, preceded, tuple},
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
/* autism time
T_max: race time
D: distance to beat
a = t'
d = v*t''
v = a*t''
t' + t'' = T
d > D
x: all possible values of t' such that d > D ... so...
v*t'' > D
t' + t'' = T_max
a*t''*t'' > D
t'*t''*t'' > D
t'*(T_max -t')^2 > D
NOPE
this is actually dumber than that
there is no acceleration in the AOC thingie
https://www.wolframalpha.com/input?i=d+%3D+v*t_2%2C+v+%3D+t_1%2C+t_1+%2B+t_2+%3D+T%2C+d+%3E+D%2C+D%3E0%2CT%3E0%2Ct_2%3E0 this is probably what i wanted
or this
https://www.wolframalpha.com/input?i=t_1+%2B+t_2+%3D+T%2C+t_1*t_2%3ED%3E0%2CT%3E0%2Ct_1%3E0%2Ct_2%3E0


*/

fn ways_to_win(time: f64, distance: f64) -> f64 {
    if (time == 2.0 * distance.sqrt()) {
        1.0
    } else if (time > 2.0 * distance.sqrt()) {
        let mut t_1_min = (0.5 * (time - (time.powf(2.0) - 4.0 * distance).sqrt()));
        let mut t_1_max = (0.5 * (time + (time.powf(2.0) - 4.0 * distance).sqrt()));
        if (t_1_min == t_1_min.round()) {
            t_1_min += 1.0;
        }
        if (t_1_max == t_1_max.round()) {
            t_1_max -= 1.0;
        }
        t_1_min = t_1_min.ceil();
        t_1_max = t_1_max.floor();
        println!("t1min {} t1max {}", t_1_min, t_1_max);
        t_1_max - t_1_min + 1.0
    } else {
        0.0
    }
}

/*
fn ways_to_win(time: u64, distance: u64) -> u64 {
    if (time == 2 * distance.isqrt()) {
        1
    } else if (time > 2 * distance.isqrt()) {
        let t_1_min = ((time - (time.pow(2) - 4 * distance).isqrt()) / 2);
        let t_1_max = ((time + (time.pow(2) - 4 * distance).isqrt()) / 2);
        println!("t1min {} t1max {}", t_1_min, t_1_max);
        t_1_max - t_1_min
    } else {
        0
    }
}
*/
fn a(lines: Lines<BufReader<File>>) -> IResult<String, f64> {
    let the_body = lines.fold("".to_owned(), |acc: String, l| {
        if let Ok(l) = l {
            format!("{}{}\n", acc, l)
        } else {
            acc
        }
    });
    print!("{}", the_body);
    let res = races(&the_body);
    println!("{:?}", res);
    match res {
        Ok((input, the_races)) => {
            println!("races: {:?}", the_races);
            let good_numbar = the_races
                .0
                .iter()
                .zip(the_races.1.iter())
                .map(|(t, d)| ways_to_win(*t, *d))
                .reduce(|a, b| a * b)
                .unwrap();
            // for part b
            let pb = ways_to_win(
                the_races
                    .0
                    .iter()
                    .fold("".to_owned(), |a, b| format!("{}{}", a, b))
                    .parse::<f64>()
                    .unwrap(),
                the_races
                    .1
                    .iter()
                    .fold("".to_owned(), |a, b| format!("{}{}", a, b))
                    .parse::<f64>()
                    .unwrap(),
            );
            println!("part b {}", pb);
            Ok((input.to_owned(), good_numbar))
        }
        Err(e) => Err(e.to_owned()),
    }
}

fn races(input: &str) -> IResult<&str, (Vec<f64>, Vec<f64>)> {
    let (input, _) = tag("Time:").parse(input)?;
    let (input, times) = many1(preceded(multispace0, take_number)).parse(input)?;
    let (input, _) = tag("\nDistance:").parse(input)?;
    let (input, distances) = many1(preceded(multispace0, take_number)).parse(input)?;
    Ok((input, (times, distances)))
}

fn take_number(input: &str) -> IResult<&str, f64> {
    map_res(
        take_while_m_n(1, 20, |c: char| c.is_ascii_digit()),
        |numstring: &str| numstring.parse::<f64>(),
    )
    .parse(input)
}
