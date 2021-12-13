use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    // println!("{}", star23());
    println!("{}", star24());
}

struct Cave {
    connected_caves: HashSet<String>,
}

impl Cave {
    fn new() -> Cave {
        Cave {
            connected_caves: HashSet::new(),
        }
    }

    fn add_cave(&mut self, name: &str) -> () {
        self.connected_caves.insert(name.to_owned());
    }
}

// fn star23() -> u32 {
//     let input = get_input("./input").unwrap();
//     let mut caves: HashMap<String, Cave> = HashMap::new();
//     for (a, b) in &input {
//         caves.insert(a.to_owned(), Cave::new());
//         caves.insert(b.to_owned(), Cave::new());
//     }

//     for (a, b) in &input {
//         caves.get_mut(a).unwrap().add_cave(b);
//         caves.get_mut(b).unwrap().add_cave(a);
//     }
//     let paths = get_unique_paths(
//         "start",
//         &HashSet::from_iter(["start".to_owned()].iter()),
//         &caves,
//     );
//     // for path in &paths {
//     //     println!("{}", path);
//     // }
//     // paths.len() as u32
//     return paths;
// }

// fn get_unique_paths(
//     start_cave: &str,
//     visited_small_caves: &HashSet<&String>,
//     caves: &HashMap<String, Cave>,
// ) -> u32 {
//     if start_cave == "end" {
//         return 1;
//     }
//     let current_cave = &caves[start_cave];
//     if current_cave
//         .small_caves
//         .iter()
//         .filter(|cave| !visited_small_caves.contains(*cave))
//         .count()
//         == 0
//         && current_cave.large_caves.is_empty()
//     {
//         return 0;
//     }
//     let mut total_paths = 0;
//     for small_cave in current_cave
//         .small_caves
//         .iter()
//         .filter(|cave| !visited_small_caves.contains(*cave))
//     {
//         let mut new_visted_small_caves = visited_small_caves.clone();
//         new_visted_small_caves.insert(small_cave);
//         total_paths += get_unique_paths(&small_cave, &new_visted_small_caves, caves);
//     }
//     for large_cave in current_cave.large_caves.iter() {
//         total_paths += get_unique_paths(&large_cave, visited_small_caves, caves);
//     }
//     return total_paths;
// }

fn star24() -> u32 {
    let input = get_input("./input").unwrap();
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for (a, b) in &input {
        caves.insert(a.to_owned(), Cave::new());
        caves.insert(b.to_owned(), Cave::new());
    }

    for (a, b) in &input {
        caves.get_mut(a).unwrap().add_cave(b);
        caves.get_mut(b).unwrap().add_cave(a);
    }
    let paths = get_unique_paths2(
        "start".to_string(),
        true,
        &mut vec!["start".to_string()],
        &caves,
    );
    return paths;
}

fn get_unique_paths2(
    start_cave: String,
    can_revisit: bool,
    visited: &mut Vec<String>,
    caves: &HashMap<String, Cave>,
) -> u32 {
    if start_cave == "end" {
        return 1;
    }
    let current_cave = &caves[&start_cave];
    let mut total_paths = 0;
    for connected_cave in &current_cave.connected_caves {
        let can_revisit = match (
            connected_cave.as_str(),
            visited.contains(connected_cave),
            connected_cave.chars().all(char::is_lowercase),
            can_revisit,
        ) {
            ("start", _, _, _) | (_, true, true, false) => continue,
            (_, true, true, true) => false,
            _ => can_revisit,
        };
        visited.push(connected_cave.to_string());
        total_paths += get_unique_paths2(connected_cave.to_string(), can_revisit, visited, caves);
        visited.pop();
    }
    return total_paths;
}

fn get_input(path: &str) -> Result<Vec<(String, String)>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: Vec<_> = reader
        .lines()
        .map(|line| line.ok().unwrap())
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (a.to_string(), b.to_string())
        })
        .collect();
    Ok(res)
}
