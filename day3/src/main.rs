use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // println!("{}", star5().unwrap());
    println!("{}", star6().unwrap());
}


fn star5() -> io::Result<i32> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let list: Vec<String> = reader
        .lines()
        .filter_map(|item| Some(item.ok().unwrap()))
        .collect();
    let len = list[0].len();
    let mut gamma_string = "".to_string();
    let mut epsilon_string = "".to_string();
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    let two: i32 = 2;
    for i in 0..len {
        let mut zeroes = 0;
        let mut ones = 0;
        for item in list.iter() {
            let test: Vec<char> = item.as_bytes().iter().map(|c| *c as char).collect();
            if len > test.len() {
                continue;
            } 
            match test[i] {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("ono")
            }
        }
        if ones > zeroes {
            gamma_string.push('1');
            epsilon_string.push('0');
            gamma += two.pow((len - 1 - i).try_into().unwrap());
        } else if zeroes > ones {
            gamma_string.push('0');
            epsilon_string.push('1');
            epsilon += two.pow((len - 1 - i).try_into().unwrap());
        } else { // ==
            panic!("ono")
        }
    }

    Ok(gamma * epsilon)
}

fn star6() -> io::Result<i32> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut list: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|item| Some(item.ok()
                                .unwrap()
                                .as_bytes()
                                .iter()
                                .map(|c| *c as char)
                                .collect::<Vec<char>>()))
        .collect();
    let mut other_list = list.clone();
    let mut oxygen: i32 = 0;
    let mut co2: i32 = 0;
    let len = list[0].len();
    for i in 0..len {
        let mut zeroes = 0;
        let mut ones = 0;
        for item in list.iter() {
            if len > item.len() {
                continue;
            } 
            match item[i] {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("ono")
            }
        }

        let most_common = match ones >= zeroes {
            true => '1',
            false => '0'
        };

        list.retain(|item| {
            item[i] == most_common
        });

        if list.len() == 1 {
            let mut total: i32 = 0;
            for (j,c) in (&list[0]).iter().enumerate() {
                let present = c.to_string().parse::<i32>().ok().unwrap();
                let exp = (len - 1 - j).try_into().unwrap();
                total += i32::pow(2, exp) * present;
            }
            oxygen = total;
        }

    }

    for i in 0..len {
        let mut zeroes = 0;
        let mut ones = 0;
        for item in other_list.iter() {
            if len > item.len() {
                continue;
            } 
            match item[i] {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("ono")
            }
        }

        let least_common = match zeroes <= ones {
            true => '0',
            false => '1'
        };

        other_list.retain(|item| {
            item[i] == least_common
        });

        if other_list.len() == 1 {
            let mut total: i32 = 0;
            for (j,c) in (&other_list[0]).iter().enumerate() {
                let present = c.to_string().parse::<i32>().ok().unwrap();
                let exp = (len - 1 - j).try_into().unwrap();
                total += i32::pow(2, exp) * present;
            }
            co2 = total;
        }

    }


    Ok(oxygen * co2)
}
