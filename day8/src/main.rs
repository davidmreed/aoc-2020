use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;
use core::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct OpcodeErr;
impl Error for OpcodeErr {}

impl fmt::Display for OpcodeErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpcodeErr")
    }
}

#[derive(Copy, Clone)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Opcode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut halves = s.trim().split(" ");
        match halves.next().ok_or(OpcodeErr)? {
            "acc" => Ok(Opcode::Acc(halves.next().ok_or(OpcodeErr)?.parse()?)),
            "jmp" => Ok(Opcode::Jmp(halves.next().ok_or(OpcodeErr)?.parse()?)),
            "nop" => Ok(Opcode::Nop(halves.next().ok_or(OpcodeErr)?.parse()?)),
            _ => Err(Box::new(OpcodeErr)),
        }
    }
}

struct VM {
    ops: Vec<Opcode>,
    ip: i32,
    acc: i32,
    visited: HashSet<i32>,
}

impl VM {
    pub fn new(ops: Vec<Opcode>) -> VM {
        VM {
            ops,
            ip: 0,
            acc: 0,
            visited: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> Result<i32, i32> {
        loop {
            if !self.visited.contains(&self.ip) {
                self.visited.insert(self.ip);

                if self.ip as usize == self.ops.len() {
                    return Ok(self.acc);
                }
                match self.ops[self.ip as usize] {
                    Opcode::Acc(d) => {
                        self.acc += d;
                        self.ip += 1;
                    }
                    Opcode::Jmp(d) => {
                        self.ip += d;
                    }
                    Opcode::Nop(_) => {
                        self.ip += 1;
                    }
                }
            } else {
                return Err(self.acc);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("input.txt")?);

    let opcodes = file
        .lines()
        .map(|x| x?.parse())
        .collect::<Result<Vec<Opcode>, Box<dyn Error>>>()?;

    if let Err(r) = VM::new(opcodes.clone()).run() {
        println!("Part 1 value: {}", r);
    }

    // Part 2
    for (i, op) in opcodes.iter().enumerate() {
        match op {
            Opcode::Nop(d) => {
                let mut mutated_ops = opcodes.clone();
                mutated_ops[i] = Opcode::Jmp(*d);
                if let Ok(r) = VM::new(mutated_ops).run() {
                    println!("Part 2 value: {}", r);
                }
            },
            Opcode::Jmp(d) => {
                let mut mutated_ops = opcodes.clone();
                mutated_ops[i] = Opcode::Nop(*d);
                if let Ok(r) = VM::new(mutated_ops).run() {
                    println!("Part 2 value: {}", r);
                }
            }
            _ => {}
        }
    }
    Ok(())
}
