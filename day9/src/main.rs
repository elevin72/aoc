use array2d::Array2D;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    println!("{}", star17());
    println!("{}", star18());
}

fn star17() -> u32 {
    let height_map = get_input("./input").unwrap();
    let mut total = 0;
    for (i, rows) in height_map.rows_iter().enumerate() {
        for (j, _) in rows.enumerate() {
            if let Some(lowest) = is_lowest(i, j, &height_map) {
                total += lowest;
            }
        }
    }
    total
}

fn star18() -> u32 {
    let heights = get_input("./input").unwrap();
    let mut visited = Array2D::filled_with(false, heights.num_rows(), heights.num_columns());
    let mut mins = Vec::<(usize, usize)>::new();
    let mut basins = Vec::<u32>::new();
    for (i, rows) in heights.rows_iter().enumerate() {
        for (j, _) in rows.enumerate() {
            if let Some(_) = is_lowest(i, j, &heights) {
                mins.push((i, j))
            }
        }
    }
    for (row, col) in mins {
        basins.push(traverse_basin(row, col, &heights, &mut visited));
    }
    basins.sort();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

fn traverse_basin(
    row: usize,
    col: usize,
    heights: &Array2D<u8>,
    visited: &mut Array2D<bool>,
) -> u32 {
    let val = heights.get(row, col).unwrap();

    if val < &9 {
        let mut total = 1;
        visited.set(row, col, true).unwrap();
        if row > 0 {
            let above = heights.get(row - 1, col).unwrap();
            if above > val && !*visited.get(row - 1, col).unwrap() {
                total += traverse_basin(row - 1, col, heights, visited);
            }
        }
        if col > 0 {
            let left = heights.get(row, col - 1).unwrap();
            if left > val && !*visited.get(row, col - 1).unwrap() {
                total += traverse_basin(row, col - 1, heights, visited);
            }
        }
        if row < heights.num_rows() - 1 {
            let below = heights.get(row + 1, col).unwrap();
            if below > val && !*visited.get(row + 1, col).unwrap() {
                total += traverse_basin(row + 1, col, heights, visited);
            }
        }
        if col < heights.num_columns() - 1 {
            let right = heights.get(row, col + 1).unwrap();
            if right > val && !*visited.get(row, col + 1).unwrap() {
                total += traverse_basin(row, col + 1, heights, visited);
            }
        }
        total
    } else {
        0
    }
}

fn is_lowest(row: usize, col: usize, heights: &Array2D<u8>) -> Option<u32> {
    let val = heights.get(row, col).unwrap();
    let mut min = u8::MAX;
    if row > 0 {
        min = u8::min(min, *heights.get(row - 1, col).unwrap());
    }
    if col > 0 {
        min = u8::min(min, *heights.get(row, col - 1).unwrap());
    }
    if row < heights.column_len() - 1 {
        min = u8::min(min, *heights.get(row + 1, col).unwrap());
    }
    if col < heights.row_len() - 1 {
        min = u8::min(min, *heights.get(row, col + 1).unwrap());
    }
    match val < &min {
        true => Some((val + 1).into()),
        false => None,
    }
}

fn get_input(path: &str) -> Result<Array2D<u8>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| -> Vec<u8> { line.ok().unwrap().chars().map(|c| c as u8 - b'0').collect() })
        .collect();
    Ok(Array2D::from_rows(&res))
}
