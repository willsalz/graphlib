extern crate graphlib;

fn main() {
    let mut g = graphlib::Graph::new();
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    println!("Graph: {:?}", g);
    println!("Path from 0 -> 1? {:?}", g.path_exists(0, 1));
}
