use std::fs::File;
use std::io::Read;

const HEADINGS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Ship {
    heading: i32,
    x: i32,
    y: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            heading: 90,
            x: 0,
            y: 0,
        }
    }
    pub fn execute(&mut self, cmd: &str) {
        let (order, operand) = cmd.split_at(1);
        let amount = operand.parse::<i32>().unwrap();

        match order {
            "N" => {
                self.y -= amount;
            }
            "S" => {
                self.y += amount;
            }
            "W" => {
                self.x -= amount;
            }
            "E" => {
                self.x += amount;
            }
            "L" => {
                self.heading -= amount;

                if self.heading < 0 {
                    self.heading = 360 + self.heading;
                    self.heading = self.heading % 360;
                }
            }
            "R" => {
                self.heading += amount;
                if self.heading >= 360 {
                    self.heading = self.heading % 360;
                }
            }
            "F" => {
                self.x += amount * HEADINGS[(self.heading / 90) as usize].0;
                self.y += amount * HEADINGS[(self.heading / 90) as usize].1;
            }
            _ => panic!("Invalid instruction {}", order),
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();

    let mut ship = Ship::new();

    file.read_to_string(&mut content).unwrap();
    for cmd in content.trim().lines() {
        ship.execute(cmd);
    }

    println!(
        "Got Manhattan distance: {} for position x: {}, y: {}",
        ship.x.abs() + ship.y.abs(),
        ship.x,
        ship.y
    );
}
