use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut score = 0;

    //if let Ok(lines) = read_lines(path) {
    //for line in lines {
    //if let Ok(line) = line {
    //let split = line.chars().count() / 2;
    //let mut first = line[..split].chars().collect::<Vec<char>>();
    //let mut second = line[split..].chars().collect::<Vec<char>>();
    //
    //first.sort();
    //first.dedup();
    //second.sort();
    //second.dedup();
    //
    //println!("{:?}", first);
    //println!("{:?}", second);
    //
    //let both: u32 = first.iter().filter(|ch| second.contains(&ch)).map(|ch| priority(&ch)).sum();
    //score = score + both;
    //}
    //}

    let lines = read_lines(path).expect("We have a no go");
    let binding = lines.map(|r| r.unwrap()).collect::<Vec<String>>();
    let chunks = binding.chunks(3);
    for chunk in chunks {
        let mut first = chunk[0].chars().collect::<Vec<char>>();
        let mut second = chunk[1].chars().collect::<Vec<char>>();
        let mut third = chunk[2].chars().collect::<Vec<char>>();
        first.sort();
        first.dedup();
        second.sort();
        second.dedup();
        third.sort();
        third.dedup();

        let result: u32 = first.iter().filter(|ch| second.contains(&ch)).filter(|ch| third.contains(&ch)).map(|ch| priority(&ch)).sum();
        score = score + result;
    }
   

    println!("{}", score);
}

fn priority(ch: &char) -> u32 {
    let p = match ch {
        'a'..='z' => (*ch as u32) - 96,
        'A'..='Z' => (*ch as u32) - 38,
        _ => panic!("No!")
    };
    return p;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
