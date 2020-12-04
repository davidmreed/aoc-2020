use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn from_file(file_name: &str) -> Vec<HashMap<String, String>> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    let mut vec = Vec::new();

    file.read_to_string(&mut content).unwrap();
    for passport in content.trim().split("\n\n") {
        let mut map = HashMap::new();
        for element in passport.trim().split_whitespace() {
            let mut components = element.split(':');
            map.insert(
                components.next().unwrap().to_string(),
                components.next().unwrap().to_string(),
            );
        }

        vec.push(map);
    }
    vec
}

fn legal_number(value: &str, min: i32, max: i32) -> bool {
    if let Ok(num) = value.parse::<i32>() {
        num >= min && num <= max
    } else {
        false
    }
}

fn is_valid(passport: &HashMap<String, String>, validate_keys: bool) -> bool {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    keys.iter().all(|&k| {
        passport.contains_key(k)
            && (!validate_keys || {
                let v = passport.get(k).unwrap();

                if !match k {
                    "byr" => legal_number(v, 1920, 2002),
                    "iyr" => legal_number(v, 2010, 2020),
                    "eyr" => legal_number(v, 2020, 2030),
                    "hcl" => {
                        v.len() == 7
                            && v.starts_with('#')
                            && v.chars().all(|c| "#0123456789abcdef".contains(c))
                    }
                    "hgt" => {
                        if v.ends_with("cm") {
                            legal_number(v.trim_end_matches("cm"), 150, 193)
                        } else if v.ends_with("in") {
                            legal_number(v.trim_end_matches("in"), 59, 76)
                        } else {
                            false
                        }
                    }
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter()
                        .position(|q| q == v)
                        .is_some(),
                    "pid" => v.len() == 9 && v.parse::<i32>().is_ok(),
                    _ => false,
                } {
                    return false;
                } else {
                    true
                }
            })
    })
}

fn main() {
    let passports = from_file("input.txt");

    println!(
        "Part 1: I found {} valid passports",
        passports
            .iter()
            .map(|x| is_valid(x, false))
            .fold(0, |acc, x| if x { acc + 1 } else { acc })
    );

    println!(
        "Part 2: I found {} valid passports",
        passports
            .iter()
            .map(|x| is_valid(x, true))
            .fold(0, |acc, x| if x { acc + 1 } else { acc })
    );
}
