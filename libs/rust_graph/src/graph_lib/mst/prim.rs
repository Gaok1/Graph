// Prim's algorithm for finding the minimum spanning tree of a graph

use std::collections::HashSet;

use crate::{
    graph_lib::edge::{self, Edge},
    DiGraph,
};

pub fn mst_from_graph(graph: &DiGraph) -> DiGraph {
    println!("Iniciando o cálculo da árvore geradora mínima (MST).");
    
    let mut mst = DiGraph::new();
    let origin = 0;
    println!("Vértice de origem selecionado: {}", origin);
    
    // Holds the vertices that are in the minimum spanning tree
    let mut v_in_mst: HashSet<i32> = HashSet::new();
    v_in_mst.insert(origin);
    println!("Vértices atualmente na MST: {:?}", v_in_mst);
    
    // Holds the edges that are in the mst
    let mut edges_in_mst: HashSet<Edge> = HashSet::new();
    println!("Inicializando o conjunto de arestas na MST.");

    let total_vertices = graph.vertices_length();
    println!("Número total de vértices no grafo: {}", total_vertices);

    while v_in_mst.len() < total_vertices {
        println!(
            "Iteração do loop principal: {} vértices na MST atualmente.",
            v_in_mst.len()
        );
        
        let mut edges_to_add = Vec::new();
        for &v in v_in_mst.iter() {
            println!("Processando vértice: {}", v);
            
            // Odeio thread safety, nem sei por que inventei de fazer essa porra
            let vertice = graph.get_vertice_arc_read(v);
            println!("Obtido arco de leitura para o vértice: {}", v);

            let (forward_edges, back_edges) = vertice.get_all_edges_tuple();
            println!(
                "Vértice {} tem {} arestas direcionadas para frente e {} para trás.",
                v,
                forward_edges.len(),
                back_edges.len()
            );

            for e in forward_edges {
                if !edges_in_mst.contains(&e) && !v_in_mst.contains(&e.destiny_key()) {
                    println!(
                        "Aresta candidata para adição: {:?} (Peso: {})",
                        e,
                        e.weight()
                    );
                    edges_to_add.push(e.clone());
                } else {
                    println!(
                        "Aresta ignorada (já na MST ou destino já está na MST): {:?}",
                        e
                    );
                }
            }
            for e in back_edges {
                if !edges_in_mst.contains(&e) && !v_in_mst.contains(&e.origin_key()) {
                    println!(
                        "Aresta candidata para adição: {:?} (Peso: {})",
                        e,
                        e.weight()
                    );
                    edges_to_add.push(e.clone());
                } else {
                    println!(
                        "Aresta ignorada (já na MST ou origem já está na MST): {:?}",
                        e
                    );
                }
            }
        }

        if edges_to_add.is_empty() {
            println!("Nenhuma aresta candidata encontrada. Possível grafo desconectado.");
            break;
        }

        println!("Total de arestas candidatas nesta iteração: {}", edges_to_add.len());
        let min_edge = get_minimum_edge(&edges_to_add);
        println!("Aresta com menor peso selecionada: {:?} (Peso: {})", min_edge, min_edge.weight());

        edges_in_mst.insert(min_edge.clone());
        mst.add_edge(min_edge.clone());
        println!("Aresta adicionada à MST. Total de arestas na MST agora: {}", edges_in_mst.len());

        // Adiciona o novo vértice à MST
        let new_vertex = if v_in_mst.contains(&min_edge.origin_key()) {
            min_edge.destiny_key()
        } else {
            min_edge.origin_key()
        };
        if v_in_mst.insert(new_vertex) {
            println!("Novo vértice adicionado à MST: {}", new_vertex);
        } else {
            println!("Vértice {} já estava na MST.", new_vertex);
        }

        println!("Estado atual dos vértices na MST: {:?}", v_in_mst);
    }

    println!("Construção da MST concluída.");
    println!("Total de vértices na MST: {}", mst.vertices_length());
    println!("Total de arestas na MST: {}", mst.edges_length());

    mst
}

pub fn get_minimum_edge(edges: &Vec<Edge>) -> Edge {
    println!("Buscando a aresta com o menor peso dentre {} arestas candidatas.", edges.len());
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
    println!("Aresta mínima encontrada: {:?} (Peso: {})", min, min.weight());
    min.clone()
}
