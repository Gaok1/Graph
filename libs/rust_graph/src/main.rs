mod graph_lib {
    pub mod busca;
    pub mod graph;
}
use graph_lib::graph::DiGraph;
use text_io::scan;

fn main() {
    println!("Digite o arquivo de entrada:");
    let file_path: String;
    scan!("{}", file_path);

    let graph = DiGraph::from_file(&file_path);
    let key: i32;
    println!("Digite a chave do vertice de origem:");
    scan!("{}", key);
    let dfs: graph_lib::busca::DfsStruct = graph.dfs_search(key);

    for (edge, classificaiton) in dfs.class_arestas.iter(){
        if edge.get_destiny_key() == key || edge.get_origin_key() == key{
            println!("Aresta: {:?} - Classificacao: {:?}", edge, classificaiton);
        }
    }
}
