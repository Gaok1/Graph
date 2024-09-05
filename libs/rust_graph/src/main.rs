mod graph_lib {
    pub mod graph;
    pub mod busca;
}
use graph_lib::graph::DiGraph;

fn main() {
    let graph = DiGraph::from_file("graphBig.txt");

    println!(
        "grafo com {} vertices e {} arestas",
        graph.get_vertices_length(),
        graph.get_edges_lenght()
    );

    let dfs= graph.dfs_search(1);
    
}
