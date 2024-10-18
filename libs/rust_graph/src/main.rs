mod graph_lib;
mod tools;

use std::{collections::HashMap, path, sync::TryLockError};

use comfy_table::{Cell, Color, ContentArrangement, Row, Table};
use edge::Edge;
use flux::ford_fulkerson;
use graph::*;
use graph_lib::*;
use minPath::floyd_warshall::{self, MinPath};
use text_io::scan;
use tools::inifinity::Infinity;
use view::GraphPainter;

#[allow(unused)]
fn main() {
    let sizes = vec![100, 500, 1_000, 2_000];

    let mut repeticoes: Vec<u32> = vec![];

    let mut graph;

    let mut v_len: u32 = 10;

    let mut s = v_len - 1;
    let mut t = 0;

    graph = DiGraph::from_random(v_len, Some(18), false, false);
    let mut painter = GraphPainter::from_digraph(&graph);
    painter.to_png("grafo", "grafo");
    let now = std::time::Instant::now();
    let mut flux = ford_fulkerson::max_flux(&graph, s as i32, t as i32);
    let used_edges = flux.0.get_used_edges();

    let mut caminhos_disjuntos =
        DiGraph::from_edges(used_edges.iter().map(|e| e.clone().0).collect());
    let mut path_num = 0;

    for e in caminhos_disjuntos.all_edges() {
        painter.update_edge_color(e.origin_key(), e.destiny_key(), view::Color::Green);
    }

    painter.to_png("caminho disjuntos graf", "caminho escolhido");

    // loop {
    //     let mut painter = GraphPainter::from_digraph(&caminhos_disjuntos);
    //     let path = caminhos_disjuntos.path_between(s as i32, t as i32);

    //     let Some(mut path) = path else {
    //         break;
    //     };
    //     for edge in &path {
    //         painter.update_edge_color(edge.origin_key(), edge.destiny_key(), view::Color::Green);
    //     }
    //     path_num += 1;
    //     painter.to_png("caminho disjuntos", "caminho escolhido");
    //     for edge in path {
    //         caminhos_disjuntos.remove_edge(edge);
    //     }
    //     let a: String;
    //     scan!("{}", a);
    // }

    println!("Média: {} mls", repeticoes.iter().sum::<u32>() / 3);
}
