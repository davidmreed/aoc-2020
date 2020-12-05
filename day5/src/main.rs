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

    for c in d.chars().take(d.len() - 1) {
        let delta = ((cur_ub - cur_lb) as f64 / 2.0).ceil() as u32;
        if c == 'F' || c == 'L' {
            cur_ub -= delta;
        } else if c == 'B' || c == 'R' {
            cur_lb += delta;
        }
    }

    let final_char = d.chars().nth(d.len() - 1).unwrap();
    if final_char == 'L' || final_char == 'F' {
        cur_lb
    } else {
        cur_ub
    }
}

fn translate_seat(d: &String) -> Seat {
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
            .map(|s| translate_seat(&s.unwrap()).get_seat_id())
            .max()
            .unwrap()
    );

    let file = io::BufReader::new(File::open("input.txt").unwrap());
    let assigned_seat_ids: HashSet<u32> = file
        .lines()
        .map(|s| translate_seat(&s.unwrap()).get_seat_id())
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
