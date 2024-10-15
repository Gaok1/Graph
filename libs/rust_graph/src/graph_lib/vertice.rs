use std::collections::HashMap;
use super::edge::Edge;

/// # Vertice
/// Estrutura destinada a representar vértices em um grafo.
///
/// Contém campos como `key` e `edges`
#[derive(Debug, PartialEq, Eq)]
pub struct Vertice {
    key: i32,
    edges: HashMap<(i32, i32), Edge>, // Arestas armazenadas como HashMap
}

impl Vertice {
    /// Cria um novo vértice com uma chave específica.
    pub fn new(vertice_key: i32) -> Vertice {
        Vertice {
            key: vertice_key,
            edges: HashMap::new(),
        }
    }

    /// Retorna a chave do vértice.
    pub fn key(&self) -> i32 {
        self.key
    }

    /// Obtém uma referência para uma aresta específica, dado um vértice destino.
    pub fn get_edge_to(&self, destiny_key: i32) -> Option<&Edge> {
        self.edges.get(&(self.key, destiny_key))
    }

    /// Retorna uma referência imutável ao `HashMap` de arestas.
    pub fn edges_hashmap(&self) -> &HashMap<(i32, i32), Edge> {
        &self.edges
    }

    /// Retorna uma referência mutável ao `HashMap` de arestas.
    pub fn edges_hashmap_mut(&mut self) -> &mut HashMap<(i32, i32), Edge> {
        &mut self.edges
    }

    /// Retorna todas as arestas como um vetor (`Vec<Edge>`).
    pub fn edges_vec(&self) -> Vec<Edge> {
        self.edges.values().cloned().collect()
    }

    pub fn edges_vec_ref(&self) -> Vec<&Edge> {
        self.edges.values().collect()
    }

    pub fn edges_vec_mut(&mut self) -> Vec<&mut Edge> {
        self.edges.values_mut().collect()
    }

    /// Adiciona uma aresta ao `HashMap`.
    pub fn add_edge(&mut self, edge: Edge) {
        let key = (self.key, edge.destiny_key());
        self.edges.insert(key, edge);
    }

    /// Verifica se há uma aresta entre este vértice e o destino dado.
    pub fn has_edge_to(&self, destiny_key: i32) -> bool {
        self.edges.contains_key(&(self.key, destiny_key))
    }
}
