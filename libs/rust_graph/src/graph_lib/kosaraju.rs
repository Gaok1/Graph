

use std::{cell::RefCell, rc::Rc};

use crate::DiGraph;

use super::busca::*;

///Kosaraju method to find conex components
#[derive(Debug)]
pub struct ConexComponents {
    len: usize,
    components : Vec<Rc<RefCell<DiGraph>>>,
}

impl ConexComponents{
    pub fn new() -> ConexComponents{
        ConexComponents{
            len:0,
            components : Vec::new(),
        }
    }

    pub fn from_dfsData(dfs_data : &mut DfsStruct) -> ConexComponents{
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

