use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::ops::RangeInclusive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let lines = read_lines(path).expect("We have a no go");
    let binding = lines.map(|r| r.unwrap()).collect::<Vec<String>>();

    let mut score = 0;

    for line in binding {
        let split = line.split(',').collect::<Vec<&str>>();//.map(|str| str.split('-')).collecT::<Vec<str>>();
        let another_split = split.iter().map(|str| str.split('-').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

        let first = &another_split[0].iter().map(|ch| ch.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let second = &another_split[1].iter().map(|ch| ch.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let range1 = RangeInclusive::new(first[0], first[1]);
        let range2 = RangeInclusive::new(second[0], second[1]);

        if contains_any(&range1, &range2) || contains_any(&range2, &range1) {
            score = score + 1;
        }
    }
    println!("{}", score);
}

fn contains_all(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    let mut contains = true;
    for num in range2.clone() {
        if !range1.contains(&num) {
            contains = false;
            break;
        }
    }
    return contains;
}

fn contains_any(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    let mut contains = false;
    for num in range2.clone() {
        if range1.contains(&num) {
            contains = true;
            break;
        }
    }
    return contains;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
