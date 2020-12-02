use regex::Regex;
use std::fs::File;
use std::io::Read;

struct Password {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl Password {
    pub fn from_string(input: &str) -> Password {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)").unwrap();

        let capture = re.captures(input).unwrap();

        Password {
            min: capture.get(1).unwrap().as_str().parse().unwrap(),
            max: capture.get(2).unwrap().as_str().parse().unwrap(),
            letter: capture.get(3).unwrap().as_str().chars().next().unwrap(),
            password: capture.get(4).unwrap().as_str().to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut count = 0;

        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }

    pub fn is_valid_part_2(&self) -> bool {
        let first_index: usize = self.min as usize - 1;
        let second_index: usize = self.max as usize - 1;

        (self.password.chars().nth(first_index).unwrap() as char == self.letter)
            ^ (self.password.chars().nth(second_index).unwrap() as char == self.letter)
    }
}

fn from_file(file_name: &str) -> Vec<Password> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    content
        .trim()
        .lines()
        .map(|x| Password::from_string(x.trim()))
        .collect()
}

fn main() {
    let pws = from_file("input.txt");
    let pw_count = pws
        .iter()
        .filter(|&pw| pw.is_valid())
        .collect::<Vec<&Password>>()
        .len();

    println!("{} passwords are valid (Part 1)", pw_count);

    let pw_count = pws
        .iter()
        .filter(|&pw| pw.is_valid_part_2())
        .collect::<Vec<&Password>>()
        .len();

    println!("{} passwords are valid (Part 2)", pw_count);
}
