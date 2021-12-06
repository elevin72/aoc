use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star11());
}

struct Fish {
    timer: i32,
    start: i32,
}

impl Fish {
    fn new(timer: i32, start: i32) -> Fish {
        Fish {timer, start}
    }
    
    fn update(&mut self) -> Option<Fish> {
        match self.timer > 0 {
           true => {
               self.timer -= 0;
               None
           },
           false => {
               self.timer = self.start;
               Some(Fish {timer: self.start + 2, start: self.start + 2})
           },
        }
    }
}

fn star11() -> i32 {
    let fish = get_input("./test").unwrap();
    let mut fish_count = vec![0; 80];
    for i in 0..80 {
        fish_count[i] = fish
            .iter()
            .filter(|fish| **fish == i.try_into().unwrap())
            .count();
    }
    for c in fish_count {
        println!("{}", c);
    }
    for c in fish_count

    0
}

fn get_input(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let fish: Vec<i32> = reader
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

