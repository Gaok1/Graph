mod graph_lib;
use graph_lib::{busca::*, graph::*,Kosaraju::*};

use text_io::scan;

fn main() {
    println!("Digite o arquivo de entrada:");
    let mut file_path: String;
    scan!("{}", file_path);
    let mut graph : Option<DiGraph> = DiGraph::from_file(&file_path);

    while let None = graph{
        println!("Digite o arquivo de entrada:");
        scan!("{}", file_path);
        graph = DiGraph::from_file(&file_path);
    }

    let graph: DiGraph = graph.unwrap();
    
    let key: i32;
    println!("Digite a chave do vertice de origem:");

    scan!("{}", key);

    let dfs:DfsStruct = graph.dfs_search(key);

   for (edge, classificaiton) in dfs.class_arestas.iter() {
       println!("Aresta: {:?} - Classificacao: {:?}", edge, classificaiton);
    }
    println!("========= KOSARAJU =========");
    let tree = graph.conex_components().clone_components();
    println!("Quantidade de componentes conexos: {}", tree.len());
    for digraph in tree.iter() {
        println!("Vertices {} Arestas {}", digraph.borrow().get_vertices_length(), digraph.borrow().get_edges_lenght());
    }
   println!("A quantidade de componentes conexos no grafo é {:?}", graph.conex_components());
}
