use array2d::Array2D;
use std::cmp;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star9());
    println!("{}", star10());
}

struct Line {
    a: Point,
    b: Point,
}

struct Point {
    x: i32,
    y: i32,
}

struct Diagram {
    d: Array2D<i32>,
}

impl Diagram {
    fn new(size: usize) -> Diagram {
        Diagram {
            d: Array2D::filled_with(0, size, size),
        }
    }

    fn increment(&mut self, y: usize, x: usize) -> Result<(), array2d::Error> {
        let element = *self.d.get_mut(y, x).unwrap();
        self.d.set(y, x, element + 1)
    }
}

fn star9() -> i32 {
    let vents = get_input("./input").unwrap();
    let diagram_size: usize = vents
        .iter()
        .map(|line| -> i32 {
            let a: i32 = cmp::max(line.a.x, line.a.y);
            let b: i32 = std::cmp::max(line.b.x, line.b.y);
            std::cmp::max(a, b) + 1
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let mut diagram = Diagram::new(diagram_size);
    for line in vents {
        if line.a.x == line.b.x {
            let x = line.a.x.try_into().unwrap();
            let smaller = std::cmp::min(line.a.y, line.b.y);
            let bigger = std::cmp::max(line.a.y, line.b.y);
            for i in smaller..bigger + 1 {
                let y = i.try_into().unwrap();
                diagram.increment(y, x).unwrap();
            }
        } else if line.a.y == line.b.y {
            let y = line.a.y.try_into().unwrap();
            let smaller = std::cmp::min(line.a.x, line.b.x);
            let bigger = std::cmp::max(line.a.x, line.b.x);
            for i in smaller..bigger + 1 {
                let x = i.try_into().unwrap();
                diagram.increment(y, x).unwrap();
            }
        }
    }
    diagram
        .d
        .rows_iter()
        .flatten()
        .filter(|num| **num >= 2)
        .count()
        .try_into()
        .unwrap()
}

fn star10() -> i32 {
    let vents = get_input("./input").unwrap();
    let diagram_size: usize = vents
        .iter()
        .map(|line| -> i32 {
            let a: i32 = cmp::max(line.a.x, line.a.y);
            let b: i32 = std::cmp::max(line.b.x, line.b.y);
            std::cmp::max(a, b) + 1
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let mut diagram = Diagram::new(diagram_size);
    for line in vents {
        if i32::abs(line.a.x - line.b.x) == i32::abs(line.a.y - line.b.y) {
            let change_x = get_slope(line.a.x < line.b.x);
            let change_y = get_slope(line.a.y < line.b.y);
            let mut x = line.a.x.try_into().unwrap();
            let mut y = line.a.y.try_into().unwrap();
            let diff = i32::abs(line.a.x - line.b.x);
            for _ in 0..diff + 1 {
                diagram.increment(x, y).unwrap();
                change_x(&mut x);
                change_y(&mut y);
            }
        } else if line.a.x == line.b.x {
            let x = line.a.x.try_into().unwrap();
            let smaller = std::cmp::min(line.a.y, line.b.y);
            let bigger = std::cmp::max(line.a.y, line.b.y);
            for i in smaller..bigger + 1 {
                let y = i.try_into().unwrap();
                diagram.increment(y, x).unwrap();
            }
        } else if line.a.y == line.b.y {
            let y = line.a.y.try_into().unwrap();
            let smaller = std::cmp::min(line.a.x, line.b.x);
            let bigger = std::cmp::max(line.a.x, line.b.x);
            for i in smaller..bigger + 1 {
                let x = i.try_into().unwrap();
                diagram.increment(y, x).unwrap();
            }
        }
    }
    diagram
        .d
        .rows_iter()
        .flatten()
        .filter(|num| **num >= 2)
        .count()
        .try_into()
        .unwrap()
}

fn parse_point(point: &str) -> Point {
    let list: Vec<&str> = point.split(",").collect();
    let x = list[0].parse::<i32>().unwrap();
    let y = list[1].parse::<i32>().unwrap();
    Point { x, y }
}

fn get_slope(pos: bool) -> fn(&mut usize) -> &usize {
    match pos {
        true => |num| {
            *num += 1;
            num
        },
        false =>|num| {
            if *num > 0 {
                *num -= 1;
                return num;
            }
            &0
        }
    }
}

fn get_input(path: &str) -> Result<Vec<Line>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|item| item.ok().unwrap());
    let list: Vec<Line> = lines
        .map(|line| {
            let segment: Vec<&str> = line.split(" -> ").collect(); // segment = ["x1,y1", "x2,y2"]
            let a = parse_point(segment[0]);
            let b = parse_point(segment[1]);
            Line { a, b }
        })
        .collect();
    Ok(list)
}
