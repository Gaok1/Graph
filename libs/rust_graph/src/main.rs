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
    let abc = 20;


}
