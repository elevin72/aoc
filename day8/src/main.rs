use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", star15());
    println!("{}", star16());
}

// acedgfb  -> 8
// cdfbe    -> 2,5
// gcdfa    -> 2,5
// fbcad    -> 3
// dab      -> 7
// cefabd   -> 6,9
// cdfgeb   -> 6,9
// eafb     -> 4
// cagedb   -> 0
// ab       -> 1

// sides = 8 - cdf = abeg
// cagedb -> 0 since it contains all sides
//
//
fn star15() -> u32 {
    let outputs = get_input("./input").unwrap();
    outputs
        .iter()
        .map(|output| count_unique_nums(&output.1))
        .sum()
}

struct CharSet {
    set: HashSet<char>,
    chars: String,
}

impl CharSet {
    fn new(chars: &String) -> CharSet {
        let set = chars.chars().sorted().collect();
        CharSet {
            set,
            chars: chars.to_string()
        }
    }

    fn union(&self, b: &CharSet) -> CharSet {
        let set = self.set.union(&b.set).cloned().collect();
        let chars = string_from_set(&set);
        CharSet { set, chars }
    }

    fn intersect(&self, b: &CharSet) -> CharSet {
        let set = self.set.intersection(&b.set).cloned().collect();
        let chars = string_from_set(&set);
        CharSet { set, chars }
    }

    fn difference(&self, b: &CharSet) -> CharSet {
        let set = self.set.difference(&b.set).cloned().collect();
        let chars = string_from_set(&set);
        CharSet { set, chars }
    }

    fn insert(&self, c: char) -> CharSet {
        let mut set: HashSet<char> = self.set.clone();
        set.insert(c);
        let chars = string_from_set(&set);
        CharSet { set, chars }
    }

    fn is_subset(&self, b: &CharSet) -> bool {
        self.set.is_subset(&b.set)
    }

    fn invert(&self) -> CharSet {
        CharSet::new(&"abcdefg".to_string()).difference(self)
    }

    fn equals(&self, b: &CharSet) -> bool {
        self.difference(b).set.is_empty() && b.difference(self).set.is_empty()
    }

    fn get_string(&self) -> String {
        self.chars.clone()
    }
}

impl fmt::Display for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chars)
    }
}

fn string_from_set(set: &HashSet<char>) -> String {
    let mut s: String = "".to_string();
    for c in set.iter() {
        s.push(*c);
    }
    s
}


fn star16() -> i32 {
    let inputs = get_input("./input").unwrap();
    let mut total = 0;
    for line in &inputs {
        let input: Vec<String> = line.0.iter().map(|s| s.chars().sorted().collect()).collect();
        let mut sln: HashMap<String, i32> = HashMap::new();
        let one = input.iter().find(|s| s.len() == 2).unwrap();
        let four = input.iter().find(|s| s.len() == 4).unwrap();
        let seven = input.iter().find(|s| s.len() == 3).unwrap();
        let eight = input.iter().find(|s| s.len() == 7).unwrap();

        sln.insert(one.chars().sorted().collect(), 1);
        sln.insert(four.chars().sorted().collect(), 4);
        sln.insert(seven.chars().sorted().collect(), 7);
        sln.insert(eight.chars().sorted().collect(), 8);

        let maybe235: Vec<CharSet> = input
            .iter()
            .filter(|s| s.len() == 5)
            .map(|s| CharSet::new(s))
            .collect();

        let maybe069: Vec<CharSet> = input
            .iter()
            .filter(|s| s.len() == 6)
            .map(|s| CharSet::new(s))
            .collect();

        let one_set = CharSet::new(one);

        let three = maybe235
            .iter()
            .find(|m235| one_set.is_subset(m235))
            .unwrap();
        sln.insert(three.to_string().chars().sorted().collect(), 3);

        let horizontals = Box::leak(maybe235.iter().fold(
            Box::new(CharSet::new(&"abcdefg".to_string())),
            |acc, elem| Box::new(Box::leak(acc).intersect(&elem)),
        ));

        let sides = horizontals.invert();

        let zero = maybe069.iter().find(|m069| sides.is_subset(m069)).unwrap();
        sln.insert(zero.to_string().chars().sorted().collect(), 0);

        let seven_set = CharSet::new(seven);
        let six = input
            .iter()
            .find(|s| s.len() == 6 && !(seven_set.is_subset(&CharSet::new(s))))
            .unwrap();
        sln.insert(six.chars().sorted().collect(), 6);

        let four_set = CharSet::new(four);
        let nine = input
            .iter()
            .find(|s| s.len() == 6 && !(four_set.is_subset(&CharSet::new(s).invert())))
            .unwrap();
        sln.insert(nine.chars().sorted().collect(), 9);

        let nine_set = CharSet::new(nine);
        let eight_set = CharSet::new(eight);
        let two = input
            .iter()
            .find(|s| s.len() == 5 && CharSet::new(s).union(&nine_set).equals(&eight_set))
            .unwrap();
        sln.insert(two.chars().sorted().collect(), 2);

        let six_set = CharSet::new(six);
        let five_set = six_set.intersect(&nine_set);
        sln.insert(five_set.to_string().chars().sorted().collect(), 5);

        for i in sln.iter() {
            println!("{} -> {}", i.0, i.1);
        }

        let output_4dig: Vec<i32> = line
            .1
            .iter()
            .map(|s| {
                let so = s.chars().sorted().collect::<String>();
                sln[&so]
            })
            .collect();

        let mut num = 0;
        for i in 0..4 {
            num += output_4dig[3-i] * i32::pow(10, i.try_into().unwrap())
        }

        total += num;
    }
    total
}

fn count_unique_nums(v: &Vec<String>) -> u32 {
    let a = v
        .iter()
        .map(|s| s.len())
        .filter(|n| is_unique_num(*n as u32))
        .count()
        .try_into()
        .unwrap();
    println!("{}", a);
    a
}

fn is_unique_num(n: u32) -> bool {
    n == 2 || n == 3 || n == 4 || n == 7
}

fn get_input(path: &str) -> Result<Vec<(Vec<String>, Vec<String>)>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let positions: Vec<(Vec<String>, Vec<String>)> = reader
        .lines()
        .map(|item| {
            let item = item.ok().unwrap().to_string();
            let (input, output) = item.split_once(" | ").unwrap();
            let a = input.split(" ").map(|s| s.to_string()).collect();
            let b = output.split(" ").map(|s| s.to_string()).collect();
            (a, b)
        })
        .collect();
    Ok(positions)
}


// ceb bgfdea febgc ec eadcgfb eagbcd fcdebg dcef gafbc egdbf | fdbgec fedbg gdabefc gefbd
// ec = 1
// dcef = 4
// ceb = 7
// eadcgfb = 8
// febgc = 3
// eagbcd = 0
//
//
// horizontals = fgb
// sides = acde

