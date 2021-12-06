use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star11()); // star12 just with 256 days
}

fn star11() -> u64 {
    let fish = get_input("./input").unwrap();
    let mut counts = vec![0; 9];
    for f in fish {
        let idx: usize = f.try_into().unwrap();
        counts[idx] += 1;
    }
    for _i in 0..256 {
        let mut alt_counts = vec![0; 80];
        for j in (1..9).rev() {
            alt_counts[j - 1] = counts[j];
        }
        alt_counts[6] += counts[0];
        alt_counts[8] += counts[0];
        counts = alt_counts;
        println!("{}: {}", _i, counts.iter().sum::<u64>());
    }
    counts.iter().sum()
}

fn get_input(path: &str) -> Result<Vec<u64>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let fish: Vec<u64> = reader
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    Ok(fish)
}
