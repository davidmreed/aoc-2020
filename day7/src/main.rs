use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Capacity {
    name: String,
    count: usize,
}

fn parse_rule(s: &str) -> (String, Vec<Capacity>) {
    let mut splits = s.split(" bags contain ");
    let name = splits.next().unwrap().to_string();
    let cap_string = splits.next().unwrap();
    let mut caps = Vec::new();

    if cap_string != "no other bags." {
        for cap in cap_string.split(", ") {
            let mut this_cap = cap.trim().trim_end_matches(".").splitn(2, " ");
            let count = this_cap.next().unwrap().trim().parse().unwrap();
            let mut name = this_cap.next().unwrap();
            if name.ends_with(" bags") {
                name = name.strip_suffix(" bags").unwrap();
            } else if name.ends_with(" bag") {
                name = name.strip_suffix(" bag").unwrap();
            }
            caps.push(Capacity {
                name: name.to_string(),
                count,
            });
        }
    }

    (name, caps)
}

fn from_file(file_name: &str) -> HashMap<String, Vec<Capacity>> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();
    let mut hm = HashMap::new();

    file.read_to_string(&mut content).unwrap();
    for (name, caps) in content.trim().lines().map(|x| parse_rule(x)) {
        hm.insert(name, caps);
    }
    hm
}

fn dfs(m: &HashMap<String, Vec<Capacity>>, v: &Vec<Capacity>, t: &str) -> usize {
    for c in v {
        if c.name == "shiny gold" || dfs(m, m.get(&c.name).unwrap(), t) == 1 {
            return 1;
        }
    }

    return 0;
}

fn count_capacity(m: &HashMap<String, Vec<Capacity>>, t: &str) -> usize {
    m.get(t)
        .unwrap()
        .iter()
        .map(|c| c.count + c.count * count_capacity(m, &c.name))
        .sum()
}

fn main() {
    let m = from_file("input.txt");

    // Locate paths to shiny gold bags.
    println!(
        "Paths to shiny gold: {}",
        m.keys()
            .filter(|&k| k != "shiny gold")
            .map(|k| dfs(&m, m.get(k).unwrap(), "shiny gold"))
            .sum::<usize>()
    );

    println!("Capacity of gold bag: {}", count_capacity(&m, "shiny gold"));
}
