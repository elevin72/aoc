fn main() {
    println!("Hello, world!");
    println!("{}", star21());
    println!("{}", star22());
}

fn star21() -> u32 {
    let mut octupi: [[u32; 10]; 10] = [
        //test
        // [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        // [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        // [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        // [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        // [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        // [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        // [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        // [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        // [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        // [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        //input
        [4, 7, 8, 1, 6, 2, 3, 8, 8, 8],
        [1, 7, 8, 4, 1, 5, 6, 1, 1, 4],
        [3, 2, 6, 5, 6, 4, 5, 1, 2, 2],
        [4, 3, 7, 1, 5, 5, 1, 4, 1, 4],
        [3, 3, 7, 7, 1, 5, 4, 8, 8, 6],
        [7, 8, 8, 2, 3, 1, 4, 4, 5, 5],
        [6, 4, 2, 1, 3, 4, 8, 6, 8, 1],
        [7, 1, 7, 5, 4, 2, 4, 2, 8, 7],
        [5, 4, 8, 8, 2, 4, 2, 1, 8, 4],
        [2, 4, 4, 8, 5, 6, 8, 2, 6, 1],
    ];
    let mut count = 0;
    for step in 0..100 {
        println!("step: {}", step);
        //increment each octupus
        for i in 0..10 {
            for j in 0..10 {
                octupi[i][j] += 1;
            }
        }

        // propogate flashes
        loop {
            let mut changed = false;
            for i in 0..10 {
                for j in 0..10 {
                    if octupi[i][j] >= 10 {
                        count += 1;
                        octupi[i][j] = 0;
                        for (dy, dx) in [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ] {
                            let new_i = i as i32 + dy;
                            let new_j = j as i32 + dx;
                            if new_i >= 0 && new_i < 10 && new_j >= 0 && new_j < 10 {
                                if octupi[new_i as usize][new_j as usize] != 0 {
                                    octupi[new_i as usize][new_j as usize] += 1;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }
    return count;
}

fn star22() -> u32 {
    let mut octupi: [[u32; 10]; 10] = [
        //input
        [4, 7, 8, 1, 6, 2, 3, 8, 8, 8],
        [1, 7, 8, 4, 1, 5, 6, 1, 1, 4],
        [3, 2, 6, 5, 6, 4, 5, 1, 2, 2],
        [4, 3, 7, 1, 5, 5, 1, 4, 1, 4],
        [3, 3, 7, 7, 1, 5, 4, 8, 8, 6],
        [7, 8, 8, 2, 3, 1, 4, 4, 5, 5],
        [6, 4, 2, 1, 3, 4, 8, 6, 8, 1],
        [7, 1, 7, 5, 4, 2, 4, 2, 8, 7],
        [5, 4, 8, 8, 2, 4, 2, 1, 8, 4],
        [2, 4, 4, 8, 5, 6, 8, 2, 6, 1],
    ];
    let mut step = 0;
    loop {
        println!("step: {}", step);
        //increment each octupus
        for i in 0..10 {
            for j in 0..10 {
                octupi[i][j] += 1;
            }
        }

        // propogate flashes
        loop {
            let mut changed = false;
            for i in 0..10 {
                for j in 0..10 {
                    if octupi[i][j] >= 10 {
                        octupi[i][j] = 0;
                        for (dy, dx) in [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ] {
                            let new_i = i as i32 + dy;
                            let new_j = j as i32 + dx;
                            if new_i >= 0 && new_i < 10 && new_j >= 0 && new_j < 10 {
                                if octupi[new_i as usize][new_j as usize] != 0 {
                                    octupi[new_i as usize][new_j as usize] += 1;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        if octupi.iter().flatten().filter(|o| **o == 0).count() == 100 {
            return step;

        }
        step += 1;
    }
}
