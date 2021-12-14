use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    println!("{}", star27());
    println!("{}", star28());
}

type Count = HashMap<char, u64>;
type Rules = HashMap<(char, char), char>;


fn star27() -> u64 {
    count_elements(10)

}

fn star28() -> u64 {
    count_elements(40)
}

fn count_elements(steps: u32) -> u64 {
    let (template, rules) = parse("./input").unwrap();
    let mut table = HashMap::<((char, char), u32), Count>::new();

    // get all the possible characters in symbols
    let mut symbols = HashSet::new();
    for pair in rules.keys() {
        symbols.insert(pair.0);
        symbols.insert(pair.1);
    }

    let mut initial_count = Count::new();
    for symbol in &symbols {
        initial_count.insert(*symbol, 0);
    }

    // populate table
    for i in 0..=steps {
        for pair in rules.keys() {
            if i == 0 {
                let mut count = initial_count.clone();
                count.insert(pair.0, *count.get(&pair.0).unwrap() + 1);
                count.insert(pair.1, *count.get(&pair.1).unwrap() + 1);
                table.insert((*pair, 0), count);
            } else {
                let gen = rules[pair];
                let mut count0 = table.get(&((pair.0, gen), i - 1)).cloned().unwrap();
                let count1 = table.get(&((gen, pair.1), i - 1)).unwrap();

                for symbol in &symbols {
                    *count0.get_mut(symbol).unwrap() += count1[symbol];
                }
                *count0.get_mut(&gen).unwrap() -= 1;

                table.insert((*pair, i), count0);
            }
        }
    }

    let count: Count = template
        .as_bytes()
        .windows(2)
        .fold(initial_count, |mut acc, pair| {
            let count = &table[&((pair[0] as char, pair[1] as char), steps)];
            for symbol in &symbols {
                let c = acc.get_mut(symbol).unwrap();
                *c += count[symbol];
            }
            *acc.get_mut(&(pair[0] as char)).unwrap() -= 1;
            acc
        });

    let max = count.iter().map(|pair| pair.1).max().unwrap();
    let min = count.iter().map(|pair| pair.1).min().unwrap();

    max - min
}

fn parse(path: &str) -> Result<(String, Rules), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let template: String = lines.next().map(|t| t.ok().unwrap()).unwrap();
    let mut rules = Rules::new();
    for line in lines {
        if let Some((pair, gen)) = line.ok().unwrap().trim().split_once(" -> ") {
            let p1 = pair.chars().nth(0).unwrap();
            let p2 = pair.chars().nth(1).unwrap();
            let g = gen.chars().nth(0).unwrap();
            rules.insert((p1, p2), g);
        }
    }
    Ok((template, rules))
}
