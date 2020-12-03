use std::error::Error;
use std::fs::File;
use std::io::Read;

struct TobogganMap {
    map: Vec<u8>,
    height: usize,
    width: usize,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl TobogganMap {
    fn from_file(file_name: &str) -> Result<TobogganMap, Box<dyn Error>> {
        let mut file = File::open(file_name)?;
        let mut content = Vec::new();

        file.read_to_end(&mut content)?;

        let width = content.iter().position(|&e| e == b'\n').unwrap();
        let height = content.len() / (width + 1);

        Ok(TobogganMap {
            map: content,
            height,
            width,
        })
    }

    pub fn is_tree(&self, point: Point) -> bool {
        let pos = ((point.y % self.height) * (self.width + 1) + (point.x % self.width));
        if let Some(val) = self.map.get(pos) {
            *val == b'#'
        } else {
            panic!(
                "Tried to access a nonexistent position {} {:?} in map with height {} and width {} (length {})",
                pos, point, self.height, self.width, self.map.len()
            );
        }
    }

    pub fn tree_count(&self, origin: Point, slope: Point) -> usize {
        let mut position = origin;
        let mut tree_count = 0;
        loop {
            position.x += slope.x;
            position.y += slope.y;
            if position.y >= self.height {
                break;
            }
            if self.is_tree(position) {
                tree_count += 1;
            }
        }

        tree_count
    }
}

fn main() {
    let map = TobogganMap::from_file("input.txt").unwrap();
    println!(
        "Tree count with 3/1 slope: {}",
        map.tree_count(Point { x: 0, y: 0 }, Point { x: 3, y: 1 })
    );

    // Part 2
    let vectors = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];
    let c = vectors
        .iter()
        .map(|&v| map.tree_count(Point { x: 0, y: 0 }, v))
        .fold(1, |acc, x| acc * x);

    println!("Tree count product: {}", c);
}
