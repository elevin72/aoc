use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    star1();
    star2();

}


fn star1() {
    let mut count = -1;
    let mut prev = 0;
     // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i32>().unwrap();
                if depth > prev {
                    count += 1
                }
                prev = depth;
            }
        }
    println!("{}", count)
    }
}

fn star2() -> io::Result<i32> {
    let mut count = -1;
    if let Ok(mut lines) = read_lines("./input") {
        let mut cur = lines.next().unwrap()?.parse::<i32>().unwrap();
        let mut next = lines.next().unwrap()?.parse::<i32>().unwrap();
        let mut next_next;
        let mut prev_total = 0;
        for line in lines {
            if let Ok(depth) = line {
                next_next = depth.parse::<i32>().unwrap();
                let total = cur + next + next_next;
                if total > prev_total {
                    count += 1
                }
                prev_total = total;
                cur = next;
                next = next_next;

            }
        }
    println!("{}", count)
    }
    let ret: Result<i32, std::io::Error> = Ok(0);
    ret

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
