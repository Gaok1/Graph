use std::{
    collections::{btree_map::Keys, HashMap},
    hash::Hash,
};

use crate::graph_lib::{graph::{self, DiGraph}, vertice::{self, Vertice}};

use super::flux_map::FluxMap;

/// Estrutura do algoritmo Push Relable
///
///  `flux_map` define mapa de fluxo para as arestas
///
///  `levels` define o nivel de cada vértice mapeando sua chave para o valor do nivel
///
///  `excess` define o excesso de cada vértice, mapeando sua chave para um valor de excesso
pub struct PushRelable {
    flux_map: FluxMap,
    levels: HashMap<i32, u32>,
    excess: HashMap<i32, i32>,
}

impl PushRelable {
    pub fn from_digraph(g: &DiGraph, s: i32, t: i32) -> Self {
        let push_relable = PushRelable::new(g, s, t);
        let mut vertices_array : Vec<&i32> = g.get_vertice_key_array().iter()
                                                .filter( |vertice| **vertice != s && **vertice != t).collect();
        
        

        push_relable
    }

    pub fn get_v_excesed(&self, vertices : Vec<&i32>) -> Option<i32>{
        for v in vertices{
            if *self.excess.get(v).unwrap() != 0 {
                return  Some(*v)
            }
        }
        None
    }

    pub fn push() {
        todo!()
    }
    
    pub fn relable() {
        todo!()
    }
}



impl PushRelable {
    fn new(g: &DiGraph, s: i32, t: i32) -> PushRelable {
        let flux_map = FluxMap::from_edges(&g.all_edges(), (s, t));
        let mut levels: HashMap<i32, u32> = HashMap::new();
        let mut excess: HashMap<i32, i32> = HashMap::new();
        for v in g.iter_vertices() {
            levels.insert(v.key(), 0);
            excess.insert(v.key(), 0);
        }
        levels.insert(s, g.vertices_length() as u32);

        PushRelable {
            flux_map,
            levels,
            excess,
        }
    }
}
