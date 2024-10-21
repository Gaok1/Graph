mod graph_lib;
mod tools;

use std::{char::TryFromCharError, time::Duration};

use graph_lib::{
    graph::*,
    search::kosaraju,
    view::{self, GraphPainter},
};

pub fn main() {
    let random = DiGraph::from_random(5, Some(6), true, false);

    let mut dfs_struct = random.dfs_search(1);
    let conex = kosaraju::Kosaraju::conex_components(&random);
    
    GraphPainter::draw(&random, "random_graph", "");

    let mut painter = GraphPainter::from_digraph(&random);
    let mut colors = view::Color::iterator();
    let mut ink = colors.next().unwrap();

    for edge in random.all_edges() {
        if !dfs_struct.is_aresta_marked(edge.id() as i32) {
            println!("aresta nao classificada");
        }
    }

    for tree in conex.clone_components() {
        let tree = tree.try_borrow().unwrap();
        for v in tree.get_vertice_key_array() {
            painter.update_vertice_color(v, ink.clone());
        }
        ink = colors.next().unwrap();
    }

    painter.to_png("trees", "");
}
