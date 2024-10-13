use crate::{
    edge::Edge,
    tools::inifinity::Infinity,
    vertice::{self, Vertice},
    DiGraph,
};
use std::collections::HashMap;

pub struct MinPath {
    min_path: HashMap<(i32, i32), Infinity>, // maps (v, w) to cost
}
use Infinity::*;
impl MinPath {
    fn new(g: &DiGraph) -> Self {
        let vertices = g.get_vertice_key_array();
        let mut matrix: HashMap<(i32, i32), Infinity> = HashMap::new();

        for &v in &vertices {
            for &w in &vertices {
                matrix.insert((v, w), Infinite);

                if v == w {
                    matrix.insert((v, w), Number(0));
                }
            }
        }
        for e in g.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            matrix.insert((v, w), Infinity::Number(e.weight()));
        }
        MinPath { min_path: matrix }
    }

    fn set_cost(&mut self, edge: (i32, i32), cost: Infinity) {
        self.min_path.insert(edge, cost);
    }

    pub fn get_cost(&self, edge: (i32, i32)) -> Infinity {
        *self.min_path.get(&edge).unwrap()
    }

    ///  Finds the minor cust to all vertices to each other
    pub fn from_digraph(g: &DiGraph) -> Self {
        let mut cost_map = MinPath::new(g);
        let vertices = g.get_vertice_key_array();

        for k in vertices.iter() {
            for v in vertices.iter() {
                for w in vertices.iter() {
                    let v_w_cost = cost_map.get_cost((*v, *w));
                    let v_k_w_cost = cost_map.get_cost((*v, *k)) + cost_map.get_cost((*k, *w));
                    if v_w_cost > v_k_w_cost {
                        cost_map.set_cost((*v, *w), v_k_w_cost);
                    }
                }
            }
        }
        cost_map
    }
}
