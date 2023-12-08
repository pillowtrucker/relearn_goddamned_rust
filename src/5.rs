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
    sequence::{delimited, preceded, tuple},
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
        Ok(lines) => match b(lines) {
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

fn translate(id: f64, origin: &str, destination: &str, almanach: &RealAlmanach) -> f64 {
    let mut inter_dest = &almanach.mappes[destination];
    //let mut ballsack = vec![];
    let mut ballsack = vec![destination.to_owned()];
    while (inter_dest.src_name != origin) {
        //        println!("pushing {}", inter_dest.src_name);
        ballsack.push(inter_dest.src_name.to_owned());
        inter_dest = &almanach.mappes[&inter_dest.src_name];
    }
    let mut nid = id;
    //    println!("starting seed {}", nid);

    ballsack.pop();
    //ballsack.push(destination.to_owned());
    //    println!("my ballsack {:?}", ballsack);
    loop {
        if let Some(to_visit) = ballsack.pop() {
            //            println!(
            //                "seed {} looking first in {} to {} map",
            //                nid, inter_dest.src_name, inter_dest.dst_name
            //            );
            if let Some(good_trans) = inter_dest
                .translations
                .iter()
                .filter(|t| t.src <= nid && nid <= t.src + t.span)
                .collect::<Vec<_>>()
                .first()
            {
                let offset = nid - good_trans.src;
                nid = good_trans.dst + offset;

                inter_dest = &almanach.mappes[&to_visit];
            //                println!(
            //                    "new id {} in {} to {} map",
            //                    nid, inter_dest.src_name, inter_dest.dst_name
            //                );
            } else {
                inter_dest = &almanach.mappes[&to_visit];
            }
        } else {
            if let Some(good_trans) = inter_dest
                .translations
                .iter()
                .filter(|t| t.src <= nid && nid <= t.src + t.span)
                .collect::<Vec<_>>()
                .first()
            {
                let offset = nid - good_trans.src;
                nid = good_trans.dst + offset;
            }
            break;
        }
    }
    //    println!("last inter_dest for {}: {:?}", nid, inter_dest);

    nid
}
/*
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
            let locations: Vec<f64> = the_almanach
                .seeds
                //                .par_iter()
                .iter()
                .map(|seed| translate(*seed, "seed", "location", &the_almanach))
                .collect();
            println!("locations: {:?}", locations);
            Ok((
                input.to_owned(),
                locations
                    .iter()
                    .min_by(|arg0: &&f64, other: &&f64| f64::total_cmp(*arg0, *other)) // so safe and elegant
                    .unwrap()
                    .to_owned(),
            ))
        }
        Err(e) => Err(e.to_owned()),
    }
}
*/
fn b(lines: Lines<BufReader<File>>) -> IResult<String, f64> {
    let the_body = lines.fold("".to_owned(), |acc: String, l| {
        if let Ok(l) = l {
            format!("{}{}\n", acc, l)
        } else {
            acc
        }
    });

    //    print!("{}", the_body);
    let res = real_almanach(&the_body);
    //    println!("{:?}", res);
    match res {
        Ok((input, the_almanach)) => {
            let locations: Vec<f64> = the_almanach
                .seed_ranges
                .par_iter()
                //.iter()
                .map(|seed_range| {
                    println!("seed_range {:?}", seed_range);
                    let locs: Vec<u64> = (seed_range.0 as u64
                        ..seed_range.0 as u64 + seed_range.1 as u64 + 1)
                        .collect();
                    //                    println!("stupid locs {:?}", locs);
                    locs.iter()
                        .map(|seed| translate(*seed as f64, "seed", "location", &the_almanach))
                        .min_by(|arg0: &f64, other: &f64| f64::total_cmp(arg0, other))
                        .unwrap()
                        .to_owned()
                })
                .collect();
            println!("locations: {:?}", locations);
            Ok((
                input.to_owned(),
                locations
                    .iter()
                    .min_by(|arg0: &&f64, other: &&f64| f64::total_cmp(*arg0, *other)) // so safe and elegant
                    .unwrap()
                    .to_owned(), // this might be off by one too
            ))
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

fn real_almanach(input: &str) -> IResult<&str, RealAlmanach> {
    let (input, real_seeds) = real_seeds(input)?;
    let (input, _) = tag("\n\n").parse(input)?;
    let (input, (mappes, _)) = many_till(mappe, eof).parse(input)?;
    let mut h = HashMap::<String, Mappe>::new();
    mappes.iter().for_each(|m| {
        h.insert(m.dst_name.to_owned(), m.to_owned());
    });
    Ok((
        input,
        RealAlmanach {
            seed_ranges: ArcArray1::from_vec(real_seeds),
            mappes: h,
        },
    ))
}
fn real_seeds(input: &str) -> IResult<&str, Vec<(f64, f64)>> {
    let (input, _) = tag("seeds:").parse(input)?;
    let (input, seed_pairs) = many1(tuple((
        preceded(multispace1, take_number),
        preceded(multispace1, take_number),
    )))
    .parse(input)?;
    Ok((input, seed_pairs))
}

#[derive(Clone, Debug)]
struct RealAlmanach {
    seed_ranges: ArcArray1<(f64, f64)>,
    mappes: HashMap<String, Mappe>,
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
