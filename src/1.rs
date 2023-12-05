use std::{
    env,
    fs::File,
    io::{BufRead as _, BufReader, Lines},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let resa: i32 = match read_lines(fname) {
        Ok(lines) => a(lines),
        Err(_) => 0,
    };
    let resb: i32 = match read_lines(fname) {
        Ok(lines) => b(lines),
        Err(_) => 0,
    };

    println!("part 1: {}, part 2: {}", resa, resb);
}
fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn b(lines: Lines<BufReader<File>>) -> i32 {
    lines.fold(0, |acc, l| -> i32 {
        let mut first = None;
        let mut last = None;
        let mut start = 0;
        let mut end: usize;
        if let Ok(ll) = l {
            for i in ll.char_indices() {
                end = i.0;
                if i.1.is_ascii_digit() {
                    start = i.0 + 1;
                    if first.is_none() {
                        first = i.1.to_digit(10);
                    };
                    last = i.1.to_digit(10);
                } else {
                    let mut save_start = start;
                    for ii in start..end {
                        let maybe_d = &ll[ii..end + 1];
                        println!("{}", maybe_d);
                        if let Some(digit) = DIGITS.iter().position(|&word| word == maybe_d) {
                            if first.is_none() {
                                first = Some(digit as u32);
                            }
                            last = Some(digit as u32);
                            save_start = ii;
                        }
                    }
                    start = save_start;
                }
            }
        }
        match (first, last) {
            (Some(fi), Some(la)) => match format!("{}{}", fi, la).parse::<i32>() {
                Ok(f) => acc + f,
                _ => acc,
            },
            _ => acc,
        }
    })
}

fn a(lines: Lines<BufReader<File>>) -> i32 {
    lines.fold(0, |acc, l| -> i32 {
        let mut first = '\0';
        let mut last = '\0';
        if let Ok(ll) = l {
            for c in ll.chars() {
                if c.is_ascii_digit() {
                    if first == '\0' {
                        first = c;
                    }
                    last = c;
                }
            }
        }
        match format!("{}{}", first, last).parse::<i32>() {
            Ok(f) => acc + f,
            _ => acc,
        }
    })
}
