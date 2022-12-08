use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let linesReader = read_lines(path).expect("We have a no go reading lines");
    let linesVec = linesReader.map(|r| r.unwrap()).collect::<Vec<String>>();
    let line = &linesVec[0];
    let iter = line.chars().tuple_windows::<(_, _, _, _)>();

    for (num, (a, b, c, d)) in iter.enumerate() {
        if(a != b && a != c && a != d && 
           b != c && b != d &&
           c != d) {
            println!("{}{}{}{}", a, b, c, d);
            println!("{:?}", num + 4);
            break;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
