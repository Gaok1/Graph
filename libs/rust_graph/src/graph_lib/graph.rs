use super::{
    busca::*,
    edge::Edge,
    kosaraju::{ConexComponents, Kosaraju},
    vertice::{self, Vertice},
};
use rand::{random, Rng};
use scan_fmt::scan_fmt;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    io::ErrorKind,
    rc::Rc,
    sync::{
        atomic::AtomicI32,
        Arc,
        RwLock,
    },
};

#[derive(Debug)]
/// # DiGraph
/// Grafo direcionado representado em lista de Adjacência
///
/// `vertices_len`: quantidade de vértices em um grafo
///
/// `edges_len`: quantidade de arestas em um grafo
///
/// `vertices`: HashMap para encontrar vértices usando sua key em O(1)
pub struct DiGraph {
    vertices_len: u32,
    edges_len: usize,
    vertices: HashMap<i32, Arc<RwLock<Vertice>>>, // thread safety
}

#[allow(unused)]
impl DiGraph {
    /// Cria um novo grafo direcionado vazio.
    pub fn new() -> DiGraph {
        DiGraph {
            vertices_len: 0,
            edges_len: 0,
            vertices: HashMap::new(),
        }
    }

    /// Cria um novo grafo direcionado com uma capacidade inicial para os vértices.
    pub fn new_sized(vertice_num: u32) -> DiGraph {
        DiGraph {
            vertices_len: 0,
            edges_len: 0,
            vertices: HashMap::with_capacity(vertice_num as usize),
        }
    }

    /// Cria um grafo direcionado a partir de um arquivo.
    ///
    /// O arquivo deve estar no formato:
    /// ```
    /// <número_de_vértices> <número_de_arestas>
    /// <origem1> <destino1>
    /// <origem2> <destino2>
    /// ...
    /// ```
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
                        "Erro {{{err}}} durante a leitura de aresta na linha {}\nconteúdo: {line}",
                        index + 2
                    );
                    return None;
                }
            };
            graph.add_edge(orig, dest);
        }
        Some(graph)
    }

    /// Cria um grafo direcionado a partir de um vetor de arestas.
    pub fn from_edges(edge_array: Vec<Edge>) -> DiGraph {
        let mut graph = DiGraph::new();

        edge_array.iter().for_each(|edge| {
            let (v,w) = (edge.origin_key(), edge.destiny_key());
            if !graph.vertice_exists(v){
                graph.add_vertice(v);
            }  
            if !graph.vertice_exists(w){
                graph.add_vertice(w);
            }
            let vertice = graph.get_vertice_arc(v).unwrap();
            let mut vertice = vertice.write().unwrap();
            vertice.add_edge(edge.clone());
            graph.edges_len += 1;
        });
        graph
    }

    /// Retorna a quantidade de vértices no grafo.
    pub fn vertices_length(&self) -> u32 {
        self.vertices_len
    }

    /// Retorna a quantidade de arestas no grafo.
    pub fn edges_length(&self) -> usize {
        self.edges_len
    }

    /// Retorna um vetor com as chaves dos vértices.
    ///
    /// `Vec<i32>` contendo as chaves dos vértices.
    pub fn get_vertice_key_array(&self) -> Vec<i32> {
        self.vertices.keys().cloned().collect()
    }

    /// Retorna a referência do Arc<RwLock<Vertice>>, se existir.
    pub fn get_vertice_arc(&self, vertice_key: i32) -> Option<Arc<RwLock<Vertice>>> {
        self.vertices.get(&vertice_key).cloned()
    }

    /// Retorna todas as arestas do grafo como um vetor.
    pub fn all_edges(&self) -> Vec<Edge> {
        let len: usize = self.edges_length();
        let mut edges = Vec::with_capacity(len as usize);
        for v in self.vertices.values() {
            let v = v.read().unwrap();
            let v_edges = v.edges_vec_ref();
            for e in v_edges {
                edges.push(e.clone());
            }
        }
        edges
    }

    pub fn remove_edge(&mut self, e:Edge){
        let mut vertice = self.get_vertice_arc(e.origin_key()).unwrap();
        let mut vertice = vertice.write().unwrap();
        vertice.remove_edge(e);
    }

    /// Aplica uma função a todas as arestas do grafo.
    ///
    /// `F`: Função que recebe uma referência mutável para `Edge`.
    pub fn mut_edges<F>(&mut self, f: F)
    where
        F: Fn(&mut Edge) -> (),
    {
        for v in self.vertices.values() {
            let mut vertice = v.write().unwrap();
            let mut_edges = vertice.edges_hashmap_mut();
            for edge_vec in mut_edges.values_mut() {
                for edge in edge_vec.iter_mut() {
                    f(edge);
                }
            }
        }
    }

    /// Verifica a existência de um vértice no grafo.
    ///
    /// `true` se existe, `false` caso contrário.
    pub fn vertice_exists(&self, vert_key: i32) -> bool {
        self.vertices.contains_key(&vert_key)
    }

    /// Adiciona um vértice ao grafo.
    pub fn add_vertice(&mut self, vertice_key: i32) {
        let vertice = Vertice::new(vertice_key);
        let vertice = Arc::new(RwLock::new(vertice));
        self.vertices.insert(vertice_key, vertice);
        self.vertices_len += 1;
    }

    /// Adiciona uma aresta ao grafo.
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
            self.edges_len += 1;
        }
    }

    /// Adiciona uma aresta ponderada ao grafo.
    pub fn add_edge_weighted(&mut self, origin_vert: i32, destiny_vert: i32, weight: i32) {
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
            self.edges_len += 1;
        }
    }

    /// Verifica se existe pelo menos uma aresta entre dois vértices.
    pub fn has_edge(&self, origin_key: i32, destiny_key: i32) -> bool {
        if let Some(vertice) = self.vertices.get(&origin_key) {
            let vertice = vertice.read().unwrap();
            return vertice.has_edge_to(destiny_key);
        }
        false
    }

    /// Retorna todas as arestas entre dois vértices.
    ///
    /// Retorna `Some(Vec<Edge>)` se houver arestas, ou `None` caso contrário.
    pub fn get_edges(&self, origin_key: i32, destiny_key: i32) -> Option<Vec<Edge>> {
        if let Some(vertice) = self.vertices.get(&origin_key) {
            let vertice = vertice.read().unwrap();
            return vertice.get_edges_to(destiny_key).cloned();
        }
        None
    }

    /// Retorna as chaves dos sucessores de um vértice.
    pub fn get_sucessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let vertice = self.get_vertice_arc(vertice_key)?;
        let vertice = vertice.read().unwrap();
        Some(
            vertice
                .edges_hashmap()
                .keys()
                .filter_map(|&(orig, dest)| if orig == vertice_key { Some(dest) } else { None })
                .collect(),
        )
    }

    /// Retorna um vetor clonado de todas as arestas de um vértice.
    pub fn get_edges_of(&self, vertice_key: i32) -> Option<Vec<Edge>> {
        let vertice = self.get_vertice_arc(vertice_key)?;
        let vertice = vertice.read().unwrap();
        Some(vertice.edges_vec())
    }

    /// Retorna as chaves dos predecessores de um vértice.
    pub fn get_predecessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let mut list: Vec<i32> = Vec::new();
        for (vert_key, vertice_ref) in &self.vertices {
            let vertice = vertice_ref.read().unwrap();
            for aresta in vertice.edges_vec_ref() {
                if aresta.destiny_key() == vertice_key {
                    list.push(aresta.origin_key());
                }
            }
        }
        Some(list)
    }

    /// Executa uma busca em profundidade a partir de uma chave de vértice.
    pub fn dfs_search(&self, mut search_key: i32) -> DfsStruct {
        let mut dfs_data = DfsStruct::new(self);
        let key_array = self.get_vertice_key_array();
        while search_key != -1 {
            self.DeepFirstSearch(search_key, &mut dfs_data);
            search_key = dfs_data.get_unexplored_vertice(&key_array);
        }
        dfs_data
    }

    /// Cria um novo grafo com todas as arestas transpostas.
    ///
    /// Na prática, apenas inverte as arestas direcionadas.
    pub fn transpose(&self) -> DiGraph {
        let mut t_graph = DiGraph::new_sized(self.vertices_len);
        let vertices = self.get_vertice_key_array();
        for vertice in vertices {
            if let Some(edges) = self.get_edges_of(vertice) {
                for edge in edges {
                    t_graph.add_edge(edge.destiny_key(), edge.origin_key());
                }
            }
        }
        t_graph
    }

    /// Verifica se um vértice alcança outro no grafo.
    ///
    /// `from_key` -> chave do vértice de origem
    ///
    /// `destiny_key` -> chave do vértice de destino
    pub fn reaches(&self, from_key: i32, destiny_key: i32) -> bool {
        let mut stack: Vec<i32> = vec![from_key];
        let mut visited: HashSet<i32> = HashSet::new();

        while let Some(vertice_key) = stack.pop() {
            if vertice_key == destiny_key {
                return true;
            }

            // Se já visitou este vértice, pule
            if !visited.insert(vertice_key) {
                continue;
            }

            if let Some(edges) = self.get_edges_of(vertice_key) {
                for aresta in edges {
                    stack.push(aresta.destiny_key());
                }
            }
        }
        false
    }

    /// Retorna o caminho entre dois vértices, se existir.
    pub fn path_between(&self, v: i32, w: i32) -> Option<Vec<Edge>> {
        let dfs_struct = self.dfs_search(v);
        let mut path = vec![];

        let mut current = w;
        while current != v {
            let father = dfs_struct.fathers.get(&current)?;
            let vertice = self.get_vertice_arc(*father).unwrap();
            let vertice = vertice.read().unwrap();
            // Coletar todas as arestas para `current` e encontrar uma que parta de `father`
            let edges_to_current = vertice.get_edges_to(current).unwrap();
            if let Some(edge) = edges_to_current.iter().find(|e| e.destiny_key() == current) {
                path.push(edge.clone());
                current = *father;
            } else {
                return None; // Caminho inconsistente
            }
        }
        path.reverse(); // Opcional: para que o caminho comece de `v` e vá até `w`
        Some(path)
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

            let arestas = self.get_edges_of(vertice_key);

            let Some(mut arestas) = arestas else {
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
                continue;
            };

            
            arestas.sort_by(|a, b| a.destiny_key().cmp(&b.destiny_key()));

            let mut descobriu_vertice = false;

            for aresta in arestas.iter() {
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
                } else if dfs_data.tempo_descoberta.get(&vertice_key).unwrap()
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
        let mut vertices_queue: Vec<(i32, i32)> =
            first_dfs_data.tempo_termino.into_iter().collect();
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

    //é const mas tem mutabilidade interna kakakak
    const MAX_EDGE_WEIGHT: AtomicI32 = AtomicI32::new(40);

    /// Cria um grafo direcionado aleatório.
    ///
    /// `vertices_len`: número de vértices
    /// `edges_len`: número de arestas (opcional)
    /// `weighted`: se as arestas devem ter pesos
    /// `negative_weight`: se os pesos podem ser negativos
    pub fn from_random(
        vertices_len: u32,
        edges_len: Option<u32>,
        weighted: bool,
        negative_weight: bool,
    ) -> DiGraph {
        let min_edges = vertices_len.saturating_sub(1);
        let mut rng = rand::thread_rng();
        let random_edges_len = rng.gen_range(0..vertices_len * Self::MAX_EDGES_MULTIPLIER);
        let mut edges_len = edges_len.unwrap_or(random_edges_len).max(min_edges);
        let mut edges_added = HashSet::<(i32, i32)>::new();
        let max_edge_weight = Self::MAX_EDGE_WEIGHT.load(std::sync::atomic::Ordering::Relaxed);
        let max_edges: u64 = vertices_len as u64 * ((vertices_len as u64) - 1);
        edges_len = edges_len.min(max_edges as u32);

        let mut graph = DiGraph::new_sized(vertices_len);
        for i in 0..vertices_len {
            graph.add_vertice(i as i32);
        }

        // Adiciona arestas para garantir conectividade mínima
        for i in 1..vertices_len {
            let i_key = i as i32;
            let dest_key: i32 = rng.gen_range(0..i_key);

            let weight = if weighted {
                let mut w = random::<i32>() % max_edge_weight;
                if !negative_weight && w < 0 {
                    w = -w;
                }
                Some(w)
            } else {
                None
            };

            if let Some(w) = weight {
                graph.add_edge_weighted(i_key, dest_key, w);
            } else {
                graph.add_edge(i_key, dest_key);
            }
            edges_added.insert((i_key, dest_key));
        }

        let mut count = vertices_len.saturating_sub(1);
        while count < edges_len {
            let origin = rng.gen_range(0..vertices_len) as i32;
            let destiny = rng.gen_range(0..vertices_len) as i32;
            if origin == destiny || edges_added.contains(&(origin, destiny)) {
                continue;
            }

            let weight = if weighted {
                let mut w = random::<i32>() % max_edge_weight;
                if !negative_weight && w < 0 {
                    w = -w;
                }
                Some(w)
            } else {
                None
            };

            if let Some(w) = weight {
                graph.add_edge_weighted(origin, destiny, w);
            } else {
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

    /// Define o peso máximo das arestas para o gerador aleatório.
    pub fn set_edge_max(coeficient: i32) {
        Self::MAX_EDGE_WEIGHT.store(coeficient, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn new_grid(height: u32, width: u32) -> DiGraph {
        let mut digraph = DiGraph::new_sized((height * width));
    
        for row in 0..height {
            for col in 0..width {
                let v = row * width + col;
    
                // Conectar com o vizinho da direita
                if col + 1 < width {
                    let right_neighbor = v + 1;
                    digraph.add_edge(v as i32, right_neighbor as i32);
                }
    
                // Conectar com o vizinho de baixo
                if row + 1 < height {
                    let bottom_neighbor = v + width;
                    digraph.add_edge(v as i32, bottom_neighbor as i32);
                }
            }
        }
    
        digraph
    }
    
}

// Iteradores
impl DiGraph {
    /// Retorna um iterador sobre os vértices do grafo.
    pub fn iter_vertices(&self) -> impl Iterator<Item = &Arc<RwLock<Vertice>>> {
        self.vertices.values()
    }
}

impl Clone for DiGraph {
    fn clone(&self) -> Self {
        Self {
            vertices_len: self.vertices_len.clone(),
            edges_len: self.edges_len.clone(),
            vertices: self.vertices.clone(),
        }
    }
}

impl DiGraph {
    /// Encontra um par de vértices (base, antibase) tal que:
    /// - `base` não tem predecessores.
    /// - `base` alcança `antibase`.
    /// - `antibase` não tem sucessores.
    pub fn find_base_antibase(&self) -> Option<(i32, i32)> {
        // 1. Encontra a base: um vértice sem predecessores
        let base = self
            .get_vertice_key_array()
            .into_iter()
            .find(|&v| self.get_predecessor(v).map_or(true, |p| p.is_empty()))?;

        // 2. Encontra a antibase: alcançável pela base e sem sucessores
        let antibase = self
            .get_vertice_key_array()
            .into_iter()
            .find(|&v| {
                v != base // Diferente da base
                    && self.reaches(base, v) // A base alcança esse vértice
                    && self.get_sucessor(v).map_or(true, |s| s.is_empty()) // Sem sucessores
            })?;
        Some((base, antibase))
    }
}
