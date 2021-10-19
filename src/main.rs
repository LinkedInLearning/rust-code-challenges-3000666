use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

// derived from the implementation provided at 
// https://doc.rust-lang.org/alloc/collections/binary_heap/index.html

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

// Creating a priority queue from a `BinaryHeap` requires depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position)) // Necessary to retain consistency between `PartialEq` and `Ord`
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut dist: HashMap<Node, Cost> = g.nodes.iter().map(|&x| (x, usize::MAX)).collect();  
    
    // (0..g.edges.len())
    //     .map(|_| usize::MAX) // use usize::MAX to represent infinity
    //     .collect();

    let mut priority_queue = BinaryHeap::new();

    if let Some(cost) = dist.get_mut(&start){
        // We're at `start`, so allocate that zero cost
        *cost = 0;
    }

    priority_queue.push(Step { cost: 0, position: start, history: vec![] });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Step { cost, position, mut history }) = priority_queue.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            history.push(goal);
            return Some((history, cost)); 
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for &(next_destination, next_cost) in &g.edges[&position] {

            // If so, add it to the frontier and continue
            if next_cost < dist[&next_destination] {
                let mut next = Step {
                    position: next_destination,
                    cost: cost + next_cost, 
                    history: history.clone(),
                };
                next.history.push(position);
                priority_queue.push(next);

                if let Some(old_cost) = dist.get_mut(&next_destination){
                    // We're at `start`, so allocate that zero cost
                    *old_cost = next_cost;
                }
            }
        }
    }

    // Goal not reachable
    None
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
