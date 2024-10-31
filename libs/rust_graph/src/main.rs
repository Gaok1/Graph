mod graph_lib;
mod tools;

use std::{char::TryFromCharError, time::Duration};

use graph_lib::{
    edge::Edge, flux::ford_fulkerson::max_flux, graph::DiGraph, minPath::floyd_warshall,
    view::GraphPainter,
};

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
    let graph = DiGraph::from_random(5, Some(9), true, false);
    let mut painter = GraphPainter::from_digraph(&graph);
    let min_path = floyd_warshall::MinPathTable::from_digraph(&graph);
    //println!("{:?}", min_path.to_table());
}