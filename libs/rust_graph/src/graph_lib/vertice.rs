use super::edge::Edge;

/// # Vertice
/// Estrutura destinada a representar vertices em um grafo
///
/// Contém campos como `key` e `edges`
#[derive(Debug, Hash,PartialEq, Eq)]
pub struct Vertice {
    key: i32,
    edges: Vec<Edge>,
}
#[allow(unused)]
impl Vertice {
    pub fn new(vertice_key: i32) -> Vertice {
        Vertice {
            key: vertice_key,
            edges: Vec::new(),
        }
    }

    pub fn key(&self) -> i32 {
        self.key
    }


    pub fn edges_borrow(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }

    /// Clona o vetor de arestas do vértice
    pub fn edges_clone(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn edges_ref(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn add_edge(&mut self,edge : Edge){
        self.edges.push(edge);
    }
}
