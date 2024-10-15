mod graph_lib;
mod tools;

use std::{collections::HashMap, path, sync::TryLockError};

use comfy_table::{Cell, Color, ContentArrangement, Row, Table};
use edge::Edge;
use flux::ford_fulkerson;
use graph::*;
use graph_lib::*;
use minPath::floyd_warshall::MinPath;
use text_io::scan;
use tools::inifinity::Infinity;
use view::GraphPainter;

#[allow(unused)]
fn main() {
    let vertice_len = 10;
    let edge_len = 14;

    let mut graph = DiGraph::from_random(vertice_len, Some(edge_len), true, true);
    let mut painter = GraphPainter::from_digraph(&graph);
    painter.to_png("graph", "graph");

    let min_path = minPath::floyd_warshall::MinPath::from_digraph(&graph);
    let v: i32;
    

    println!("{}", min_path.to_table());
    println!("Grafo criado em graph.png\nSelecione um vértice para pegar seus menores caminhos :");
    scan!("{}", v);
    let paths = min_path.min_paths_from_v(v);
    for edge in paths{
        let (v,w) = (edge.origin_key(), edge.destiny_key());
        painter.update_edge_color(v, w, view::Color::Green);
    }
    
    let title = format!("Menores caminhos de {v}");
    painter.to_png(title.as_str(), title.as_str());

}
