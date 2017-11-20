extern crate graphlib;

fn main() {
    let mut g = graphlib::Graph::new();
    g.add_edge(0, 1, 1);
    g.add_edge(0, 2, 1);
    g.add_edge(1, 2, 1);
    g.add_edge(2, 3, 1);
    println!("Graph: {:?}", g);
    println!("Path from 0 -> 3? {:?}", g.path_exists(0, 3));
    println!("Shortest path from 0 -> 3? {:?}", g.shortest_path(0, 3));
}
