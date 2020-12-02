use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    pub fn from_string(input: &str) -> Result<Password, Box<dyn Error>> {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)")?;

        let capture = re.captures(input).ok_or("Bad password format")?;
        let min = capture
            .get(1)
            .ok_or("Bad password format")?
            .as_str()
            .parse()?;
        let max = capture
            .get(2)
            .ok_or("Bad password format")?
            .as_str()
            .parse()?;
        let letter = capture
            .get(3)
            .ok_or("Bad password format")?
            .as_str()
            .chars()
            .next()
            .ok_or("Bad password format")?;
        let password = capture
            .get(4)
            .ok_or("Bad password format")?
            .as_str()
            .to_string();

        if min <= password.len() && max <= password.len() {
            Ok(Password {
                min,
                max,
                letter,
                password,
            })
        } else {
            None.ok_or("Bad password format")?
        }
    }

    pub fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .fold(0, |acc, c| if c == self.letter { acc + 1 } else { acc });

        count >= self.min && count <= self.max
    }

    pub fn is_valid_part_2(&self) -> bool {
        let first_index = self.min - 1;
        let second_index = self.max - 1;

        // unwrap() is safe because indices are validated.
        (self.password.chars().nth(first_index).unwrap() as char == self.letter)
            ^ (self.password.chars().nth(second_index).unwrap() as char == self.letter)
    }
}

fn from_file(file_name: &str) -> Result<Vec<Password>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    content
        .trim()
        .lines()
        .map(|x| Ok(Password::from_string(x.trim())?))
        .collect()
}

fn main() {
    let pws = from_file("input.txt").expect("Unable to parse passwords");
    let pw_count = pws
        .iter()
        .fold(0, |acc, pw| if pw.is_valid() { acc + 1 } else { acc });

    println!("{} passwords are valid (Part 1)", pw_count);

    let pw_count = pws.iter().fold(
        0,
        |acc, pw| if pw.is_valid_part_2() { acc + 1 } else { acc },
    );

    println!("{} passwords are valid (Part 2)", pw_count);
}
