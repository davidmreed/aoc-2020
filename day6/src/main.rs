use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn from_file(file_name: &str) -> Vec<HashSet<char>> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    let mut vec = Vec::new();

    file.read_to_string(&mut content).unwrap();
    for group in content.trim().split("\n\n") {
        let mut hs = HashSet::new();
        for c in group.chars() {
            if c != '\n' {
                hs.insert(c);
            }
        }

        vec.push(hs);
    }
    vec
}

fn from_file_part_2(file_name: &str) -> Vec<HashSet<char>> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();

    let mut vec = Vec::new();

    file.read_to_string(&mut content).unwrap();
    for group in content.trim().split("\n\n") {
        let mut hs = HashSet::new();
        let mut first = true;
        for person in group.split_whitespace() {
            let mut this_person = HashSet::new();
            for c in person.chars() {
                this_person.insert(c);
            }
            if !first {
                hs = hs.intersection(&this_person).cloned().collect();
            } else {
                hs = this_person;
                first = false;
            }
        }

        vec.push(hs);
    }
    vec
}


fn main() {
    let groups = from_file("input.txt");

    println!(
        "Sum of group counts: {}",
        groups.iter().map(|g| g.len()).sum::<usize>()
    );

    let groups_part_2 = from_file_part_2("input.txt");

    println!(
        "Sum of group counts (part 2): {}",
        groups_part_2.iter().map(|g| g.len()).sum::<usize>()
    );

}
