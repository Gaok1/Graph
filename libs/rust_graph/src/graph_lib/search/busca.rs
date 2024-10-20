use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::graph_lib::{edge::Edge, graph::DiGraph};
//busca em profundidade
#[derive(Clone, Debug)]
pub enum EdgeClassification {
    Arvore,
    Retorno,
    Avanco,
    Cruzamento,
}

#[allow(unused)]
impl EdgeClassification {
    pub fn is_arvore(&self) -> bool {
        match self {
            EdgeClassification::Arvore => true,
            _ => false,
        }
    }
    pub fn is_retorno(&self) -> bool {
        match self {
            EdgeClassification::Retorno => true,
            _ => false,
        }
    }
    pub fn is_avanco(&self) -> bool {
        match self {
            EdgeClassification::Avanco => true,
            _ => false,
        }
    }
    pub fn is_cruzamento(&self) -> bool {
        match self {
            EdgeClassification::Cruzamento => true,
            _ => false,
        }
    }
}

/// Estrutura destinada a armazenar o resultado da busca em profundidade
#[derive(Clone, Debug)]
pub struct DfsStruct {
    pub tempo_descoberta: HashMap<i32, i32>,
    pub tempo_termino: HashMap<i32, i32>,
    pub fathers: HashMap<i32, i32>,
    pub class_arestas: HashMap<Edge, EdgeClassification>,
    pub arestas_marked: HashMap<i32, bool>,
    pub trees: Vec<Rc<RefCell<DiGraph>>>,
    clock: i32,
}
#[allow(unused)]
impl DfsStruct {
    pub fn new(g: &DiGraph) -> DfsStruct {
        let v_len = g.vertices_length() as usize;
        let e_len = g.edges_length() as usize;
        DfsStruct {
            tempo_descoberta: HashMap::with_capacity(v_len),
            tempo_termino: HashMap::with_capacity(v_len),
            fathers: HashMap::with_capacity(v_len),
            class_arestas: HashMap::with_capacity(e_len),
            arestas_marked: HashMap::with_capacity(e_len),
            trees: vec![],
            clock: 0,
        }
    }

    pub fn start_exploring(&mut self, vertice_key: i32) {
        self.tempo_descoberta.insert(vertice_key, self.clock);
        self.clock += 1;
    }
    /// Finaliza a exploração de um vertice maracndo o tempo de termino e incrementando o clock
    /// `vertice_k` - chave do vertice a ser finalizado
    pub fn finish_exploring(&mut self, vertice_k: i32) {
        self.tempo_termino.insert(vertice_k, self.clock);
        self.clock += 1;
    }
    /// Retorna a chave de um vertice ainda não explorado
    /// `g` - grafo utilizado na busca
    pub fn get_unexplored_vertice(&self, key_array: &Vec<i32>) -> i32 {
        for key in key_array {
            if self.tempo_descoberta.get(&key).is_none() {
                return *key;
            }
        }
        -1
    }
    /// checa se uma aresta ja foi explorada
    /// `aresta_key` - chave da aresta a ser classificada
    ///
    /// ## Retorna
    /// `true` se a aresta ja foi classificada, `false` caso contrario
    pub fn is_aresta_marked(&self, aresta_key: i32) -> bool {
        match self.arestas_marked.get(&aresta_key) {
            Some(value) => *value,
            None => false,
        }
    }
    /// Checa se um vertice ja foi visitado
    ///
    /// um vertice é considerado visitado se o tempo de descoberta ja foi marcado
    ///
    /// `vertice_key` - chave do vertice a ser verificado
    pub fn already_visited(&self, vertice_key: i32) -> bool {
        self.tempo_descoberta.get(&vertice_key).is_some()
    }
    /// Checa se um vertice ja foi explorado
    ///
    /// um vertice é considerado explorado se o tempo de termino ja foi marcado
    ///
    /// `vertice_key` - chave do vertice a ser verificado
    pub fn already_explored(&self, vertice_key: i32) -> bool {
        self.tempo_termino.get(&vertice_key).is_some()
    }

    pub fn classificate_aresta(&mut self, aresta: &Edge, class: EdgeClassification) {
        self.class_arestas.insert(aresta.clone(), class);
    }

    /// ### get all the roots from a deep first search
    pub fn get_roots(&self) -> Vec<i32> {
        let mut roots: Vec<i32> = vec![];
        for (vertice, _) in self.tempo_descoberta.iter() {
            if self.fathers.get(vertice).is_none() {
                roots.push(*vertice);
            }
        }
        roots
    }

    pub fn add_tree_edge(&mut self, origin_vert: i32, destiny_vert: i32) {
        let trees: &mut Vec<Rc<RefCell<DiGraph>>> = self.trees.borrow_mut();
        if self.fathers.get(&origin_vert).is_none() {
            let mut new_root = DiGraph::new();
            new_root.add_edge(origin_vert, destiny_vert);
            trees.push(Rc::new(RefCell::new(new_root)));
            return;
        }
        for tree in trees.iter_mut() {
            let mut tree_mut = tree.try_borrow_mut().unwrap();
            if tree_mut.vertice_exists(origin_vert) {
                tree_mut.add_edge(origin_vert, destiny_vert);
            }
        }
    }
}

pub trait DeepFirstSearch {
    fn DeepFirstSearch(&self, start_vertice: i32, data: &mut DfsStruct);
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

            let arestas = self.edges_of(vertice_key);

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