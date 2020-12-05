use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    pub fn get_seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

fn partition_search(d: &str, lb: u32, ub: u32) -> u32 {
    let mut cur_lb = lb;
    let mut cur_ub = ub;

    for c in d[0..d.len() - 1].chars() {
        if c == 'F' || c == 'L' {
            let delta = ((cur_ub - cur_lb) as f64 / 2.0).ceil();
            cur_ub = cur_ub - delta as u32;
        } else if c == 'B' || c == 'R' {
            let delta = ((cur_ub - cur_lb) as f64 / 2.0).ceil();
            cur_lb = cur_lb + delta as u32;
        }
    }

    if d.chars().nth(d.len() - 1).unwrap() == 'L' || d.chars().nth(d.len() - 1).unwrap() == 'F' {
        cur_lb
    } else {
        cur_ub
    }
}

fn translate_seat(d: &str) -> Seat {
    Seat {
        row: partition_search(&d[0..7], 0, 127),
        col: partition_search(&d[7..], 0, 7),
    }
}

fn main() {
    let file = io::BufReader::new(File::open("input.txt").unwrap());

    println!(
        "Maximum Seat Id = {}",
        file.lines()
            .map(|s| translate_seat(s.unwrap().as_str()).get_seat_id())
            .max()
            .unwrap()
    );

    let file = io::BufReader::new(File::open("input.txt").unwrap());
    let assigned_seat_ids: HashSet<u32> = file
        .lines()
        .map(|s| translate_seat(s.unwrap().as_str()).get_seat_id())
        .collect();

    println!(
        "My Seat Id = {:?}",
        (1..)
            .filter(|id| !assigned_seat_ids.contains(&id)
                && assigned_seat_ids.contains(&(id - 1))
                && assigned_seat_ids.contains(&(id + 1)))
            .next()
            .unwrap()
    );
}
