use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star13());
    println!("{}", star14());
}

fn star13() -> i32 {
    let mut positions: Vec<i32> = get_input("./input").unwrap();
    positions.sort_unstable();
    let total = positions.len();
    let center = positions[total / 2];
    positions.iter().map(|num| i32::abs(num - center)).sum()
}

fn star14() -> i64 {
    let positions: Vec<i32> = get_input("./input").unwrap();
    let max: i32 = *(positions.iter().max().unwrap());
    let min: i32 = *(positions.iter().min().unwrap());
    let mut ret = i64::MAX;
    for i in min..max + 1 {
        let a = f(i, &positions);
        ret = match a < ret {
            true => a,
            false => ret,
        };
    }
    ret
}

fn f(x: i32, a: &Vec<i32>) -> i64 {
    a.iter()
        .map(|y| -> i64 {
            let d = i32::abs(y - x);
            (d * (d + 1)) as i64
        })
        .sum::<i64>()
        / 2
}

fn get_input(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let positions: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    Ok(positions)
}
