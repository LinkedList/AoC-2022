use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use sorted_vec::ReverseSortedVec;
use std::cmp::Reverse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut sums: ReverseSortedVec<u32> = ReverseSortedVec::<u32>::new();
    let mut local_sum: u32 = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    sums.insert(Reverse(local_sum.clone()));
                    local_sum = 0;
                } else {
                    local_sum = local_sum + line.parse::<u32>().expect("Not an integer");
                }
            }
        }
    }

    println!("{:?}", sums.iter().take(3).map(|r| r.0).sum::<u32>());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

