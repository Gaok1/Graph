mod graph_lib {
    pub mod graph;
}
use graph_lib::graph::DiGraph;

fn main() {
    let graph = DiGraph::from_file("graphBig.txt");

    println!(
        "grafo com {} vertices e {} arestas",
        graph.get_vertices_length(),
        graph.get_edges_lenght()
    );

    let vertice_key = 1;
    let sucessors = graph.get_sucessor(vertice_key);
    let predecessor = graph.get_predecessor(vertice_key);
    println!("sucessores do vertice {}: {:?}", vertice_key, sucessors);
    println!("predecessores do vertice {}: {:?}", vertice_key, predecessor);
}
