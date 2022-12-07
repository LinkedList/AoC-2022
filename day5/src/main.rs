use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_crates = &args[1];
    let path_moves = &args[2];

    let crates = parse_input(path_crates);
    let moves = parse_input(path_moves);

    let mut stacks = build_stacks(crates);
    println!("{:?}", stacks);
}

fn build_stacks(crates: Vec<String>) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::<usize, Vec<char>>::new();
    for (pos, stack_str) in crates.iter().enumerate() {
        let mut stack = Vec::<char>::new();
        for ch in stack_str.chars() {
            stack.push(ch);
        }
        stacks.insert(pos + 1, stack);
    }

    return stacks;
}

fn parse_input<P>(filename:P) -> Vec<String> where P: AsRef<Path>, {
    let lines = read_lines(filename).expect("We have a no go reading lines");
    lines.map(|r| r.unwrap()).collect::<Vec<String>>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
