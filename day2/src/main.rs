use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut score = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                score = score + match line.as_str() {
                    // A Rock 1
                    // B Paper 2
                    // C Scissors 3

                    // X - Lose
                    // Y - Draw
                    // Z - Win

                    // Losses
                    "A X" => 3,
                    "B X" => 1,
                    "C X" => 2,

                    // Draws
                    "A Y" => 4,
                    "B Y" => 5,
                    "C Y" => 6,

                    // Wins
                    "A Z" => 8,
                    "B Z" => 9,
                    "C Z" => 7,
                    &_ => todo!("Not a match"),
                }
            }
        }
    }

    println!("{}", score);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

