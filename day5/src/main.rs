use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    times: usize,
    from: usize,
    to: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_crates = &args[1];
    let path_moves = &args[2];

    let crates = parse_input(path_crates);
    let moves_vec = parse_input(path_moves);
    let mut stacks = build_stacks(crates);

    print_stacks(&stacks);

    for move_str in moves_vec {
        let m = parse_move(move_str);
        println!("{:?}", m);
        println!();

        let mut temp = VecDeque::<char>::new();

        for _ in 0..(m.times) {
            let from = stacks.get_mut(&m.from).unwrap();
            let object = from.pop().unwrap();
            temp.push_front(object);
        }

        for _ in 0..(m.times) {
            let to = stacks.get_mut(&m.to).unwrap();
            let object = temp.pop_front().unwrap();
            to.push(object);
        }


        print_stacks(&stacks);
        println!();
    }
    print_stacks(&stacks);
}

fn print_stacks(stacks: &HashMap<usize, Vec<char>>) {
    for i in 1..=9 {
        println!("{:?}", stacks.get(&i));
    }
}


fn parse_move(str: String) -> Move {
    let moves = str.split(',').collect::<Vec<&str>>();

    Move {
        times: to_int(moves[0]),
        from: to_int(moves[1]),
        to: to_int(moves[2])
    }
}

fn to_int(string: &str) -> usize {
    string.parse::<usize>().unwrap()
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
