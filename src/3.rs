use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead as _, BufReader, Lines},
    path::Path,
    vec,
};

use ndarray::{
    parallel::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    s, ArcArray2, Array2, Axis, Ix2, Shape, Slice, SliceArg,
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

    println!("part 1: {}, part 2: {}", resa, 0); //, resb);
}
fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Booger {
    num: u32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

fn a(lines: Lines<BufReader<File>>) -> u32 {
    let mut dim_y = 256;
    let mut dim_x = 256;
    let mut grid = ArcArray2::from_elem((dim_y + 1, dim_x + 1), '.');
    let mut y = 0;
    lines.for_each(|l| {
        if let Ok(ll) = l {
            let le = ll.len();
            if le > dim_x {
                dim_x = le
            };
            y += 1;
            if (y > dim_y) {
                dim_y = y
            };
            grid.reshape((dim_y + 1, dim_x + 1));
            ll.char_indices().for_each(|(i, c)| {
                grid[[y - 1, i]] = c;
            });
        }
    });
    println!("{:?}", grid);
    let mut h = HashSet::<Booger>::new();
    grid.indexed_iter().for_each(|((yy, xx), elem)| {
        if elem.is_ascii_digit() {
            let mut x_start = xx;
            let mut x_end = xx;
            let y = yy;
            //            println!("found digit {} at x: {} y: {}", elem, xx, yy);
            loop {
                if !(grid[[y, x_start]].is_ascii_digit()) {
                    x_start += 1;
                    break;
                }
                if (x_start > 0) {
                    x_start -= 1;
                } else {
                    break;
                }
            }

            while (x_end <= dim_x && grid[[y, x_end]].is_ascii_digit()) {
                x_end += 1;
            }
            if (x_end > dim_x) {
                x_end -= 1;
            }
            /*            let sl = Slice {
                start: x_start as isize,
                end: Some(x_end as isize),
                step: 1,
            };*/
            //let mut border_start_x = 0;
            /*            if (x_start == 1) {
                border_start_x = 2
            } else {
                border_start_x = x_start
            };*/
            let sl_border_h = Slice {
                start: max(x_start as isize - 1, 0),
                end: Some(min(x_end as isize + 1, dim_x as isize)),
                step: 1,
            };
            let sl_border_v = Slice {
                start: max(y as isize - 1, 0),
                end: Some(min(y as isize + 2, dim_y as isize)),
                step: 1,
            };

            let num_s: String = grid
                /*                .slice_axis(Axis(1), sl)
                .slice_axis(
                    Axis(0),
                    Slice {
                        start: y as isize,
                        end: Some(y as isize + 1),
                        step: 1,
                    },
                )
                */
                .index_axis(Axis(0), y)
                .slice(s![x_start..x_end])
                .iter()
                .collect();
            let num_border_h = grid.slice_axis(Axis(1), sl_border_h);
            let num_border = num_border_h.slice_axis(Axis(0), sl_border_v);
            let bogered = num_border
                .into_par_iter()
                .all(|bog| *bog == '.' || bog.is_ascii_digit());

            println!(
                "x_start: {:?},x_end: {:?}, num_s: {}, num_border: {}",
                x_start, x_end, num_s, num_border
            );
            if (!bogered) {
                if let Ok(num) = num_s.parse::<u32>() {
                    let b = Booger {
                        x_start,
                        x_end,
                        num,
                        y,
                    };
                    HashSet::insert(&mut h, b);
                    println!("{:?}", h);
                }
            }
        }
    });
    h.iter().fold(0, |acc, unique_boger| acc + unique_boger.num)
}
