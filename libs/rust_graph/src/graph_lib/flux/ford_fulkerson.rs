use std::{collections::HashMap, fmt::format};

use comfy_table::Color;

use crate::{
    graph_lib::edge::Edge,
    graph_lib::view::{self, GraphPainter},
    DiGraph,
};

/// Define atributos das arestas para a implementação do fluxo máximo
/// de Ford-Fulkerson
#[derive(Debug, Clone)]
pub struct EdgeAtt {
    flux: u32,
    capacity: u32,
}

impl EdgeAtt {
    /// Cria um novo `EdgeAtt` a partir de uma aresta
    ///
    /// `flux` é inicializado com 0
    ///
    /// `capacity` é inicializado com o peso da aresta
    ///
    /// # Panics
    ///
    /// Se ``e.weight() < 0``
    pub fn from_edge(e: &Edge) -> Self {
        let flux = 0;
        let capacity: u32 = if e.weight() >= 0 {
            e.weight() as u32
        } else {
            panic!("Edge weight is negative")
        };
        EdgeAtt { flux, capacity }
    }

    /// Define o fluxo da aresta
    ///
    /// Panica se o fluxo exceder a capacidade
    pub fn set_flux(&mut self, flux: u32) {
        if self.capacity < flux {
            panic!("Flux is above the capacity");
        }
        self.flux = flux;
    }

    /// Obtém o fluxo da aresta
    pub fn get_flux(&self) -> u32 {
        self.flux
    }

    /// Obtém a capacidade da aresta
    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    /// Retorna uma tupla (fluxo, capacidade)
    pub fn tuple(&self) -> (u32, u32) {
        (self.flux, self.capacity)
    }
}

/// Implementação da estrutura de grafo residual
///
/// Onde temos arestas invertidas por onde passa fluxo
///
/// E arestas diretas onde pode passar fluxo
///
/// `graph` o próprio grafo modificado com as arestas invertidas
///
/// `edge_inverted` mapa de arestas invertidas para true se invertida e false para não invertida
pub struct ResidualGraph {
    graph: DiGraph,
    edge_inverted: HashMap<Edge, bool>, // Mapa Edge para bool indicando se está invertida
}

impl ResidualGraph {
    /// Cria um grafo residual a partir de um grafo original e um mapa de fluxo
    pub fn from_graph(g: &DiGraph, flux_map: &FluxMap) -> Self {
        let mut map: HashMap<Edge, bool> = HashMap::new();
        let mut edges = Vec::<Edge>::new();

        for e in g.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            // Obter os atributos da aresta (v, w)
            let Some(att) = flux_map.get(&(v, w)) else {
                panic!("Aresta ({}, {}) não encontrada no FluxMap", v, w);
            };
            let (flux, capacity) = att.tuple();

            if flux > 0 {
                // Adiciona a aresta invertida
                let inverted_edge = Edge::new_weighted(w, v, flux as i32);
                edges.push(inverted_edge.clone());
                map.insert(inverted_edge, true);
            }

            if capacity > flux {
                // Adiciona a aresta residual
                let residual_edge = Edge::new_weighted(v, w, (capacity - flux) as i32);
                edges.push(residual_edge.clone());
                map.insert(residual_edge, false);
            }
        }

        let graph = DiGraph::from_edges(edges);
        ResidualGraph {
            graph,
            edge_inverted: map,
        }
    }

    /// Converte o grafo residual para um `GraphPainter` para visualização
    pub fn to_graph_painter(&self) -> GraphPainter {
        let mut painter = GraphPainter::from_digraph(&self.graph);

        for e in self.graph.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            if let Some(is_inverted) = self.edge_inverted.get(&e) {
                if *is_inverted {
                    painter.update_edge_color(v, w, view::Color::Red);
                } else {
                    painter.update_edge_color(v, w, view::Color::Green);
                }
            }
        }
        painter
    }
}

/// Implementação do mapa de fluxo
///
/// tuple `(i32, i32)` -> chave para uma aresta (v, w)
///
/// `EdgeAtt` -> atributos da aresta
pub struct FluxMap {
    map: HashMap<(i32, i32), EdgeAtt>, // Mapa (v, w) para atributos de aresta
    s_t: (i32, i32),                   // source e terminal
    max_flux: i32,
}

impl FluxMap {
    /// Cria um novo mapa de fluxo a partir de um grafo
    ///
    /// Todos os valores de fluxo são inicializados com 0
    pub fn from_edges(edges: &Vec<Edge>, s_t: (i32, i32)) -> Self {
        let mut flux_map = HashMap::new();
        for e in edges {
            let (v, w) = (e.origin_key(), e.destiny_key());
            flux_map.insert((v, w), EdgeAtt::from_edge(e));
        }
        FluxMap {
            map: flux_map,
            s_t,
            max_flux: 0,
        }
    }

    /// Obtém uma referência imutável para os atributos de uma aresta
    pub fn get(&self, key: &(i32, i32)) -> Option<&EdgeAtt> {
        self.map.get(key)
    }

    /// Obtém uma referência mutável para os atributos de uma aresta
    pub fn get_mut(&mut self, key: &(i32, i32)) -> Option<&mut EdgeAtt> {
        self.map.get_mut(key)
    }

    /// Obtém o fluxo máximo
    pub fn get_max_flux(&self) -> i32 {
        self.max_flux
    }

    /// Converte o mapa de fluxo para um `GraphPainter` para visualização
    pub fn to_graph_painter(&self) -> GraphPainter {
        let mut painter = GraphPainter::new();
        let (base, antibase) = self.s_t;

        for ((v, w), att) in self.map.iter() {
            painter.add_edge(*v, *w, None, None);
            if att.get_flux() != 0 {
                painter.update_vertice_color(*v, view::Color::Yellow);
                let vertice_label = format!("{} - i", v);
                painter.update_vertice_label(*v, vertice_label);
                painter.update_edge_color(*v, *w, view::Color::Green);
            }
            painter.update_edge_label(*v, *w, format!("{}/{}", att.get_flux(), att.get_capacity()));
        }
        painter.update_vertice_color(base, view::Color::Green);
        painter.update_vertice_color(antibase, view::Color::Red);
        let vertice_label = format!("{} - S", base);
        painter.update_vertice_label(base, vertice_label);
        let vertice_label = format!("{} - T", antibase);
        painter.update_vertice_label(antibase, vertice_label);
        painter
    }

    /// Retorna as arestas utilizadas com seus respectivos fluxos
    pub fn get_used_edges(&self) -> Vec<(Edge, EdgeAtt)> {
        let mut edges = vec![];
        for ((v, w), att) in self.map.iter() {
            if att.get_flux() > 0 {
                let e = Edge::new_weighted(*v, *w, att.get_capacity() as i32);
                edges.push((e, att.clone()));
            }
        }
        edges
    }
}

pub fn max_flux_multi_s_t(
    g: &DiGraph,
    sources: Vec<i32>,
    terminals: Vec<i32>,
) -> (FluxMap, ResidualGraph) {
    let mut graph = g.clone();

    let max_source = graph.unused_v_key_from(graph.vertices_length() as i32);
    graph.add_vertice(max_source);

    let max_terminal = graph.unused_v_key_from(max_source + 1);
    graph.add_vertice(max_terminal);

    for &s in sources.iter() {
        let mut edge_weight = 0;
        for edge in graph.edges_of(s).iter().flatten() {
            edge_weight += edge.weight();
        }
        graph.add_edge_weighted(max_source, s, edge_weight);
    }

    for &t in terminals.iter() {
        let mut edge_weight = 0;
        for p in graph.predecessor_edges(t).iter().flatten() {
            edge_weight += p.weight()
        }
        graph.add_edge_weighted(t, max_terminal, edge_weight);
    }

    max_flux(&graph, max_source, max_terminal)
}

#[allow(unused)]
pub fn max_flux(g: &DiGraph, s: i32, t: i32) -> (FluxMap, ResidualGraph) {
    let mut flux_map = FluxMap::from_edges(&g.all_edges(), (s, t));

    let mut residual_graph = ResidualGraph::from_graph(g, &flux_map);

    let mut increasing_path = IncreasingPath::from_residual_graph(&residual_graph, s, t);

    while let Some(ref path) = increasing_path {
        let gargalo = path.gargalo;
        for e in &path.edges {
            let (v, w) = (e.origin_key(), e.destiny_key());

            let is_inverted = residual_graph
                .edge_inverted
                .get(e)
                .expect("Aresta não encontrada no ResidualGraph");

            let key = if *is_inverted { (w, v) } else { (v, w) };

            let Some(att) = flux_map.get_mut(&key) else {
                panic!("Edge ({}, {}) not found in FluxMap", key.0, key.1);
            };

            if *is_inverted {
                // Reduz o fluxo na aresta original
                att.set_flux(att.get_flux() - gargalo);
            } else {
                // Aumenta o fluxo na aresta original
                att.set_flux(att.get_flux() + gargalo);
            }
        }

        residual_graph = ResidualGraph::from_graph(g, &flux_map);
        increasing_path = IncreasingPath::from_residual_graph(&residual_graph, s, t);
    }

    // Calcula o fluxo máximo a partir das arestas saindo da fonte
    let vertice = g
        .get_vertice_arc(s)
        .expect("Vértice de origem não encontrado");
    let binding = vertice.read().unwrap();
    let edges = binding.edges_vec_ref();
    for e in edges {
        let (v, w) = (e.origin_key(), e.destiny_key());
        if let Some(att) = flux_map.get(&(v, w)) {
            flux_map.max_flux += att.get_flux() as i32;
        }
    }

    (flux_map, residual_graph)
}

struct IncreasingPath {
    gargalo: u32,
    edges: Vec<Edge>, // Arestas do caminho
}

impl IncreasingPath {
    fn from_residual_graph(g: &ResidualGraph, s: i32, t: i32) -> Option<Self> {
        let path = g.graph.path_between(s, t)?;
        let mut gargalo = u32::MAX;
        for e in &path {
            let weight = e.weight() as u32;
            if weight < gargalo {
                gargalo = weight;
            }
        }
        Some(IncreasingPath {
            gargalo,
            edges: path,
        })
    }
}
