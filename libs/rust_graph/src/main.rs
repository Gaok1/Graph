use std::{fs::File, io::Write, process::Command};

use graph::*;
use graph_lib::*;

mod graph_lib;
mod tools;

use crate::graph_lib::bellman;

fn main() {
    let graph = DiGraph::from_random(6,Some(12), true);
    graph.to_dot_png("graph");
    let  data = bellman::find_shortest_path(&graph, 0);

    for i in data.pred().iter() {
        println!("{} -> {} w {}", i.1, i.0, data.pot().get(i.0).unwrap());
    }

    
    
}
