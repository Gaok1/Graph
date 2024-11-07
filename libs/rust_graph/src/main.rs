mod graph_lib;
mod tools;

use std::{char::TryFromCharError, time::Duration};

use graph_lib::{
    edge::Edge, flux::ford_fulkerson::max_flux, graph::DiGraph, minPath::floyd_warshall,
    view::GraphPainter,
};
use tools::inifinity::Infinity;

/*

(1 → 2): 3
(1 → 3): 8
(1 → 5): -4
(2 → 4): 1
(2 → 5): 7
(3 → 2): 4
(4 → 1): 2
(4 → 3): -5
(5 → 4): 6

*/
pub fn main() {
    let graph = DiGraph::from_random(30, Some(100), false, false);
    let min_path = floyd_warshall::MinPathTable::from_digraph(&graph);
    let vertices = graph.get_vertice_key_array();
    let mut vertice_center = -1;
    let mut cust_min = Infinity::Infinite;
    for v in vertices.iter() {
        let mut cust_sum = Infinity::Number(0);
        for w in vertices.iter() {
            cust_sum = cust_sum + *min_path.get_cost((*v, *w)).unwrap();
        }
        println!("cust_sum = {cust_sum}");
        if cust_sum < cust_min {
            cust_min = cust_sum;
            vertice_center = *v;
        }
    }
    println!("the center vertice is {vertice_center}");
    let mut painter = GraphPainter::from_digraph(&graph);
    painter.update_vertice_color(vertice_center, graph_lib::view::Color::Green);
    GraphPainter::draw(&graph, "grafo", "");
    painter.to_png("grafo", "");
}
