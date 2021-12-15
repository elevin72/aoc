use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

fn main() {
    println!("{}", star29());
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // row, col

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    row: usize,
    col: usize,
    f: u32,
    g: u32,
    h: u32,
    graph: &'a Vec<Vec<u32>>,
}

impl<'a> State<'a> {
    fn new(row: usize, col: usize, g: u32, graph: &Vec<Vec<u32>>) -> State {
        let h = State::manhattan_distance(row, col, graph);
        let f = g + h;
        State {
            row,
            col,
            f,
            g,
            h,
            graph,
        }
    }
    // fn next_states(&self) -> Vec<State> {
    //     let mut next = vec![];
    //     for (dy, dx) in DIRS {
    //         let new_y = dy + self.row as i32;
    //         let new_x = dx + self.col as i32;
    //         let len = self.graph.len() as i32;
    //         if new_y >= 0 && new_y < len && new_x >= 0 && new_x < len {
    //             let new_cost = self.g + self.graph[new_y as usize][new_x as usize];
    //             next.push(State {
    //                 row: new_y as usize,
    //                 col: new_x as usize,
    //                 g: new_cost,
    //                 graph: self.graph,
    //             });
    //         }
    //     }
    //     next
    // }

    fn manhattan_distance(row: usize, col: usize, graph: &Vec<Vec<u32>>) -> u32 {
        ((graph.len() - row) + (graph[0].len() - col)) as u32
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.g + State::manhattan_distance(self.row, self.col, self.graph);
        let b = other.g + State::manhattan_distance(other.row, other.col, other.graph);
        b.cmp(&a)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl<'a> fmt::Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "row: {}, col: {}, cost: {}, heuristic: {}",
            self.row,
            self.col,
            self.g,
            State::manhattan_distance(self.row, self.col, self.graph)
        )
    }
}

fn next_states<'a>(current: State<'a>, graph: &'a Vec<Vec<u32>>) -> Vec<State<'a>> {
    let mut next = vec![];
    for (dy, dx) in DIRS {
        let new_y = dy + current.row as i32;
        let new_x = dx + current.col as i32;
        let len = current.graph.len() as i32;
        if new_y >= 0 && new_y < len && new_x >= 0 && new_x < len {
            let new_cost = current.g + graph[new_y as usize][new_x as usize];
            next.push(State::new(new_y as usize, new_x as usize, new_cost, graph));
        }
    }
    next
}

fn a_star((start_y, start_x): (usize, usize), graph: &Vec<Vec<u32>>) -> u32 {
    let end_y = graph.len() - 1;
    let end_x = graph[0].len() - 1;

    let mut frontier = BinaryHeap::<State>::new();
    let mut visited = HashSet::new();

    let mut current_state = State::new(start_y, start_x, 0, graph);

    frontier.push(current_state);

    while !frontier.is_empty() {
        current_state = frontier.pop().unwrap();
        visited.insert(current_state);

        if (current_state.row, current_state.col) == (end_y, end_x) {
            return current_state.g;
        }

        println!("\ncurrent state: ({})", current_state);

        for next_state in next_states(current_state, graph) {
            if let Some(n) = frontier
                .iter()
                .find(|s| s.row == next_state.row && s.col == next_state.col)
            {
                if next_state.g < n.g {
                    frontier = frontier
                        .into_iter()
                        .filter(|s| !(s.row == next_state.row && s.col == next_state.col))
                        .collect();
                    frontier.push(next_state);
                }
            } else {
                frontier.push(next_state);
            }
        }
    }
    panic!("damnit")
}

fn star29() -> u32 {
    let test_map: Vec<Vec<u32>> = Vec::from([
        vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
        vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
        vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
        vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
        vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
        vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
        vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
        vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
        vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
        vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
    ]);

    a_star((0, 0), &test_map)
}
