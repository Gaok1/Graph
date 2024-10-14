mod graph_lib;
mod tools;

use comfy_table::{Table, Row, Cell, ContentArrangement, Color};
use edge::Edge;
use flux::ford_fulkerson;
use graph::*;
use graph_lib::*;
use minPath::floyd_warshall::MinPath;
use text_io::scan;
use view::GraphPainter;

fn main() {
    // Cria um grafo direcionado aleatório
    let vertice_len = 14;
    let edge_len = 20;

    let mut graph: Option<DiGraph>  = None ;
    let mut base_antibase: Option<(i32,i32)> = None; 

    while let None = graph {
        let mut g_test = DiGraph::from_random(vertice_len, Some(edge_len), true, false);
        base_antibase = g_test.find_base_antibase();
        if base_antibase.is_some() {
            graph = Some(g_test);
        }
    }
    let graph = graph.unwrap();
    GraphPainter::from_digraph(&graph).to_dot_png("graph", "Graph");

    println!("Digite a base e antibase ou deixe em branco para escolher automaticamente");
    let (base, antibase): (i32, i32);

    scan!("{} {}", base, antibase);
    
   
    let max_flux: ford_fulkerson::FluxMap = ford_fulkerson::max_flux(&graph, base, antibase);
    let mut painter = max_flux.to_png();

    painter.to_dot_png("max_flux",format!("Max Flux from {} to {} = {}", base, antibase, max_flux.get_max_flux()).as_str());
    println!("Max Flux from {} to {} = {}", base, antibase, max_flux.get_max_flux());
}