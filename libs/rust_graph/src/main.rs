mod graph_lib;
mod tools;

use graph::*;
use graph_lib::*;
use minPath::floyd_warshall::MinPath;

fn main() {
    let graph = DiGraph::from_random(5, Some(10), true, true);

    let min_path = MinPath::from_digraph(&graph);
    let painter = view::GraphPainter::from_digraph(&graph);
    painter.to_dot_png("graph");
    let vertices = graph.get_vertice_key_array();
    for v in vertices.iter() {
        for w in vertices.iter() {
            println!(
                "Menor custo de {v} -> {w} = {}",
                min_path.get_cost((*v, *w))
            );
        }
    }
}
