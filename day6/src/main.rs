use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn part_1(file_name: &str) -> usize {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    content
        .trim()
        .split("\n\n")
        .map(|group| group.chars().filter(|&c| c != '\n').collect::<HashSet<char>>())
        .map(|g| g.len())
        .sum()
}

fn part_2(file_name: &str) -> usize {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    content
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .fold(None, |acc: Option<HashSet<char>>, x: HashSet<char>| {
                    if let Some(a) = acc {
                        Some(a.intersection(&x).cloned().collect())
                    } else {
                        Some(x)
                    }
                })
        })
        .map(|s| s.unwrap().len())
        .sum()
}

fn main() {
    println!(
        "Sum of group counts: {}",
        part_1("input.txt")
    );

    println!("Sum of group counts (part 2): {}", part_2("input.txt"));
}
