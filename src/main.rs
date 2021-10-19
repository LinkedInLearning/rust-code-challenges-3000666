use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

// derived from the implementation provided at 
// https://doc.rust-lang.org/alloc/collections/binary_heap/index.html

type Node = usize;
type Cost = usize;

struct Graph {
    /* TODO: design data structure */
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        todo!()
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    todo!();
}

fn main() {
    let edge_list = vec![
        (0, 1, 1),
        (1, 2, 1),
        (2, 1, 1),
        (1, 3, 3),
        (2, 3, 1),
        (2, 4, 3),
        (3, 5, 1),
        (4, 5, 1),
        (5, 6, 1),
        (2, 6, 2),
    ];
    
    let g = Graph::from_edge_list(&edge_list);
    if let Some((path, cost)) = shortest_path(&g, 0, 6) {
        println!("{}->{}, {:?} {}", 0, 6, path, cost);
    };
}
