use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::graph_lib::{edge::Edge, graph::DiGraph};

// Busca em profundidade
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
        matches!(self, EdgeClassification::Arvore)
    }
    pub fn is_retorno(&self) -> bool {
        matches!(self, EdgeClassification::Retorno)
    }
    pub fn is_avanco(&self) -> bool {
        matches!(self, EdgeClassification::Avanco)
    }
    pub fn is_cruzamento(&self) -> bool {
        matches!(self, EdgeClassification::Cruzamento)
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
        println!(
            "Descobrindo vértice {} no tempo {}",
            vertice_key, self.clock
        );
        self.clock += 1;
    }

    /// Finaliza a exploração de um vértice marcando o tempo de término e incrementando o clock
    /// `vertice_k` - chave do vértice a ser finalizado
    pub fn finish_exploring(&mut self, vertice_k: i32) {
        self.tempo_termino.insert(vertice_k, self.clock);
        println!(
            "Finalizando exploração do vértice {} no tempo {}",
            vertice_k, self.clock
        );
        self.clock += 1;
    }

    /// Retorna a chave de um vértice ainda não explorado
    /// `key_array` - array de chaves de vértices a serem verificados
    pub fn get_unexplored_vertice(&self, key_array: &Vec<i32>) -> i32 {
        for key in key_array {
            if self.tempo_descoberta.get(&key).is_none() {
                return *key;
            }
        }
        -1
    }

    /// Checa se uma aresta já foi explorada
    /// `aresta_id` - chave da aresta a ser classificada
    ///
    /// ## Retorna
    /// `true` se a aresta já foi classificada, `false` caso contrário
    pub fn is_aresta_marked(&self, aresta_id: i32) -> bool {
        self.arestas_marked.get(&aresta_id).is_some()
    }

    /// Checa se um vértice já foi visitado
    ///
    /// Um vértice é considerado visitado se o tempo de descoberta já foi marcado
    ///
    /// `vertice_key` - chave do vértice a ser verificado
    pub fn already_visited(&self, vertice_key: i32) -> bool {
        self.tempo_descoberta.get(&vertice_key).is_some()
    }

    /// Checa se um vértice já foi explorado
    ///
    /// Um vértice é considerado explorado se o tempo de término já foi marcado
    ///
    /// `vertice_key` - chave do vértice a ser verificado
    pub fn already_explored(&self, vertice_key: i32) -> bool {
        self.tempo_termino.get(&vertice_key).is_some()
    }

    pub fn classificate_aresta(&mut self, aresta: &Edge, class: EdgeClassification) {
        self.class_arestas.insert(aresta.clone(), class.clone());
        println!(
            "Classificando aresta {} -> {} como {:?}",
            aresta.origin_key(),
            aresta.destiny_key(),
            class
        );
        if class.is_arvore() {
            let (v, w) = aresta.v_w();
            self.add_tree_edge(v, w);
        }
    }

    /// Obtém todas as raízes de uma busca em profundidade
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

        // Primeiro, tente encontrar uma árvore que já contenha o vértice de origem
        for tree in trees.iter_mut() {
            let mut tree_mut = tree.try_borrow_mut().unwrap();
            if tree_mut.vertice_exists(origin_vert) {
                tree_mut.add_edge(Edge::new(origin_vert, destiny_vert));
                println!(
                    "Adicionando aresta {} -> {} na árvore existente",
                    origin_vert, destiny_vert
                );
                return;
            }
        }

        // Se o vértice de origem não foi encontrado em nenhuma árvore, crie uma nova árvore
        println!("Adicionando nova árvore com raiz {}", origin_vert);
        let mut new_tree = DiGraph::new();
        new_tree.add_edge(Edge::new(origin_vert, destiny_vert));
        trees.push(Rc::new(RefCell::new(new_tree)));
    }

    pub fn add_root(&mut self, root: i32) {
        let trees: &mut Vec<Rc<RefCell<DiGraph>>> = self.trees.borrow_mut();
        let mut new_tree = DiGraph::new();
        new_tree.add_vertice(root);
        trees.push(Rc::new(RefCell::new(new_tree)));
        println!("Adicionando raiz {}", root);
    }
}

pub trait DeepFirstSearch {
    fn DeepFirstSearch(&self, start_vertice: i32, data: &mut DfsStruct);
}

/// Implementação de busca em profundidade
impl DeepFirstSearch for DiGraph {
    fn DeepFirstSearch(&self, search_key: i32, dfs_data: &mut DfsStruct) {
        println!("Iniciando DFS a partir do vértice {}", search_key);
        let mut stack: Vec<i32> = Vec::new();
        stack.push(search_key);
        dfs_data.add_root(search_key);

        while let Some(&vertice_key) = stack.last() {
            println!("Topo da pilha: {}", vertice_key);

            if !dfs_data.already_visited(vertice_key) {
                dfs_data.start_exploring(vertice_key);
            } else {
                println!("Vértice {} já foi visitado", vertice_key);
            }

            let arestas = self.edges_of(vertice_key);

            let Some(mut arestas) = arestas else {
                println!(
                    "Vértice {} não possui arestas restantes. Finalizando exploração.",
                    vertice_key
                );
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
                continue;
            };

            arestas.sort_by(|a, b| a.destiny_key().cmp(&b.destiny_key()));
            println!("Processando arestas do vértice {}", vertice_key);

            let mut descobriu_vertice = false;

            for aresta in arestas.iter() {
                if dfs_data.is_aresta_marked(aresta.id() as i32) {
                    println!("Aresta {} já foi marcada. Pulando.", aresta.id());
                    continue; // Aresta já classificada
                }
                dfs_data.arestas_marked.insert(aresta.id() as i32, true); // Marca a aresta que está sendo explorada
                println!("Marcando aresta {} como explorada", aresta.id());

                if !dfs_data.already_visited(aresta.destiny_key()) {
                    println!(
                        "Aresta de árvore detectada: {} -> {}",
                        vertice_key,
                        aresta.destiny_key()
                    );
                    // Não foi descoberto ainda, árvore
                    dfs_data.fathers.insert(aresta.destiny_key(), vertice_key);
                    stack.push(aresta.destiny_key()); // Empilha o vértice
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Arvore);
                    descobriu_vertice = true;
                    break;
                }
                if !dfs_data.already_explored(aresta.destiny_key()) {
                    println!(
                        "Aresta de retorno detectada: {} -> {}",
                        vertice_key,
                        aresta.destiny_key()
                    );
                    // Se ainda não finalizou, é retorno
                    dfs_data.classificate_aresta(&aresta, EdgeClassification::Retorno);
                } else {
                    let vertice_descoberta =
                        dfs_data.tempo_descoberta.get(&vertice_key).unwrap_or(&-1);
                    let destino_descoberta = dfs_data
                        .tempo_descoberta
                        .get(&aresta.destiny_key())
                        .unwrap_or(&-1);
                    if vertice_descoberta < destino_descoberta {
                        println!(
                            "Aresta de avanço detectada: {} -> {}",
                            vertice_key,
                            aresta.destiny_key()
                        );
                        // Se já finalizou a busca, mas ele é mais novo que o vertice_key, é avanço
                        dfs_data.classificate_aresta(&aresta, EdgeClassification::Avanco);
                    } else {
                        println!(
                            "Aresta de cruzamento detectada: {} -> {}",
                            vertice_key,
                            aresta.destiny_key()
                        );
                        // Se já finalizou a busca, mas ele é mais velho que o vertice_key, é cruzamento
                        dfs_data.classificate_aresta(&aresta, EdgeClassification::Cruzamento);
                    }
                }
            }

            if !descobriu_vertice {
                println!(
                    "Todas as arestas do vértice {} foram processadas. Finalizando exploração.",
                    vertice_key
                );
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
            }
        }
        println!("Árvores resultantes:");
    }
}
