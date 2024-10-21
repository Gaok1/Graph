

use std::{cell::RefCell, rc::Rc};

use crate::DiGraph;

use super::busca::{DeepFirstSearch, DfsStruct};



///Kosaraju method to find conex components
#[derive(Debug)]
pub struct ConexComponents {
    len: usize,
    components : Vec<Rc<RefCell<DiGraph>>>,
}
#[allow(unused)]
impl ConexComponents{
    pub fn new() -> ConexComponents{
        ConexComponents{
            len:0,
            components : Vec::new(),
        }
    }

    fn from_dfsData(dfs_data : &mut DfsStruct) -> ConexComponents{
        let mut len = 0;
        let mut components = ConexComponents::new();

        for digraph in dfs_data.trees.iter_mut() {
            let graph = digraph.clone();
            components.components.push(graph);
            len+=1;
        }
        components.len = len;
        components
    }

    pub fn clone_components(&self)->Vec<Rc<RefCell<DiGraph>>> {
        self.components.clone()
    }
    
}


pub trait Kosaraju {
    fn conex_components(&self) -> ConexComponents;
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

