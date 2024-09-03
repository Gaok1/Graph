// Implementar lista de adjacência em grafos

use scan_fmt::scan_fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, fs};


#[derive(Clone, Debug)]
struct Edge {
    destiny_key: i32,
    origin_key: i32,
    id: usize,
}

impl Edge {
    pub fn new(origin_vertice: i32, destiny_vertice: i32) -> Edge {
        static mut EDGE_COUNTER: i32 = 0;
        let mut id = 0;
        unsafe {
            id = EDGE_COUNTER;
            EDGE_COUNTER += 1;
        }
        Edge {
            id: id as usize,
            destiny_key: destiny_vertice,
            origin_key: origin_vertice,
        }
    }
}

///# Vertice
/// Estrutura destinada a representar vertices em um grafo
///
/// contém campos como `key` e `edges`
struct Vertice {
    key: i32,
    edges: Vec<Edge>,
}
impl Vertice {
    pub fn new(vertice_key: i32) -> Vertice {
        Vertice {
            key: vertice_key,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, destiny_key: i32){
        self.edges.insert(self.edges.len(), Edge::new(self.key, destiny_key));
    }
    ///Clona o vetor de arestas do vértice
    /// 
    pub fn get_Edges_clone(&self) -> Vec<Edge>{
        self.edges.clone()
    }
    pub fn get_Edges_ref(&self) -> &Vec<Edge>{
         &self.edges
    }
}

/// # DiGraph
/// Grafo direcionado representado em lista de Adjacência
///
/// `vertices_num` quantidade de vértices em um grafo
///
/// `edges_num` quantidade de arestas em um grafo
///
/// `Vertices` HashSet para encontrar vértices usando sua key em O(1)
pub struct DiGraph {
    vertices_num: u32,
    edges_num: u32,
    vertices: HashMap<i32, Rc<RefCell<Vertice>>>,
}

impl DiGraph {
    pub fn new(vertice_num: u32, edge_num: u32) -> DiGraph {
        DiGraph {
            vertices_num: vertice_num,
            edges_num: edge_num,
            vertices: HashMap::new(),
        }
    }

    pub fn get_vertices_length(&self) -> u32{
        return self.vertices_num;
    }

    pub fn get_edges_lenght(&self) -> u32{
        return self.edges_num;
    }

    pub fn get_Vertice_cloneRef(&self, vertice_key:i32) -> Option<Rc<RefCell<Vertice>>>{
        let vertice = self.vertices.get(&vertice_key);
        if(vertice.is_none()){
            return None;
        }
        Some(vertice.unwrap().clone())
    }

    /// ## Verifica existência de um vértice no grafo
    ///
    /// `true` se existe
    ///
    /// `false` se não existe
    pub fn vertice_exists(&self, vert_key: i32) -> bool {
        !self.vertices.get(&vert_key).is_none()
    }

    pub fn add_vertice(&mut self, vertice_key: i32) {
        let vertice = Vertice::new(vertice_key);
        let vertice = Rc::new(RefCell::new(vertice));
        self.vertices.insert(vertice_key, vertice);
    }

    pub fn add_edge(&mut self, origin_vert: i32, destiny_vert: i32) {
        if (!self.vertice_exists(origin_vert)) {
            self.add_vertice(origin_vert);
        }
        if (!self.vertice_exists(destiny_vert)) {
            self.add_vertice(destiny_vert);
        }

        // Obtém o `Rc<RefCell<Vertice>>` referente ao vértice de origem
        let mut vertice_origem = self.vertices.get(&origin_vert).unwrap().try_borrow_mut().unwrap();
        vertice_origem.add_edge(destiny_vert); // precisa ser mutavel
    }

    pub fn from_file(file_name: &str) -> DiGraph {
        let content = fs::read_to_string(file_name).unwrap();
        let mut first_line: bool = true;

        let mut graph = DiGraph::new(0, 0);

        for line in content.lines() {
            if (first_line) {
                let (vert_num, edge_num) = scan_fmt!(line, "{} {}", u32, u32).unwrap();
                graph = DiGraph::new(vert_num, edge_num);
                first_line = false;
                continue;
            }
            let (orig, dest) = scan_fmt!(line, "{} {}", i32, i32).unwrap();
            graph.add_edge(orig, dest);
        }
        graph
    }

    /// retorna as chaves dos sucessores do vértice
    /// 
    pub fn get_sucessor(&self, vertice_key:i32) -> Option<Vec<i32>>{
        let mut vertice: Option<Rc<RefCell<Vertice>>> = self.get_Vertice_cloneRef(vertice_key);
        if(vertice.is_none()){
            return  None;
        }

        let vert_ref = vertice.unwrap();
        
        let vertice = vert_ref.borrow();
        let arestas: &Vec<Edge> = vertice.get_Edges_ref();

        let mut sucessors: Vec<i32> = Vec::new();
        for aresta in arestas.iter(){
            sucessors.insert(sucessors.len(), aresta.destiny_key);
        }
        return Some(sucessors);
    }

    // retorna sa chaves dos predecessores do vértice
    /// 
    pub fn get_predecessor(&self, vertice_key:i32)-> Option<Vec<i32>>{
        let mut vertice: Option<Rc<RefCell<Vertice>>> = self.get_Vertice_cloneRef(vertice_key);
        if(vertice.is_none()){
            return  None;
        }
        let mut list: Vec<i32> = Vec::new();
        for (vert_key, vertice_ref) in self.vertices.iter(){
            let vertice_ref = vertice_ref.borrow();
            for aresta in vertice_ref.edges.iter(){
                if(aresta.destiny_key == vertice_key){
                    list.insert(list.len(), aresta.origin_key);
                }
            }
        }

        return  Some(list);
    }

}
