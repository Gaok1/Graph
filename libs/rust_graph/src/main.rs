use core::borrow;
use std::{fs::File, i32, io::Write, process::Command};

use graph::*;
use graph_lib::*;
use kosaraju::Kosaraju;
use minPath::{bellman, dijkstra::{self, Dijkstra}};
use tools::{heap::HeapMin, inifinity::Infinity};
use view::{Color, GraphPainter};



mod graph_lib;
mod tools;


use Infinity::{Infinite, Number};
fn main() {
    let graph = DiGraph::from_random(200, Some(300), true, false);

    let mut draw = GraphPainter::from_digraph(&graph);
    draw.to_dot_png("graph");
    let data = Dijkstra::shortest_path(&graph, 10);
    
    for (w, v) in data.pred() {
        draw.update_edge_color(*v, *w, Color::Red);
    }
    draw.remove_edges_by_color(Color::Black);
    draw.to_dot_png("djikstra");    
    
}   
