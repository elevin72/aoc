use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

fn main() {
    println!("{}", star29());
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // row, col

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    row: usize,
    col: usize,
    cost_from_start: u32,
    graph: &'a Vec<Vec<u32>>,
}

impl<'a> State<'a> {
    fn next_states(&self) -> Vec<State> {
        let mut next = vec![];
        for (dy, dx) in DIRS {
            let new_y = dy + self.row as i32;
            let new_x = dx + self.col as i32;
            let len = self.graph.len() as i32;
            if new_y >= 0 && new_y < len && new_x >= 0 && new_x < len {
                let new_cost = self.cost_from_start + self.graph[new_y as usize][new_x as usize];
                next.push(State {
                    row: new_y as usize,
                    col: new_x as usize,
                    cost_from_start: new_cost,
                    graph: self.graph,
                });
            }
        }
        next
    }

    fn manhattan_distance(&self) -> u32 {
        ((self.graph.len() - self.row) + (self.graph[0].len() - self.col)) as u32
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.cost_from_start + self.manhattan_distance();
        let b = other.cost_from_start + other.manhattan_distance();
        a.cmp(&b)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> fmt::Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(row: {}, col: {}, cost: {})",
            self.row, self.col, self.cost_from_start
        )
    }
}

fn a_star((start_y, start_x): (usize, usize), graph: &Vec<Vec<u32>>) -> u32 {
    let end_y = graph.len();
    let end_x = graph[0].len();

    let mut frontier = BinaryHeap::<State>::new();

    let mut current_state = State {
        row: start_y,
        col: start_x,
        cost_from_start: 0,
        graph,
    };

    frontier.push(current_state);

    while (current_state.row, current_state.col) != (end_y, end_x) {
        println!("{}", current_state);
        for next_state in current_state.next_states() {
            frontier.push(next_state);
        }
        current_state = frontier.pop().unwrap()
    }

    current_state.cost_from_start
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
