use std::collections::{HashMap, HashSet, VecDeque};

type NodeId = u32;
type Cost = u32;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<NodeId, HashMap<NodeId, Cost>>,
}

#[derive(Debug)]
pub struct Path {
    path: Vec<NodeId>,
    cost: Cost,
}

impl Path {
    pub fn new() -> Path {
        Path {
            path: Vec::new(),
            cost: std::u32::MAX,
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edges: HashMap::new() }
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, cost: Cost) {
        self.edges
            .entry(from)
            .or_insert(HashMap::new())
            .entry(to)
            .or_insert(cost);
    }

    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Path {
        let mut shortest_path = Path::new();
        let mut costs: HashMap<NodeId, Cost> = HashMap::new();
        let mut q: VecDeque<NodeId> = VecDeque::new();
        q.push_back(from);
        while !q.is_empty() {
            let current_node = q.pop_front();
        }
        shortest_path
    }

    pub fn path_exists(&self, from: NodeId, to: NodeId) -> bool {
        // Visited
        let mut visited: HashSet<NodeId> = HashSet::new();

        // To Visit
        let mut q: VecDeque<NodeId> = VecDeque::new();

        // Start node
        q.push_back(from);

        // Iterate To Visit
        while !q.is_empty() {
            // Current Node
            // NOTE(wjs): not threadsafe
            let current_node = q.pop_front().unwrap();

            // Terminating Condition
            if current_node == to {
                return true;
            } else {
                // Breadcrumb
                visited.insert(current_node);

                // Scan for Neighbors
                let neighbors = self.edges.get(&current_node).unwrap();
                for (&neighbor, &weight) in neighbors.iter() {

                    // Explore Non-0 Edges
                    if weight > 0 {
                        q.push_back(neighbor);
                    }
                }
            }
        }

        false
    }
}
