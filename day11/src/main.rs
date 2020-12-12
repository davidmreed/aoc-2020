use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Delta {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum Mode {
    Adjacency,
    Visibility,
}

#[derive(Clone, Debug, PartialEq)]
struct StateMachine {
    state: Vec<u8>,
    size: Pair,
    mode: Mode,
}

struct VectorIterator {
    size: Pair,
    cur: Pair,
    vector: Delta,
}

impl Iterator for VectorIterator {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let next_x = (self.cur.x as i32) + self.vector.x;
        let next_y = (self.cur.y as i32) + self.vector.y;

        if next_x < 0
            || next_x as usize >= self.size.x
            || next_y < 0
            || next_y as usize >= self.size.y
        {
            None
        } else {
            self.cur = Pair {
                x: next_x as usize,
                y: next_y as usize,
            };
            Some(self.cur)
        }
    }
}

impl fmt::Display for StateMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size: ({}, {})\n", self.size.x, self.size.y)?;
        for row in 0..self.size.y {
            write!(
                f,
                "{}",
                std::str::from_utf8(
                    &self.state[((self.size.x + 1) * row)..((self.size.x + 1) * (row + 1))]
                )
                .unwrap()
            )?;
        }
        Ok(())
    }
}

impl StateMachine {
    fn from_vec(content: Vec<u8>, mode: Mode) -> Result<StateMachine, Box<dyn Error>> {
        let x = content
            .iter()
            .position(|&e| e == b'\n')
            .ok_or("invalid format")?;
        let y = content.len() / (x + 1);

        Ok(StateMachine {
            state: content,
            size: Pair { x, y },
            mode,
        })
    }
    fn from_file(file_name: &str, mode: Mode) -> Result<StateMachine, Box<dyn Error>> {
        let mut file = File::open(file_name)?;
        let mut content = Vec::new();

        file.read_to_end(&mut content)?;
        StateMachine::from_vec(content, mode)
    }

    fn from_string(content: &str, mode: Mode) -> Result<StateMachine, Box<dyn Error>> {
        StateMachine::from_vec(content.as_bytes().to_vec(), mode)
    }

    pub fn get_occupied_seats(&self) -> usize {
        (0..self.state.len())
            .filter(|&x| *self.state.get(x).unwrap() == b'#')
            .count()
    }

    pub fn set_cell(&mut self, cell: Pair, value: u8) {
        if cell.x < self.size.x && cell.y < self.size.y {
            let pos = cell.y * (self.size.x + 1) + cell.x;
            self.state[pos] = value;
        }
    }

    pub fn get_cell(&self, cell: Pair) -> Option<&u8> {
        if cell.x < self.size.x && cell.y < self.size.y {
            let pos = cell.y * (self.size.x + 1) + cell.x;
            self.state.get(pos)
        } else {
            None
        }
    }

    pub fn get_neighbor_value(&self, cell: Pair) -> u8 {
        if let Some(&x) = self.get_cell(cell) {
            if x == b'#' {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn get_neighbor_count(&self, cell: Pair) -> u8 {
        let mut count = 0;
        let x = cell.x as i32;
        let y = cell.y as i32;

        for &x in [x - 1, x, x + 1].iter() {
            for &y in [y - 1, y, y + 1].iter() {
                let pair = Pair {
                    x: x as usize,
                    y: y as usize,
                };
                if pair == cell {
                    continue;
                }
                count += self.get_neighbor_value(pair);
            }
        }

        count
    }

    fn get_vector_iterator(&self, cell: Pair, vector: Delta) -> VectorIterator {
        VectorIterator {
            size: self.size,
            cur: cell,
            vector: vector,
        }
    }

    pub fn get_visible_count(&self, cell: Pair) -> u8 {
        let mut count = 0;

        for &x_delta in [-1, 0, 1].iter() {
            for &y_delta in [-1, 0, 1].iter() {
                if x_delta == 0 && y_delta == 0 {
                    continue;
                }

                let first_seat = self
                    .get_vector_iterator(
                        cell,
                        Delta {
                            x: x_delta,
                            y: y_delta,
                        },
                    )
                    .filter(|&cell| *self.get_cell(cell).unwrap() != b'.')
                    .next();

                count +=
                    if first_seat.is_some() && self.get_neighbor_value(first_seat.unwrap()) == 1 {
                        1
                    } else {
                        0
                    };
            }
        }

        count
    }

    pub fn next_state(&self) -> StateMachine {
        let mut next = self.clone();
        let threshold = if let Mode::Adjacency = self.mode {
            4
        } else {
            5
        };

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let pair = Pair { x, y };
                let cell = *self.get_cell(pair).unwrap();

                let neighbor_value = if let Mode::Adjacency = self.mode {
                    self.get_neighbor_count(pair)
                } else {
                    self.get_visible_count(pair)
                };

                if cell == b'L' && neighbor_value == 0 {
                    // This seat becomes occupied
                    next.set_cell(pair, b'#');
                } else if cell == b'#' && neighbor_value >= threshold {
                    // This seat becomes empty
                    next.set_cell(pair, b'L');
                }
            }
        }

        next
    }
}

fn main() {
    // Part 1
    let mut sm = StateMachine::from_file("input.txt", Mode::Adjacency).unwrap();

    loop {
        let next = sm.next_state();
        if next == sm {
            println!(
                "Found stable state with {} occupied seats",
                sm.get_occupied_seats()
            );
            break;
        }

        sm = next;
    }

    let mut sm = StateMachine::from_file("input.txt", Mode::Visibility).unwrap();

    loop {
        let next = sm.next_state();
        if next == sm {
            println!(
                "Found stable state with {} occupied seats",
                sm.get_occupied_seats()
            );
            break;
        }

        sm = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let sm = StateMachine::from_string("..L#\nLLLL\n", Mode::Adjacency);
        assert!(sm.is_ok());
        let sm = sm.unwrap();
        assert_eq!(4, sm.size.x);
        assert_eq!(2, sm.size.y);
    }

    #[test]
    fn test_eq() {
        let sm = StateMachine::from_string("..L#\nLLLL\n", Mode::Adjacency).unwrap();
        let other = StateMachine::from_string("..L#\nLLLL\n", Mode::Adjacency).unwrap();

        assert_eq!(sm, other);
    }

    #[test]
    fn test_next_eq() {
        let sm = StateMachine::from_string("....\n....\n", Mode::Adjacency).unwrap();
        let next = sm.next_state();

        assert_eq!(sm, next);
    }

    #[test]
    fn test_next() {
        let sm = StateMachine::from_string(
            "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
            Mode::Adjacency,
        )
        .unwrap();

        let next = sm.next_state();

        println!("{}", next);

        assert_eq!(
            next,
            StateMachine::from_string(
                "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
                Mode::Adjacency
            )
            .unwrap()
        );
    }

    #[test]
    fn test_get_cell() {
        let sm = StateMachine::from_string(
            "\
L.LL.LL.LL
LLLLLLL.LL
",
            Mode::Adjacency,
        )
        .unwrap();

        assert_eq!(b'L', *sm.get_cell(Pair { x: 0, y: 0 }).unwrap());
        assert_eq!(b'.', *sm.get_cell(Pair { x: 1, y: 0 }).unwrap());
        assert_eq!(b'L', *sm.get_cell(Pair { x: 9, y: 1 }).unwrap());
        assert_eq!(None, sm.get_cell(Pair { x: 20, y: 20 }));
    }

    #[test]
    fn test_set_cell() {
        let mut sm = StateMachine::from_string(
            "\
L.LL.LL.LL
LLLLLLL.LL
",
            Mode::Adjacency,
        )
        .unwrap();
        assert_eq!(b'L', *sm.get_cell(Pair { x: 0, y: 0 }).unwrap());
        sm.set_cell(Pair { x: 0, y: 0 }, b'#');
        assert_eq!(b'#', *sm.get_cell(Pair { x: 0, y: 0 }).unwrap());
    }

    #[test]
    pub fn get_neighbor_value() {
        let sm = StateMachine::from_string(
            "\
#.LL.LL.LL
LLLLLLL.LL
",
            Mode::Adjacency,
        )
        .unwrap();

        assert_eq!(1, sm.get_neighbor_value(Pair { x: 0, y: 0 }));
        assert_eq!(0, sm.get_neighbor_value(Pair { x: 1, y: 0 }));
    }

    #[test]
    pub fn get_neighbor_count() {
        let sm = StateMachine::from_string(
            "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
            Mode::Adjacency,
        )
        .unwrap();
        assert_eq!(2, sm.get_neighbor_count(Pair { x: 0, y: 0 }));
        assert_eq!(8, sm.get_neighbor_count(Pair { x: 5, y: 8 }));
        assert_eq!(2, sm.get_neighbor_count(Pair { x: 9, y: 9 }));
    }
}
