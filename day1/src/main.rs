use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let fileStr = fs::read_to_string(path).expect("Should have been able to read the file");;

    let mut split = fileStr.split("\n\n");
    let mut total: Vec<u32> = Vec::new();

    for s in split {
        let lines: Vec<u32> = s.lines().into_iter().map(|s| s.parse::<u32>().expect("This is not a integer")).collect();
        let sum: u32 = lines.iter().sum();
        total.push(sum)
    }
    total.sort();
    let result: u32 = total.iter().rev().take(3).sum();

    println!("{:?}", result);
}
