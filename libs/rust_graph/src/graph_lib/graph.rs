use super::{busca::*, edge::{self, Edge}, kosaraju::{ConexComponents, Kosaraju}, vertice::{self, Vertice}};
use scan_fmt::scan_fmt;
use rand::{ random, Rng};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    io::ErrorKind,
    sync::{Arc, RwLock},
};


#[derive(Debug)]
/// # DiGraph
/// Grafo direcionado representado em lista de Adjacência
///
/// `vertices_num`: quantidade de vértices em um grafo
///
/// `edges_num`: quantidade de arestas em um grafo
///
/// `vertices`: HashMap para encontrar vértices usando sua key em O(1)
pub struct DiGraph {
    vertices_num: u32,
    edges_num: usize,
    vertices: HashMap<i32, Arc<RwLock<Vertice>>>,
}

#[allow(unused)]
impl DiGraph {
    pub fn new() -> DiGraph {
        DiGraph {
            vertices_num: 0,
            edges_num: 0,
            vertices: HashMap::new(),
        }
    }

    pub fn new_sized(vertice_num: u32) -> DiGraph {
        DiGraph {
            vertices_num: 0,
            edges_num: 0,
            vertices: HashMap::with_capacity(vertice_num as usize),
        }
    }

    pub fn from_file(file_path: &str) -> Option<DiGraph> {
        let file = fs::read_to_string(file_path);
        let Ok(file_content) = file else {
            let error = file.err().unwrap();
            match error.kind() {
                ErrorKind::PermissionDenied => println!("Acesso ao arquivo foi negado!"),
                ErrorKind::NotFound => println!("Arquivo não encontrado!"),
                _ => {
                    println!("Um erro inesperado aconteceu!");
                }
            }
            return None;
        };

        let mut lines = file_content.lines(); // Iterador do arquivo

        let (vert_num, edge_num) = match scan_fmt!(lines.next().unwrap_or(""), "{} {}", u32, u32) {
            Ok((v, e)) => (v, e),
            Err(err) => {
                eprintln!("Erro de leitura: {err}\nO arquivo pode não estar no formato requerido.");
                return None;
            }
        };

        let mut graph = DiGraph::new();

        for (index, line) in lines.enumerate() {
            let (orig, dest) = match scan_fmt!(line, "{} {}", i32, i32) {
                Ok(tuple) => tuple,
                Err(err) => {
                    println!(
                        "Erro {{{err}}} durante a leitura de aresta na linha {}\ncontent: {line}",
                        index + 2
                    );
                    return None;
                }
            };
            graph.add_edge(orig, dest);
        }
        Some(graph)
    }

    pub fn from_edges(edge_array: Vec<Edge>) -> DiGraph {
        let mut graph = DiGraph::new();
        edge_array.iter().for_each(|edge| {
            graph.add_edge(edge.origin_key(), edge.destiny_key());
        });
        graph
    }

    pub fn get_vertices_length(&self) -> u32 {
        self.vertices_num
    }

    pub fn get_edges_length(&self) -> usize {
        self.edges_num
    }

    /// ## Retorna um vetor com as chaves dos vértices
    ///
    /// `Vec<i32>` contendo as chaves dos vértices
    pub fn get_vertice_key_array(&self) -> Vec<i32> {
        self.vertices.keys().cloned().collect()
    }

    /// ## Returns the reference counter of the Vertice, if exists
    pub fn get_vertice_arc(&self, vertice_key: i32) -> Option<Arc<RwLock<Vertice>>> {
        self.vertices.get(&vertice_key).cloned()
    }

    pub fn all_edges(&self)-> Vec<Edge>{
        let len: usize = self.get_edges_length();
        let mut edges = Vec::with_capacity(len as usize);
        for v in self.vertices.values(){
            let v = v.read().unwrap();
            let v_edges = v.edges_ref();
            for e in v_edges{
                edges.push(e.clone());
            }
        }
        edges
        
    }

    /// ## Verifica existência de um vértice no grafo
    ///
    /// `true` se existe
    ///
    /// `false` se não existe
    pub fn vertice_exists(&self, vert_key: i32) -> bool {
        self.vertices.contains_key(&vert_key)
    }

    pub fn add_vertice(&mut self, vertice_key: i32) {
        let vertice = Vertice::new(vertice_key);
        let vertice = Arc::new(RwLock::new(vertice));
        self.vertices.insert(vertice_key, vertice);
        self.vertices_num += 1;
    }

    pub fn add_edge(&mut self, origin_vert: i32, destiny_vert: i32) {
        if !self.vertice_exists(origin_vert) {
            self.add_vertice(origin_vert);
        }
        if !self.vertice_exists(destiny_vert) {
            self.add_vertice(destiny_vert);
        }

        // Obtém o Arc<RwLock<Vertice>> referente ao vértice de origem
        if let Some(vertice_origem) = self.vertices.get(&origin_vert) {
            let mut vertice_origem = vertice_origem.write().unwrap();
            let edge = Edge::new(vertice_origem.key(), destiny_vert);
            vertice_origem.add_edge(edge);
            self.edges_num += 1;
        }
    }

    pub fn add_edge_weighted(&mut self, origin_vert: i32, destiny_vert: i32, weight:i32) {
        if !self.vertice_exists(origin_vert) {
            self.add_vertice(origin_vert);
        }
        if !self.vertice_exists(destiny_vert) {
            self.add_vertice(destiny_vert);
        }

        // Obtém o Arc<RwLock<Vertice>> referente ao vértice de origem
        if let Some(vertice_origem) = self.vertices.get(&origin_vert) {
            let mut vertice_origem = vertice_origem.write().unwrap();
            let edge = Edge::new_weighted(vertice_origem.key(), destiny_vert, weight);
            vertice_origem.add_edge(edge);
            self.edges_num += 1;
        }
    }

    /// Retorna as chaves dos sucessores do vértice
    pub fn get_sucessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let vertice = self.get_vertice_arc(vertice_key)?;
        let vertice = vertice.read().unwrap();
        Some(vertice.edges_clone().iter().map(|e| e.destiny_key()).collect())
    }

    /// ### Retorna um vetor clonado de arestas do vértice
    pub fn get_edges(&self, vertice_key: i32) -> Option<Vec<Edge>> {
        let vertice = self.get_vertice_arc(vertice_key)?;
        let vertice = vertice.read().unwrap();
        Some(vertice.edges_clone())
    }

    /// ### Retorna as chaves dos predecessores do vértice
    pub fn get_predecessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let mut list: Vec<i32> = Vec::new();
        for (vert_key, vertice_ref) in &self.vertices {
            let vertice = vertice_ref.read().unwrap();
            for aresta in vertice.edges_borrow() {
                if aresta.destiny_key() == vertice_key {
                    list.push(aresta.origin_key());
                }
            }
        }
        Some(list)
    }

    pub fn dfs_search(&self, mut search_key: i32) -> DfsStruct {
        let mut dfs_data = DfsStruct::new(self);
        let key_array = self.get_vertice_key_array();
        while search_key != -1 {
            self.DeepFirstSearch(search_key, &mut dfs_data);
            search_key = dfs_data.get_unexplored_vertice(&key_array);
        }
        dfs_data
    }

    /// ## Cria um novo grafo com todas as arestas transpostas
    pub fn transpose(&self) -> DiGraph {
        let mut t_graph = DiGraph::new_sized(self.vertices_num);
        let vertices = self.get_vertice_key_array();
        for vertice in vertices {
            if let Some(edges) = self.get_edges(vertice) {
                for edge in edges {
                    t_graph.add_edge(edge.destiny_key(), edge.origin_key());
                }
            }
        }
        t_graph
    }

    pub fn find_from(&self, from_key: i32, destiny_key: i32) -> bool {
        let mut stack: Vec<i32> = vec![from_key];
        let mut visited: HashSet<i32> = HashSet::new();
    
        while let Some(vertice_key) = stack.pop() {
            if vertice_key == destiny_key {
               // println!("Caminho encontrado de {} para {}", from_key, destiny_key);
                return true;
            }

            // Se já visitou este vértice, pule
            if !visited.insert(vertice_key) {
                continue;
            }

            if let Some(edges) = self.get_edges(vertice_key) {
                for aresta in edges {
                    stack.push(aresta.destiny_key());
                }
            }
        }
        false
    }

    pub fn print(&self) {
        for (key, vertice) in &self.vertices {
            let vertice = vertice.read().unwrap();
            println!("Vértice: {}", key);
            for aresta in vertice.edges_borrow() {
                println!("Aresta: {:?}", aresta);
            }
        }
    }

    

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        dot.push_str("layout=dot;\n");  // Define o layout como 'dot' (árvore)
        dot.push_str("node [shape=circle];\n");
        dot.push_str("edge [dir=forward];\n");  // Direção das arestas
    
        for (key, vertice) in &self.vertices {
            let vertice = vertice.read().unwrap();
    
            for aresta in vertice.edges_borrow() {
                dot.push_str(&format!(
                    "{} -> {} [label=\"{}\"];\n",  // Define as conexões e inclui o peso
                    key, aresta.destiny_key(), aresta.weight()
                ));
            }
        }
    
        dot.push_str("}");
        dot
    }
    
    pub fn to_dot_png(&self, file_path: &str) {
        let dot = self.to_dot();
        let dot_file = format!("{}.dot", file_path);
        let png_file = format!("{}.png", file_path);
    
        fs::write(&dot_file, dot).expect("Erro ao escrever arquivo DOT");
    
        let output = std::process::Command::new("dot")
            .arg("-Tpng")
            .arg(&dot_file)
            .arg("-o")
            .arg(&png_file)
            .output()
            .expect("Erro ao gerar imagem PNG");
    
        if !output.status.success() {
            eprintln!("Erro ao gerar imagem PNG: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    
    


}

/// Implementação de busca em profundidade
impl DeepFirstSearch for DiGraph {
    fn DeepFirstSearch(&self, search_key: i32, dfs_data: &mut DfsStruct) {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(search_key);

        while let Some(&vertice_key) = stack.last() {
            if !dfs_data.already_visited(vertice_key) {
                dfs_data.start_exploring(vertice_key);
            }

            let arestas = self.get_edges(vertice_key);

            let Some(mut arestas) = arestas else {
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
                continue;
            };

            arestas.sort();
            let mut descobriu_vertice = false;

            for aresta in arestas {
                if dfs_data.is_aresta_marked(aresta.id() as i32) {
                    continue; // Aresta já classificada
                }
                dfs_data.arestas_marked.insert(aresta.id() as i32, true); // Marca a aresta que está sendo explorada

                if !dfs_data.already_visited(aresta.destiny_key()) {
                    // Não foi descoberto ainda, árvore
                    dfs_data.fathers.insert(aresta.destiny_key(), vertice_key);
                    stack.push(aresta.destiny_key()); // Empilha o vértice
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Arvore);
                    descobriu_vertice = true;
                    dfs_data.add_tree_edge(aresta.origin_key(), aresta.destiny_key());
                    break;
                }
                if !dfs_data.already_explored(aresta.destiny_key()) {
                    // Se ainda não finalizou, é retorno
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Retorno);
                } else if dfs_data
                    .tempo_descoberta
                    .get(&vertice_key)
                    .unwrap()
                    < dfs_data
                        .tempo_descoberta
                        .get(&aresta.destiny_key())
                        .unwrap()
                {
                    // Se já finalizou a busca, mas ele é mais novo que o vertice_key, é avanço
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Avanco);
                } else {
                    // Se já finalizou a busca, mas ele é mais velho que o vertice_key, é cruzamento
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Cruzamento);
                }
            }

            if !descobriu_vertice {
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
            }
        }
    }
}

// Kosaraju
impl Kosaraju for DiGraph {
    fn conex_components(&self) -> ConexComponents {
        let t_graph = self.transpose();
        let first_dfs_data = t_graph.dfs_search(1);
        let mut vertices_queue: Vec<(i32, i32)> = first_dfs_data.tempo_termino.into_iter().collect();
        vertices_queue.sort_by(|a, b| b.1.cmp(&a.1)); // Ordenar decrescente por tempo de término

        let vertices_queue: Vec<i32> = vertices_queue.into_iter().map(|tuple| tuple.0).collect(); // Filtrar para conter apenas os vertices
                                                                                                 // Agora temos em ordem decrescente o tempo de término, basta realizar a busca e pegar os componentes fortemente conexos
        let mut dfs_data = DfsStruct::new(self);
        let mut search_key = dfs_data.get_unexplored_vertice(&vertices_queue);
        while search_key != -1 {
            self.DeepFirstSearch(search_key, &mut dfs_data);
            search_key = dfs_data.get_unexplored_vertice(&vertices_queue);
        }
        ConexComponents::from_dfsData(&mut dfs_data)
    }
}



// Gerador aleatório de grafo
impl DiGraph {
    const MAX_EDGES_MULTIPLIER: u32 = 20;

    const MAX_EDGE_WEIGHT: i32 = 200;

    pub fn from_random(vertices_len: u32, edges_len: Option<u32>, weighted : bool, negative_weight:bool) -> DiGraph {
        let min_edges = if vertices_len > 0 { vertices_len - 1 } else { 0 };
        let mut rng = rand::thread_rng();
        let random_edges_len = rng.gen_range(0..vertices_len * Self::MAX_EDGES_MULTIPLIER);
        let mut edges_len = edges_len.unwrap_or(random_edges_len).max(min_edges); // Compara e retorna o máximo de dois valores
        let mut edges_added = HashSet::<(i32, i32)>::new();
        
        let max_edges = (vertices_len * (vertices_len-1));
        edges_len = u32::min(edges_len, max_edges);
        
        let mut graph = DiGraph::new_sized(vertices_len);
        if vertices_len > 0 {
            graph.add_vertice(0);
        }

        for i in 1..vertices_len {
            let i_key = i as i32;
            let dest_key: i32 = rng.gen_range(0..i_key);
            if weighted {
                if negative_weight{
                    graph.add_edge_weighted(i_key, dest_key, random::<i32>() % Self::MAX_EDGE_WEIGHT);
                }else{
                    let mut weight = random::<i32>() % Self::MAX_EDGE_WEIGHT;
                    if weight < 0 {weight = weight * -1;}
                    graph.add_edge_weighted(i_key, dest_key, weight);
                }
                
            }
            else {
                graph.add_edge(i_key, dest_key);
            }
            edges_added.insert((i_key, dest_key));
        }

        let mut count = min_edges;
        while count < edges_len {
            let origin = rng.gen_range(0..vertices_len) as i32;
            let destiny = rng.gen_range(0..vertices_len) as i32;
            if origin == destiny || edges_added.contains(&(origin, destiny)) {
                continue;
            }
            if weighted {
                if negative_weight{
                    graph.add_edge_weighted(origin, destiny, random::<i32>() % Self::MAX_EDGE_WEIGHT);
                }else{
                    let mut weight = random::<i32>() % Self::MAX_EDGE_WEIGHT;
                    if weight < 0 {weight = weight * -1;}
                    graph.add_edge_weighted(origin, destiny, weight);
                }
            }
            else {
                graph.add_edge(origin, destiny);
            }
            edges_added.insert((origin, destiny));
            count += 1;
        }
        println!(
            "Grafo gerado com {} vértices e {} arestas",
            vertices_len, edges_len
        );
        graph
    }
}


// iterators

impl DiGraph{

    pub fn iter_vertices(&self) -> impl Iterator<Item = &Arc<RwLock<Vertice>>> {
        self.vertices.values()
    }   

}

