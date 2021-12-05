use array2d::Array2D;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;

fn main() {
    println!("Hello, world!");
    println!("{}", star7());
    println!("{}", star8());
}

fn star7() -> i32 {
    if let Ok((num_list, mut board_list)) = get_input(&"./input".to_string()) {
        for num in num_list {
            // check off num in all boards
            for board in board_list.iter_mut() {
                for i in 0..5 {
                    for j in 0..5 {
                        if board.index((i, j)).0 == num {
                            board.set(i, j, (num, true)).unwrap();
                        }
                    }
                }
            }

            // check if any boards one yet
            for board in &board_list {
                if let Ok(value) = board_is_won(&board) {
                    return value * num;
                }
            }
        }
    }
    -1
}

fn star8() -> i32 {
    if let Ok((num_list, mut board_list)) = get_input(&"./input".to_string()) {
        for num in num_list {
            // check off num in all boards
            for board in board_list.iter_mut() {
                for i in 0..5 {
                    for j in 0..5 {
                        if board.index((i, j)).0 == num {
                            board.set(i, j, (num, true)).unwrap();
                        }
                    }
                }
            }

            if board_list.len() != 1 {
                // get rid of more boards
                board_list.retain(|board| {
                    if let Ok(_) = board_is_won(board) {
                        return false;
                    }
                    true
                });
            } else {
                // find the last board
                if let Ok(value) = board_is_won(&board_list[0]) {
                    return value * num;
                }
            }
        }
    }
    -1
}

fn get_input(path: &String) -> Result<(Vec<i32>, Vec<Array2D<(i32, bool)>>), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|item| item.ok().unwrap());
    let num_list: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.parse().ok().unwrap())
        .collect();
    lines.next();
    let mut board: Array2D<(i32, bool)> = Array2D::filled_with((0, false), 5, 5);
    let mut board_list = Vec::new();

    let mut count = 0;
    for line in lines {
        if count != 5 {
            let line_nums: Vec<i32> = line
                .split_whitespace()
                .map(|item| item.trim().parse::<i32>().ok().unwrap())
                .collect();
            for (j, num) in line_nums.into_iter().enumerate() {
                Some(board.set(count, j, (num, false)));
            }
            count += 1;
        } else {
            board_list.push(board.clone());
            board = Array2D::filled_with((0, false), 5, 5);
            count = 0;
        }
    }
    Ok((num_list, board_list))
}

fn board_is_won(board: &Array2D<(i32, bool)>) -> Result<i32, ()> {
    for i in 0..5 {
        let index: usize = i.try_into().unwrap();
        if board.row_iter(index).all(|item| item.1) || board.column_iter(index).all(|item| item.1) {
            let val = board
                .rows_iter()
                .map(|row| -> i32 { row.filter(|item| !item.1).map(|item| item.0).sum() })
                .sum();
            return Ok(val);
        }
    }
    Err(())
}
