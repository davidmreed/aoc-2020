use core::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
enum Opcode {
    Mask(u64, u64, u64),
    Assign(u64, u64),
}

impl FromStr for Opcode {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Opcode, Box<dyn Error>> {
        let mask_re = Regex::new(r"mask = ([01X]+)")?;
        let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

        if let Some(capture) = mask_re.captures(input) {
            let mut mask_1s: u64 = 0;
            let mut mask_0s: u64 = 0;
            let mut mask_fluctuate: u64 = 0;

            // For the 1s mask (|'ed), we need a 64-bit number with
            // the bit positions that are 1 in the mask set to 1.
            // For the 0s mask (&'ed), we need a 64-bit number
            // with the bit positions that are nonzero set to 1.
            // For the fluctuating mask, we need to record all bit positions that are Xs.

            for (i, &c) in capture
                .get(1)
                .ok_or("Bad format")?
                .as_str()
                .as_bytes()
                .iter()
                .enumerate()
            {
                if c != b'0' {
                    mask_0s |= 1 << (35 - i);
                }
                if c == b'1' {
                    mask_1s |= 1 << (35 - i)
                }
                if c == b'X' {
                    mask_fluctuate |= 1 << (35 - i);
                }
            }

            return Ok(Opcode::Mask(mask_0s, mask_1s, mask_fluctuate));
        }
        if let Some(capture) = mem_re.captures(input) {
            return Ok(Opcode::Assign(
                capture.get(1).ok_or("Bad format")?.as_str().parse()?,
                capture.get(2).ok_or("Bad format")?.as_str().parse()?,
            ));
        }
        None.ok_or("Bad format: no match")?
    }
}

struct VM {
    mask_0: u64,
    mask_1: u64,
    mask_fluctuate: u64,
    instructions: Vec<Opcode>,
    memory: HashMap<u64, u64>,
}

impl VM {
    fn from_file(file_name: &str) -> Result<VM, Box<dyn Error>> {
        let mut file = File::open(file_name)?;
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();
        Ok(VM {
            instructions: content
                .trim()
                .lines()
                .map(|x| Ok(x.trim().parse::<Opcode>()?))
                .collect::<Result<Vec<Opcode>, Box<dyn Error>>>()?,
            mask_1: 0_u64,
            mask_0: 0_u64,
            mask_fluctuate: 0_u64,
            memory: HashMap::new(),
        })
    }

    fn run(&mut self) {
        for i in &self.instructions {
            match i {
                Opcode::Mask(mask_0, mask_1, mask_fluctuate) => {
                    self.mask_0 = *mask_0;
                    self.mask_1 = *mask_1;
                    self.mask_fluctuate = *mask_fluctuate;
                }
                Opcode::Assign(loc, value) => {
                    self.memory
                        .insert(*loc, (*value | self.mask_1) & self.mask_0);
                }
            }
        }
    }

    fn run_part2(&mut self) {
        for i in &self.instructions {
            match i {
                Opcode::Mask(mask_0, mask_1, mask_fluctuate) => {
                    self.mask_0 = *mask_0;
                    self.mask_1 = *mask_1;
                    self.mask_fluctuate = *mask_fluctuate;
                }
                Opcode::Assign(loc, value) => {
                    let locs = self.fluctuating_locations(*loc);
                    for new_loc in locs {
                        self.memory.insert(new_loc, *value);
                    }
                }
            }
        }
    }

    fn fluctuating_locations(&self, loc: u64) -> Vec<u64> {
        let mut locs = Vec::new();
        let mut masks: Vec<(u64, u64)> = Vec::new();

        // Create a bitmask to apply for each value (on/off) of each fluctuating bit.
        for i in 0..36 {
            let this_bit: u64 = 1 << i;
            if self.mask_fluctuate & this_bit != 0 {
                // This is a fluctuating bit.
                masks.push((this_bit, !this_bit)); // Mask to SET this bit via |, Mask to UNSET this bit via &
            }
        }

        for index in 0..(2_u64.pow(masks.len() as u32)) {
            // Each number between 0 and 2^len is a series of bits denoting on-off state per mask
            let mut this_loc = loc;
            for (i, m) in masks.iter().enumerate() {
                // If bit i is set in the index, use the "setter" mask for this index
                // Otherwise, use the unsetter mask
                if index & 1 << i != 0 {
                    this_loc |= m.0;
                } else {
                    this_loc &= m.1;
                }
            }
            this_loc = this_loc | self.mask_1; // We don't apply the zeros mask in Mode 2
            locs.push(this_loc);
        }

        locs
    }
}

fn main() {
    let mut vm = VM::from_file("input.txt").expect("Unable to parse instructions");

    vm.run();

    println!(
        "I have memory locations summing to {}",
        vm.memory.values().sum::<u64>()
    );

    let mut vm = VM::from_file("input.txt").expect("Unable to parse instructions");
    vm.run_part2();

    println!(
        "For Part 2, I have memory locations summing to {}",
        vm.memory.values().sum::<u64>()
    );

}
