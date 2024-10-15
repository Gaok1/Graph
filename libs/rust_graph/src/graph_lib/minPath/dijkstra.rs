use std::collections::HashMap;

use crate::{
    graph, tools::{heap::HeapMin, inifinity::Infinity}, vertice, DiGraph
};

#[derive(Clone, Copy)]
struct VerticeDist(i32, Infinity);

impl VerticeDist {
    pub fn vertice(&self) -> i32 {
        self.0
    }
    pub fn dist(&self) -> Infinity {
        self.1
    }

    pub fn set_dist(&mut self, dist: Infinity) {
        self.1 = dist;
    }
}

pub struct Dijkstra {
    pred: HashMap<i32, i32>,
    dist: HashMap<i32, Infinity>,
}
use Infinity::{Infinite, Number};
impl Dijkstra {
    pub fn new() -> Dijkstra {
        Dijkstra {
            pred: HashMap::new(),
            dist: HashMap::new(),
        }
    }

    pub fn new_sized(size: usize) -> Dijkstra {
        let mut data = Dijkstra {
            pred: HashMap::with_capacity(size),
            dist: HashMap::with_capacity(size),
        };
        data
    }

    pub fn pred(&self) -> &HashMap<i32, i32> {
        &self.pred
    }
    pub fn dist(&self) -> &HashMap<i32, Infinity> {
        &self.dist
    }

    pub fn shortest_path(g: &DiGraph, v_key: i32) -> Self {
        let mut data = Dijkstra::new_sized(g.vertices_length() as usize);
        let mut queue = HeapMin::new(|a: &VerticeDist, b: &VerticeDist| a.dist().cmp(&b.dist()));
        data.dist.insert(v_key, Infinity::new(0));
        queue.insert((VerticeDist(v_key, Infinity::new(0))));

        for v in g.iter_vertices() {
            let v = v.read().unwrap();
            if v.key() != v_key {
                data.dist.insert(v.key(), Infinite);
                data.pred.insert(v.key(), -1);
            }
        }


        while !queue.empty() {
            let v = queue.pop().unwrap().0;
            let vertice = g.get_vertice_arc(v);
            if vertice.is_none(){
                panic!("Vertice does not exist in graph {v}");
            }
            let vertice = vertice.unwrap();

            let v = vertice.read().unwrap();
            for e in v.edges_vec_ref() {
                let w = e.destiny_key();
                let v_d = *data.dist.get(&v.key()).unwrap();
                let w_d = *data.dist.get(&w).unwrap();
                if w_d > (v_d + Number(e.weight())) {
                    println!("Relaxando d[{}] = {} com d[{}] = {} + w(v,w) = {}",w, w_d, v.key(),v_d, e.weight());
                    data.dist.insert(w, Number(v_d.unwrap() + e.weight()));
                    data.pred.insert(w, v.key());
                    queue.insert(VerticeDist(w, Number(v_d.unwrap() + e.weight())));
                }
            }
        }
        data
    }
}
