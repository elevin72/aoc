
use core::panic;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use itertools::Itertools;


fn main() {
    println!("Hello, world!");
    println!("{}", star19());
    println!("{}", star20());
}

fn star19() -> u32 {
    let lines = get_input("./input").unwrap();
    let res: u32 = lines.iter().fold(0, |mut acc, line| {
        if let IllegalLineStatus::Broken(val) = get_line_status(line) {
            acc += val;
        }
        acc
    });
    res
}

fn star19_o() -> u32 {
    let lines = get_input("./input").unwrap();
    lines.iter().fold(0, |mut acc, line| {
        acc += star19_top(line);
        acc
    })
}

fn star19_top(line: &String) -> u32 {
    let first = &line[0..1];
    let rest = &line[1..];

}
fn star19_i(line: &String) -> u32 {
}

fn star20() -> u64 {
    let lines: Vec<u64> = get_input("./input").unwrap().iter().filter_map(|line| {
        match get_line_status(line) {
            IllegalLineStatus::Broken(_) => None,
            IllegalLineStatus::Incomplete(val) => Some(val),
        }
    }).sorted().collect_vec();
    let count = lines.len();
    lines[count/2]
}

enum IllegalLineStatus {
    Broken(u32),
    Incomplete(u64),
}

fn get_line_status(line: &String) -> IllegalLineStatus {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' | '}' | ']' | '>' => {
                let legal = match stack.pop().unwrap() {
                    '(' => c == ')',
                    '{' => c == '}',
                    '[' => c == ']',
                    '<' => c == '>',
                    _ => panic!("wutwut"),
                };
                if !legal {
                    let incomplete_value = match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("wutwut"),
                    };
                    return IllegalLineStatus::Broken(incomplete_value);
                }
            }
            _ => panic!("not recognized character: {}", c),
        }
    }
    let completion_string_val = stack.iter().rev().fold(0, |mut acc, c| {
        acc *= 5;
        match c {
            '(' => acc += 1,
            '[' => acc += 2,
            '{' => acc += 3,
            '<' => acc += 4,
            _ => panic!("ffff")
        };
        acc
    });
    return IllegalLineStatus::Incomplete(completion_string_val);
}

fn get_input(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: Vec<String> = reader
        .lines()
        .map(|line| line.ok().unwrap().trim().to_string())
        .collect();
    Ok(res)
}
