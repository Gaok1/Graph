// Prim's algorithm for finding the minimum spanning tree of a graph

use std::collections::HashSet;

use crate::{
    graph_lib::edge::{self, Edge},
    DiGraph,
};
/// # Algoritmo de Prim
///
/// Gera uma MST a partir de um vértice de um grafo direcionado
///
/// Começa com um conjunto de vértice S = {v} e adiciona arestas mínimas que definem o corte de aresta entre S e G aumentando assim o conjunto S.
///
///
/// No final, a MST não leva em conta a direção das arestas, apenas seu peso.
pub fn mst_from_graph(graph: &DiGraph, v: i32) -> DiGraph {
    let mut mst = DiGraph::new();

    // Holds the vertices that are in the minimum spanning tree
    let mut v_in_mst: HashSet<i32> = HashSet::new();
    v_in_mst.insert(v);

    // Holds the edges that are in the mst
    let mut edges_in_mst: HashSet<Edge> = HashSet::new();

    let total_vertices = graph.vertices_length();

    while v_in_mst.len() < total_vertices {
        let mut edges_to_add = Vec::new();
        for &v in v_in_mst.iter() {
            // Odeio thread safety, nem sei por que inventei de fazer essa porra
            let vertice = graph.get_vertice_arc(v).unwrap();

            let (forward_edges, back_edges) = vertice.get_all_edges_tuple();

            for e in forward_edges {
                if !edges_in_mst.contains(&e) && !v_in_mst.contains(&e.destiny_key()) {
                    edges_to_add.push(e.clone());
                } else {
                }
            }
            for e in back_edges {
                if !edges_in_mst.contains(&e) && !v_in_mst.contains(&e.origin_key()) {
                    edges_to_add.push(e.clone());
                } else {
                }
            }
        }

        if edges_to_add.is_empty() {
            break;
        }

        let min_edge = get_minimum_edge(&edges_to_add);

        edges_in_mst.insert(min_edge.clone());
        mst.add_edge(min_edge.clone());

        // Adiciona o novo vértice à MST
        if v_in_mst.contains(&min_edge.origin_key()) {
            min_edge.destiny_key()
        } else {
            min_edge.origin_key()
        };
    }

    println!("Construção da MST concluída.");
    println!("Total de vértices na MST: {}", mst.vertices_length());
    println!("Total de arestas na MST: {}", mst.edges_length());
    mst
}

pub fn get_minimum_edge(edges: &Vec<Edge>) -> Edge {
    println!(
        "Buscando a aresta com o menor peso dentre {} arestas candidatas.",
        edges.len()
    );
    let mut min = &edges[0];
    for e in edges.iter().skip(1) {
        if e.weight() < min.weight() {
            println!(
                "Aresta {:?} tem peso menor ({}) do que a aresta atual mínima {:?} (Peso: {}).",
                e,
                e.weight(),
                min,
                min.weight()
            );
            min = e;
        }
    }
    println!(
        "Aresta mínima encontrada: {:?} (Peso: {})",
        min,
        min.weight()
    );
    min.clone()
}
