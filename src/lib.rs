use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[macro_use]
extern crate log;

type NodeId = u32;
type Cost = u32;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<NodeId, HashMap<NodeId, Cost>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Path {
    path: VecDeque<NodeId>,
    cost: Cost,
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    pub fn new() -> Path {
        Path {
            // Initialize empty path
            path: VecDeque::new(),

            // Initialize to 'infinity'
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
        debug!(
            "Added edge from {:?} -> {:?} with cost {:?}",
            from,
            to,
            cost
        );
    }

    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Path {
        // Handle to shortest Path
        let mut shortest_path = Path::new();

        // Global Path costs from 'from' to 'to'
        let mut costs: HashMap<NodeId, Cost> = HashMap::new();

        // To Visit
        let mut q: BinaryHeap<Path> = BinaryHeap::new();

        // Start node
        q.push({
            let mut path = Path::new();
            path.path.push_back(from);
            path.cost = 0;
            path
        });

        // Explore
        while let Some(current_path) = q.pop() {
            info!("Current Path: {:?}", current_path);
            info!("Rest of queue: {:?}", q);

            let current_node: &NodeId = current_path.path.back().unwrap();
            info!("Current node: {:?}", current_node);

            if *current_node == to && current_path.cost < shortest_path.cost {
                shortest_path = current_path.clone()
            }

            let neighbors: &HashMap<NodeId, Cost> = match self.edges.get(&current_node) {
                Some(neighbors) => neighbors,
                None => continue,
            };
            info!("Neighbors: {:?}", neighbors);

            // Explore Neighbors
            for (&neighbor, &edge_cost) in neighbors.iter() {

                // Explore Non-0 Edges
                if edge_cost > 0 {

                    // Get *total* cost of visiting neighbor
                    let proposed_cost: Cost = current_path.cost + edge_cost;

                    match costs.get(&neighbor) {
                        // We've already visited the neighbor, and it's faster!
                        Some(&existing_cost) if proposed_cost > existing_cost => continue,

                        // We've already visited the neighbor, and we're faster!
                        Some(_) | None => {
                            // Update cost to neighbor
                            costs.insert(neighbor, proposed_cost);

                            q.push({
                                let mut path = current_path.clone();
                                path.path.push_back(neighbor);
                                path.cost = proposed_cost;
                                info!("New: {:?}", path);
                                path
                            });
                        }
                    }
                }
            }

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
                for (&neighbor, &cost) in neighbors.iter() {

                    // Explore Non-0 Edges
                    if cost > 0 {
                        q.push_back(neighbor);
                    }
                }
            }
        }

        false
    }
}
