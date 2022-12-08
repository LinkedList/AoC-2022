use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let linesReader = read_lines(path).expect("We have a no go reading lines");
    let linesVec = linesReader.map(|r| r.unwrap()).collect::<Vec<String>>();
    let line = &linesVec[0];

    for (num, ch) in line.chars().enumerate() {
        let mut set = HashSet::<char>::new();
        let slice = &line[num..(num+14)];
        for ch in slice.chars() {
            set.insert(ch);
        }
        if set.len() == 14 {
            println!("{:?}", num + 14);
            break;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
