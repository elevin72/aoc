use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star29());
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    row: usize,
    col: usize,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self).reverse())
    }
}

fn dijsktra(graph: &Vec<Vec<u32>>) -> u32 {
    let len = graph.len();

    let mut q = BinaryHeap::new();
    let mut dist = vec![vec![u32::MAX - 200; len]; len];

    dist[0][0] = 0;

    q.push(Node {
        row: 0,
        col: 0,
        cost: 0,
    });

    while !q.is_empty() {
        let u = q.pop().unwrap();
        if u.row == len - 1 && u.col == len -1 {
            return u.cost;
        }

        if u.cost > dist[u.row][u.col] {
            continue;
        }

        for (row, col) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let vr = row as i32 + u.row as i32;
            let vc = col as i32 + u.col as i32;
            if vr >= 0 && vr < len as i32 && vc >= 0 && vc < len as i32 {
                let cost = dist[u.row][u.col] + graph[vr as usize][vc as usize];
                if cost < dist[vr as usize][vc as usize] {
                    q.push(Node {row: vr as usize, col: vc as usize, cost});
                    dist[vr as usize][vc as usize] = cost;
                }
            }
        }
    }
    panic!("damn")
}

fn parse(path: &str) -> Result<Vec<Vec<u32>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|l| l.ok().unwrap())
        .map(|l| -> Vec<u32> { l.chars().map(|c| c.to_digit(10).unwrap()).collect() })
        .collect())
}

fn expand(graph: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let len = graph.len() * 5;
    let mut ret = vec![vec![0; len]; len];
    for i in 0..graph.len() {
        for j in 0..graph.len() {
            for mult_i in 0..5 {
                for mult_j in 0..5 {
                    let mut value = graph[i][j] + mult_i as u32 + mult_j as u32;
                    if value > 9 {
                        value -= 9;
                    }
                    ret[i + (mult_i * graph.len())][j + (mult_j * graph.len())] = value;
                }
            }
        }
    }
    ret
}

fn star29() -> u32 {
    let test_map: Vec<Vec<u32>> = expand(&parse("./input").unwrap());
    // let test_map: Vec<Vec<u32>> = expand(&Vec::from([
    //     vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
    //     vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
    //     vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
    //     vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
    //     vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
    //     vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
    //     vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
    //     vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
    //     vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
    //     vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
    // ]));
    dijsktra(&test_map)
}
