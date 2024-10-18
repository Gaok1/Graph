use std::collections::HashMap;
use super::edge::Edge;

/// # Vertice
/// Estrutura destinada a representar vértices em um grafo.
///
/// Contém campos como `key` e `edges`.
#[derive(Debug, PartialEq, Eq)]
pub struct Vertice {
    key: i32,
    edges: HashMap<(i32, i32), Vec<Edge>>, // Arestas armazenadas como HashMap com vetores para permitir arestas paralelas
}

impl Vertice {
    /// Cria um novo vértice com uma chave específica.
    ///
    /// # Argumentos
    ///
    /// * `vertice_key` - A chave (identificador) do vértice.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let vertice = Vertice::new(1);
    /// ```
    pub fn new(vertice_key: i32) -> Vertice {
        Vertice {
            key: vertice_key,
            edges: HashMap::new(),
        }
    }

    /// Retorna a chave do vértice.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let vertice = Vertice::new(1);
    /// assert_eq!(vertice.key(), 1);
    /// ```
    pub fn key(&self) -> i32 {
        self.key
    }

    /// Obtém uma referência para as arestas específicas que levam a um vértice destino.
    ///
    /// # Argumentos
    ///
    /// * `destiny_key` - A chave do vértice destino.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let mut vertice = Vertice::new(1);
    /// // Suponha que Edge tenha uma função `new` adequada
    /// vertice.add_edge(Edge::new(1, 2));
    /// assert!(vertice.get_edges_to(2).is_some());
    /// ```
    pub fn get_edges_to(&self, destiny_key: i32) -> Option<&Vec<Edge>> {
        self.edges.get(&(self.key, destiny_key))
    }

    /// Retorna uma referência imutável ao `HashMap` de arestas.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let vertice = Vertice::new(1);
    /// let edges_map = vertice.edges_hashmap();
    /// ```
    pub fn edges_hashmap(&self) -> &HashMap<(i32, i32), Vec<Edge>> {
        &self.edges
    }

    /// Retorna uma referência mutável ao `HashMap` de arestas.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let mut vertice = Vertice::new(1);
    /// let edges_map_mut = vertice.edges_hashmap_mut();
    /// ```
    pub fn edges_hashmap_mut(&mut self) -> &mut HashMap<(i32, i32), Vec<Edge>> {
        &mut self.edges
    }

    /// Retorna todas as arestas como um vetor (`Vec<Edge>`).
    ///
    /// # Exemplo
    ///
    /// ```
    /// let vertice = Vertice::new(1);
    /// let all_edges = vertice.edges_vec();
    /// ```
    pub fn edges_vec(&self) -> Vec<Edge> {
        self.edges.values().flat_map(|vec| vec.clone()).collect()
    }

    /// Retorna referências imutáveis para todas as arestas.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let vertice = Vertice::new(1);
    /// let all_edges_ref = vertice.edges_vec_ref();
    /// ```
    pub fn edges_vec_ref(&self) -> Vec<&Edge> {
        self.edges.values().flat_map(|vec| vec.iter()).collect()
    }

    /// Retorna referências mutáveis para todas as arestas.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let mut vertice = Vertice::new(1);
    /// let all_edges_mut = vertice.edges_vec_mut();
    /// ```
    pub fn edges_vec_mut(&mut self) -> Vec<&mut Edge> {
        self.edges.values_mut().flat_map(|vec| vec.iter_mut()).collect()
    }

    /// Adiciona uma aresta ao `HashMap`. Permite múltiplas arestas para o mesmo destino.
    ///
    /// # Argumentos
    ///
    /// * `edge` - A aresta a ser adicionada.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let mut vertice = Vertice::new(1);
    /// let edge = Edge::new(1, 2); // Supondo que Edge tenha essa função
    /// vertice.add_edge(edge);
    /// ```
    pub fn add_edge(&mut self, edge: Edge) {
        let key = (self.key, edge.destiny_key());
        self.edges.entry(key).or_insert_with(Vec::new).push(edge);
    }

    /// Verifica se há pelo menos uma aresta entre este vértice e o destino dado.
    ///
    /// # Argumentos
    ///
    /// * `destiny_key` - A chave do vértice destino.
    ///
    /// # Exemplo
    ///
    /// ```
    /// let mut vertice = Vertice::new(1);
    /// vertice.add_edge(Edge::new(1, 2));
    /// assert!(vertice.has_edge_to(2));
    /// assert!(!vertice.has_edge_to(3));
    /// ```
    pub fn has_edge_to(&self, destiny_key: i32) -> bool {
        self.edges.contains_key(&(self.key, destiny_key))
    }


    pub fn remove_edge(&mut self, e: Edge) {
        let (v, w) = (e.origin_key(), e.destiny_key());
    
        if let Some((_, edges)) = self.edges.remove_entry(&(v, w)) {
            let filtered_edges: Vec<_> = edges.into_iter()
                .filter(|edge| edge.weight() != e.weight())
                .collect();
    
            if !filtered_edges.is_empty() {
                self.edges.insert((v, w), filtered_edges);
            }
        }
    }
    

}
