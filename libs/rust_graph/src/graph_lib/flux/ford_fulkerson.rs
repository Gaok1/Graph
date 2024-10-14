use std::{clone, collections::HashMap};

use comfy_table::Color;

use crate::{
    edge::Edge,
    view::{self, GraphPainter},
    DiGraph,
};

/// Define atributos das arestas para a implementação do fluxo maximo
/// de ford fulkerson
#[derive(Debug)]
pub struct EdgeAtt {
    flux: u32,
    capacity: u32,
}
#[allow(unused)]

impl EdgeAtt {
    /// Cria um novo EdgeAtt a partir de uma aresta
    ///
    /// ``flux`` é inicializado com 0
    ///
    /// ``capacity`` é inicializado com o peso da aresta
    pub fn from_edge(e: &Edge) -> Self {
        let flux = 0;

        let capacity: u32 = if e.weight() >= 0 {
            e.weight() as u32
        } else {
            panic!("Edge weight is negative")
        };

        EdgeAtt { flux, capacity }
    }

    pub fn set_flux(&mut self, flux: u32) {
        if self.capacity < flux {
            panic!("Flux is above the capacity")
        }
        self.flux = flux;
    }

    pub fn get_flux(&self) -> u32 {
        self.flux
    }
    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }
    pub fn tuple(&self) -> (u32, u32) {
        (self.flux, self.capacity)
    }
}

/// Implementação da estrutura de grafo residual
///
/// onde temos arestas invertidas por onde passa fluxo
///
/// e arestas diretas onde pode passar fluxo
///
/// `graph` o próprio grafo modificado com as arestas invertidas
///
/// `edgeInverted` mapa de arestas invertidas para true se invertida e false para não invertida
struct ResidualGraph {
    graph: DiGraph,
    edgeInverted: HashMap<(i32, i32), bool>,
}

impl ResidualGraph {
    pub fn from_graph(g: &DiGraph, flux_map: &FluxMap) -> Self {
        let original_graph = g.clone();
        let mut map = HashMap::new();
        let mut edges = Vec::<Edge>::new();

        for e in original_graph.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            let (flux, capacity) = flux_map.get(&(v, w)).unwrap().tuple();
            if flux != 0 {
                //aresta invertida do fluxo
                edges.push(Edge::new_weighted(w, v, (flux) as i32));
                map.insert((w, v), true);
            }
            if capacity - flux != 0 {
                edges.push(Edge::new_weighted(v, w, (capacity - flux) as i32));
                map.insert((v, w), false);
            }
        }
        let graph = DiGraph::from_edges(edges);
        ResidualGraph {
            graph: graph,
            edgeInverted: map,
        }
    }

    pub fn print(&self) {
        let mut painter = GraphPainter::from_digraph(&self.graph);

        for e in self.graph.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            let inverted = self.edgeInverted.get(&(v, w)).unwrap();
            if *inverted {
                painter.update_edge_color(v, w, crate::graph_lib::view::Color::Red);
            } else {
                painter.update_edge_color(v, w, crate::graph_lib::view::Color::Green);
            }
        }
        painter.to_dot_png("residual_graph", "Residual Graph");
    }
}

/// Implementação do mapa de fluxo
///
/// tuple `(i32,i32)` -> key for an edge (v,w)
///
/// `EdgeAtt` -> atributos da aresta
pub struct FluxMap {
    map: HashMap<(i32, i32), EdgeAtt>,
    s_t: (i32, i32), // source and terminal
    max_flux: i32,
}
#[allow(unused)]
impl FluxMap {
    /// Cria um novo mapa de fluxo a partir de um grafo
    ///
    /// todos os valores de fluxo são inicializados com 0
    pub fn from_edges(edges: &Vec<Edge>, s_t: (i32, i32)) -> Self {
        let mut flux_map = HashMap::new();
        for e in edges {
            flux_map.insert((e.origin_key(), e.destiny_key()), EdgeAtt::from_edge(&e));
        }
        FluxMap {
            map: flux_map,
            s_t,
            max_flux: 0,
        }
    }

    pub fn get(&self, key: &(i32, i32)) -> Option<&EdgeAtt> {
        self.map.get(key)
    }
    pub fn get_mut(&mut self, key: &(i32, i32)) -> Option<&mut EdgeAtt> {
        self.map.get_mut(key)
    }
    pub fn set(&mut self, key: (i32, i32), value: EdgeAtt) {
        self.map.insert(key, value);
    }
    pub fn insert(&mut self, key: (i32, i32), value: EdgeAtt) {
        self.map.insert(key, value);
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<(i32, i32), EdgeAtt> {
        self.map.iter()
    }

    pub fn get_max_flux(&self) -> i32 {
        self.max_flux
    }

    pub fn to_png(&self) -> GraphPainter {
        let mut max_val = 0;
        let mut painter = GraphPainter::new();
        let (base, antibase) = self.s_t;
        for ((v, w), att) in self.map.iter() {
            painter.add_edge(*v, *w, None, None);

            if *v == base {
                max_val += att.get_flux();
            }
            if att.get_flux() != 0 {
                painter.update_vertice_color(*v, view::Color::Yellow);
                painter.update_vertice_label(*v, "i".to_owned());
                painter.update_edge_color(*v, *w, view::Color::Green);
            }
            painter.update_edge_label(*v, *w, format!("{}/{}", att.get_flux(), att.get_capacity()));
        }

        painter.update_vertice_color(base, view::Color::Green);
        painter.update_vertice_color(antibase, view::Color::Red);
        painter.update_vertice_label(base, "S".to_owned());
        painter.update_vertice_label(antibase, "T".to_owned());

        painter
    }
}
#[allow(unused)]
pub fn max_flux(g: &DiGraph, s: i32, t: i32) -> FluxMap {
    let mut flux_map = FluxMap::from_edges(&g.all_edges(), (s, t));

    let mut rede_residual = ResidualGraph::from_graph(g, &flux_map);

    let mut increasing_path = IncreasingPath::fromResidualGraph(&rede_residual, s, t);

    while let Some(ref path) = increasing_path {
        let gargalo = path.gargalo;
        for e in path.edges.clone() {
            let (v, w) = (e.origin_key(), e.destiny_key());

            let att = {
                if let Some(att) = flux_map.get_mut(&(v, w)) {
                    att
                } else {
                    flux_map.get_mut(&(w, v)).unwrap() //aresta inversa não está no mapa
                    // mas o que importa é o fluxo e capacidade da aresta original
                }
            };
            if *rede_residual.edgeInverted.get(&(v, w)).unwrap() {
                att.set_flux(att.get_flux() - gargalo);
            } else {
                att.set_flux(att.get_flux() + gargalo);
            }
        }
        rede_residual = ResidualGraph::from_graph(g, &flux_map);
        increasing_path = IncreasingPath::fromResidualGraph(&rede_residual, s, t);
    }

    for e in g.all_edges() {
        let (v, w) = (e.origin_key(), e.destiny_key());
        let att = flux_map.get(&(v, w)).unwrap();
        if v == s {
            flux_map.max_flux += (att.get_flux()) as i32;
        }
    }

    flux_map
}

struct IncreasingPath {
    gargalo: u32,
    edges: Vec<(Edge)>, // edges of the path
}

impl IncreasingPath {
    fn fromResidualGraph(g: &ResidualGraph, s: i32, t: i32) -> Option<Self> {
        let path = g.graph.path_between(s, t);
        let Some(path) = path else {
            return None; // não existe caminho entre s e t aumentante
        };
        let mut gargalo = u32::MAX;
        for e in path.iter() {
            if (e.weight() as u32) < gargalo {
                gargalo = e.weight() as u32;
            }
        }
        Some(IncreasingPath {
            gargalo,
            edges: path,
        })
    }
}
