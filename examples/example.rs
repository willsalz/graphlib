extern crate graphlib;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let mut g = graphlib::Graph::new();
    g.add_edge(0, 1, 1);
    g.add_edge(0, 2, 1);
    g.add_edge(1, 2, 1);
    g.add_edge(2, 3, 2);
    g.add_edge(1, 3, 1);
    g.add_edge(0, 3, 3);
    warn!("Graph: {:?}", g);
    warn!("Shortest path from 0 -> 3? {:?}", g.shortest_path(0, 3));
}
