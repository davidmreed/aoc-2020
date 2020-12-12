use std::fs::File;
use std::io::Read;

const HEADINGS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Debug)]
struct Point(i32, i32);

struct Ship {
    heading: i32,
    pos: Point,
    waypoint: Point,
    mode: Mode,
}

enum Mode {
    Normal,
    Waypoint,
}

impl Ship {
    pub fn new(mode: Mode) -> Ship {
        Ship {
            heading: 90,
            pos: Point(0, 0),
            waypoint: Point(10, -1),
            mode,
        }
    }

    fn perform_move(&mut self, delta_x: i32, delta_y: i32) {
        if let Mode::Normal = self.mode {
            self.pos.0 += delta_x;
            self.pos.1 += delta_y;
        } else {
            self.waypoint.0 += delta_x;
            self.waypoint.1 += delta_y;
        }
    }

    pub fn execute(&mut self, cmd: &str) {
        let (order, operand) = cmd.split_at(1);
        let amount = operand.parse::<i32>().unwrap();

        match order {
            "N" => self.perform_move(0, -amount),
            "S" => self.perform_move(0, amount),
            "W" => self.perform_move(-amount, 0),
            "E" => self.perform_move(amount, 0),
            "L" => {
                if let Mode::Normal = self.mode {
                    self.heading -= amount;

                    if self.heading < 0 {
                        self.heading = 360 + self.heading;
                        self.heading = self.heading % 360;
                    }
                } else {
                    let rotation = amount % 360;
                    match rotation {
                        270 => self.waypoint = Point(-self.waypoint.1, self.waypoint.0),
                        180 => self.waypoint = Point(-self.waypoint.0, -self.waypoint.1),
                        90 => self.waypoint = Point(self.waypoint.1, -self.waypoint.0),
                        _ => panic!("Invalid rotation {}", rotation),
                    };
                }
            }
            "R" => {
                if let Mode::Normal = self.mode {
                    self.heading += amount;
                    if self.heading >= 360 {
                        self.heading = self.heading % 360;
                    }
                } else {
                    let rotation = amount % 360;
                    match rotation {
                        90 => self.waypoint = Point(-self.waypoint.1, self.waypoint.0),
                        180 => self.waypoint = Point(-self.waypoint.0, -self.waypoint.1),
                        270 => self.waypoint = Point(self.waypoint.1, -self.waypoint.0),
                        _ => panic!("Invalid rotation {}", rotation),
                    };
                }
            }
            "F" => {
                if let Mode::Normal = self.mode {
                    self.perform_move(
                        amount * HEADINGS[(self.heading / 90) as usize].0,
                        amount * HEADINGS[(self.heading / 90) as usize].1,
                    )
                } else {
                    self.pos.0 += amount * self.waypoint.0;
                    self.pos.1 += amount * self.waypoint.1;
                }
            }
            _ => panic!("Invalid instruction {}", order),
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();

    let mut ship = Ship::new(Mode::Normal);
    let mut ship_part2 = Ship::new(Mode::Waypoint);

    file.read_to_string(&mut content).unwrap();
    for cmd in content.trim().lines() {
        ship.execute(cmd);
        ship_part2.execute(cmd);
    }

    println!(
        "Part 1: got Manhattan distance: {} for position x: {}, y: {}",
        ship.pos.0.abs() + ship.pos.1.abs(),
        ship.pos.0,
        ship.pos.1
    );
    println!(
        "Part 2: got Manhattan distance: {} for position x: {}, y: {}",
        ship_part2.pos.0.abs() + ship_part2.pos.1.abs(),
        ship_part2.pos.0,
        ship_part2.pos.1
    );
}
