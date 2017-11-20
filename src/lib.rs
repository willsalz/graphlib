use std::collections::{HashMap, HashSet, VecDeque};

type NodeId = u32;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<NodeId, HashMap<NodeId, bool>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edges: HashMap::new() }
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId) {
        self.edges
            .entry(a)
            .or_insert(HashMap::new())
            .entry(b)
            .or_insert(true);
    }

    pub fn path_exists(&self, start: NodeId, end: NodeId) -> bool {
        let mut visited: HashSet<NodeId> = HashSet::new();
        let mut q: VecDeque<NodeId> = VecDeque::new();
        q.push_back(start);
        while !q.is_empty() {
            let node_id = q.pop_front().unwrap();
            if node_id == end {
                return true;
            } else {
                visited.insert(node_id);
                let neighbors = self.edges.get(&node_id);
                for neighbor in neighbors.values() {}
            }
        }

        false
    }
}
