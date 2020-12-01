use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn from_file(file_name: &str) -> Vec<u32> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    content
        .trim()
        .split('\n')
        .map(|x| {
            x.trim()
                .parse::<u32>()
                .expect(format!("{} is not a valid number", x).as_str())
        })
        .collect()
}

fn main() {
    let input: Vec<u32> = from_file("input.txt");

    let result = input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == 2020)
        .next()
        .unwrap();

    println!("Output (Part 1) is {}", result.0 * result.1);

    let result = input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b, &c)| a + b + c == 2020)
        .next()
        .unwrap();

    println!("Output (Part 2) is {}", result.0 * result.1 * result.2);
}
