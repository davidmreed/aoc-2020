use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use core::str::FromStr;
enum Opcode {
    Mask(u64, u64),
    Assign(u32, u64)
}

impl FromStr for Opcode {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Opcode, Box<dyn Error>> {
        let mask_re = Regex::new(r"mask = ([01X]+)")?;
        let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

        if let Some(capture) = mask_re.captures(input) {
            let mut mask_1s: u64 = 0;
            let mut mask_0s: u64 = 0;

            // For the 1s mask (|'ed), we need a 64-bit number with
            // the bit positions that are 1 in the mask set to 1.
            // For the 0s mask (&'ed), we need a 64-bit number
            // with the bit positions that are nonzero set to 1.

            for (i, &c) in capture.get(1).ok_or("Bad format")?.as_str().as_bytes().iter().enumerate() {
                if c != b'0' {
                    mask_0s |= 1 << (35 - i);
                }
                if c == b'1' {
                    mask_1s |= 1 << (35 - i)
                }
            }

            return Ok(Opcode::Mask(mask_0s, mask_1s));
        }
        if let Some(capture) = mem_re.captures(input) {
            return Ok(Opcode::Assign(capture.get(1).ok_or("Bad format")?.as_str().parse()?, capture.get(2).ok_or("Bad format")?.as_str().parse()?));
        }
        None.ok_or("Bad format: no match")?
    }
}

struct VM {
    mask_1: u64,
    mask_0: u64,
    instructions: Vec<Opcode>,
    memory: HashMap<u32, u64>
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
            memory: HashMap::new(),
        })
    }

    fn run(&mut self) {
        for i in &self.instructions {
            match i {
                Opcode::Mask(mask_0, mask_1) => { self.mask_0 = *mask_0; self.mask_1 = *mask_1; },
                Opcode::Assign(loc, value) => {
                    self.memory.insert(*loc, (*value | self.mask_1) & self.mask_0);
                }
            }
        }
    }
}

fn main() {
    let mut vm = VM::from_file("input.txt").expect("Unable to parse instructions");

    vm.run();

    println!("I have memory locations summing to {}", vm.memory.values().sum::<u64>());
}
