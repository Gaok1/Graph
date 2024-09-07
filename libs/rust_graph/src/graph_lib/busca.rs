
use super::graph::*;
use std::collections::HashMap;
//busca em profundidade
#[derive(Clone,Debug)]
pub enum DfsClassification {
    Arvore,
    Retorno,
    Avanco,
    Cruzamento,
}
#[allow(unused)]
impl DfsClassification{
    pub fn is_arvore(&self) -> bool{
        match self{
            DfsClassification::Arvore => true,
            _ => false,
        }
    }
    pub fn is_retorno(&self) -> bool{
        match self{
            DfsClassification::Retorno => true,
            _ => false,
        }
    }
    pub fn is_avanco(&self) -> bool{
        match self{
            DfsClassification::Avanco => true,
            _ => false,
        }
    }
    pub fn is_cruzamento(&self) -> bool{
        match self{
            DfsClassification::Cruzamento => true,
            _ => false,
        }
    }
}

/// Estrutura destinada a armazenar o resultado da busca em profundidade
#[derive(Clone,Debug)]
pub struct DfsStruct{
    pub tempo_descoberta : HashMap<i32,i32>,
    pub tempo_termino : HashMap<i32,i32>,
    pub fathers : HashMap<i32,i32>,
    pub class_arestas : HashMap<Edge,DfsClassification>,
    pub arestas_marked : HashMap<i32,bool>,
    clock: i32,
}
impl DfsStruct{
    
    pub fn new(g:&DiGraph) -> DfsStruct{
        let v_len = g.get_vertices_length() as usize;
        let e_len = g.get_edges_lenght() as usize;
        DfsStruct{
            tempo_descoberta: HashMap::with_capacity(v_len),
            tempo_termino: HashMap::with_capacity(v_len),
            fathers: HashMap::with_capacity(v_len),
            class_arestas: HashMap::with_capacity(e_len),
            arestas_marked: HashMap::with_capacity(e_len),
            clock: 0,
        }
    }

    pub fn start_exploring(&mut self, vertice_key:i32){
        self.tempo_descoberta.insert(vertice_key, self.clock);
        self.clock += 1;
    }
    /// Finaliza a exploração de um vertice maracndo o tempo de termino e incrementando o clock
    /// `vertice_k` - chave do vertice a ser finalizado
    pub fn finish_exploring(&mut self, vertice_k:i32){
        self.tempo_termino.insert(vertice_k, self.clock);
        self.clock += 1;
    }
    /// Retorna a chave de um vertice ainda não explorado
    /// `g` - grafo utilizado na busca
    pub fn get_unexplored_vertice(&self, g:&DiGraph)-> i32{
        for key in g.get_vertice_key_array() {
            if self.tempo_descoberta.get(&key).is_none() {
                return key;
            }
        }
        -1
    }
    /// checa se uma aresta ja foi explorada
    /// `aresta_key` - chave da aresta a ser classificada
    /// 
    /// ## Retorna
    /// `true` se a aresta ja foi classificada, `false` caso contrario
    pub fn is_aresta_marked(&self, aresta_key:i32) -> bool{
        match self.arestas_marked.get(&aresta_key){
            Some(value) => *value,
            None => false,
        }
    }
    /// Checa se um vertice ja foi visitado
    /// 
    /// um vertice é considerado visitado se o tempo de descoberta ja foi marcado
    /// 
    /// `vertice_key` - chave do vertice a ser verificado
    pub fn already_visited(&self, vertice_key:i32) -> bool{
        self.tempo_descoberta.get(&vertice_key).is_some()
    }
    /// Checa se um vertice ja foi explorado
    /// 
    /// um vertice é considerado explorado se o tempo de termino ja foi marcado
    /// 
    /// `vertice_key` - chave do vertice a ser verificado
    pub fn already_explored(&self, vertice_key:i32) -> bool{
        self.tempo_termino.get(&vertice_key).is_some()
    }

    pub fn classificate_aresta(&mut self, aresta:&Edge, class:DfsClassification){
        self.class_arestas.insert(aresta.clone(), class);
    }

}





