use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    println!("{}", star25());
}

fn star25() -> u32 {
    let (mut dots, instructions) = get_input("./input").unwrap();
    for instruction in instructions {
        let mut new_dots = HashSet::new();
        let mut old_dots = HashSet::new();
        match instruction.0 {
            'x' => {
                old_dots = dots
                    .iter()
                    .filter(|d| d.0 < instruction.1)
                    .copied()
                    .collect();
                for dot in dots.iter().filter(|d| d.0 > instruction.1) {
                    let new_x = instruction.1 - (dot.0 - instruction.1);
                    let new_y = dot.1;
                    new_dots.insert((new_x, new_y));
                }
            }
            'y' => {
                old_dots = dots
                    .iter()
                    .filter(|d| d.1 < instruction.1)
                    .copied()
                    .collect();
                for dot in dots.iter().filter(|d| d.1 > instruction.1) {
                    let new_x = dot.0;
                    let new_y = instruction.1 - (dot.1 - instruction.1);
                    new_dots.insert((new_x, new_y));
                }
            }
            _ => panic!("wutwut"),
        }
        dots = new_dots.union(&old_dots).copied().collect();
    }
    let len = dots.len();
    for i in 0..len {
        for j in 0..len {
            match dots.contains(&(j as u32 ,i as u32)) {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!("")
    }
    0
}

fn get_input(path: &str) -> Result<(HashSet<(u32, u32)>, Vec<(char, u32)>), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut instructions: Vec<(char, u32)> = Vec::new();
    for line in reader.lines() {
        if let Some((xs, ys)) = line.as_ref().unwrap().split_once(",") {
            let x: u32 = xs.parse().unwrap();
            let y: u32 = ys.parse().unwrap();
            dots.insert((x, y));
        } else if let Some((dir, line_no_s)) = line.unwrap().split_once("=") {
            let dir = dir.chars().last().unwrap();
            let line_no: u32 = line_no_s.parse().unwrap();
            instructions.push((dir, line_no));
        }
    }
    Ok((dots, instructions))
}
