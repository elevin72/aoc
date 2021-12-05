use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    if let Ok(res) = star3() {
        println!("{}", res);
    }

    println!("{}", star4().unwrap());
}

fn star3() -> io::Result<i32> {
    let (mut horiz, mut depth) = (0, 0);
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|item| Some(item.ok().unwrap()))
        .for_each(|s| {
            let l: Vec<&str> = s.split(" ").collect();
            if let Ok(amount) = l[1].parse::<i32>() {
                // println!("DEBUG {} {}", l[0], amount);
                match l[0] {
                    "forward" => horiz += amount,
                    "up" => depth -= amount,
                    "down" => depth += amount,
                    _ => panic!(),
                }
            }
        });

    Ok(horiz * depth)
}

fn star4() -> io::Result<i32> {
    let (mut horiz, mut depth, mut aim) = (0, 0, 0);
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|item| Some(item.ok().unwrap()))
        .for_each(|s| {
            let l: Vec<&str> = s.split(" ").collect();
            if let Ok(amount) = l[1].parse::<i32>() {
                // println!("DEBUG {} {}", l[0], amount);
                match l[0] {
                    "forward" => {
                        horiz += amount;
                        depth += aim * amount;
                    },
                    "up" => aim -= amount,
                    "down" => aim += amount,
                    _ => panic!(),
                }
            }
        });

    Ok(horiz * depth)
}
