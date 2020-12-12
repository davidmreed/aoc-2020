use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_legal_skips(nums: &[u32]) -> u64 {
    let mut total_paths = 1;
    // The complete list is a legal path.
    // 1 2 3 4 5
    // If 2 is a legal skip, then 1 3 4 5 is a legal path, and we need to find its legal skips:
    // 1 3 5 => done
    // Then if 3 is a legal skip, 1 2 4 5 is a legal path. What are its skips?
    // 1 2 5 => done

    if nums.len() > 0 {
        for (i, n) in nums.iter().skip(1).enumerate() {
            // Is the nth number a legal skip?
            if i + 2 < nums.len() && nums[i + 2] - nums[i] <= 3 {
                total_paths += find_legal_skips(&nums[i + 2..]);
            }
        }
    }

    total_paths
}

fn legal_paths(nums: &[u32], val: u32) -> u64 {
    if nums.len() == 0 {
        return 1;
    }

    let mut total_paths = 0;

    if nums.len() >= 1 && nums[0] - val <= 3 {
        // First out is a valid choice
        total_paths += legal_paths(&nums[1..], nums[0])
    }
    if nums.len() >= 2 && nums[1] - val <= 3 {
        // Second out is a valid choice
        total_paths += legal_paths(&nums[2..], nums[1])
    }
    if nums.len() >= 3 && nums[2] - val <= 3 {
        // Third out is a valid choice
        total_paths += legal_paths(&nums[3..], nums[2])
    }

    return total_paths;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("input.txt")?);

    let mut numbers = file
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<u32>>();

    numbers.push(0);
    numbers.sort();
    numbers.push(numbers[numbers.len() - 1] + 3);

    let diffs_counts = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| b - a)
        .fold((0, 0, 0), |(ones, twos, threes), x| match x {
            1 => (ones + 1, twos, threes),
            2 => (ones, twos + 1, threes),
            3 => (ones, twos, threes + 1),
            _ => panic!("Bad differential {}", x),
        });

    println!("Diffs: {:?}", diffs_counts);
    println!("Output: {}", diffs_counts.0 * diffs_counts.2);

    let skippables: u32 = (1..numbers.len() - 1).fold(0, |acc, x| {
        if numbers[x + 1] - numbers[x - 1] <= 3 {
            acc + 1
        } else {
            acc
        }
    });
    println!("Skippables: {}", skippables);
    // Maybe try: for each Skippable, check which other skippables are still skippable...? But are there situations where two skippables being skipped make a third non-skippable?
    // (0) 1 2 3 4 5
    // 3 is _per se_ skippable but is not skippable if 1 and 2 are both skipped
//    println!("find_legal_skips: {}", find_legal_skips(&numbers[..]));
    println!("find_legal_paths: {}", legal_paths(&numbers[1..], 0));
    Ok(())
}
