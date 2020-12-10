use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("input.txt")?);
    const WINDOW_SIZE: usize = 25;

    let numbers = file
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<u64>>();

    let (_, first_invalid) = numbers
        .iter()
        .skip(WINDOW_SIZE)
        .enumerate()
        .filter(|(i, d)| {
            (&numbers[*i..*i + WINDOW_SIZE])
                .iter()
                .tuple_combinations()
                .map(|(a, b)| a + b)
                .position(|q| q == **d)
                .is_none()
        })
        .next()
        .ok_or("No invalid number found")?;

    println!("First invalid number is {}", first_invalid);

    for start_pos in 0..numbers.len() {
        for length in 2..numbers.len() - start_pos {
            let contiguous_range = &numbers[start_pos..start_pos + length];
            if contiguous_range.iter().sum::<u64>() == *first_invalid {
                println!(
                    "Encryption weakness is {}",
                    contiguous_range.iter().min().unwrap() + contiguous_range.iter().max().unwrap()
                );
                return Ok(());
            }
        }
    }

    Ok(())
}
