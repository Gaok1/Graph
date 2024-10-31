use std::{collections::HashMap, fmt::format, hash::Hash};

use comfy_table::Color;

use crate::{
    graph_lib::edge::Edge,
    graph_lib::view::{self, GraphPainter},
    DiGraph,
};

use super::flux_map::FluxMap;



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
    original_edge: HashMap<Edge, Edge>, // Mapa Edge residual para Edge original
}

impl ResidualGraph {
    /// Cria um grafo residual a partir de um grafo original e um mapa de fluxo
    fn from_graph(g: &DiGraph, flux_map: &FluxMap) -> Self {
        let mut map: HashMap<Edge, bool> = HashMap::new();
        let mut original_edge: HashMap<Edge, Edge> = HashMap::new();
        let mut edges = Vec::<Edge>::new();

        for e in g.all_edges() {
            let (v, w) = e.v_w();
            // Obter os atributos da aresta (v, w)
            let Some(att) = flux_map.get(&(v, w)) else {
                panic!("Aresta ({}, {}) não encontrada no FluxMap", v, w);
            };
            let (flux, capacity) = att.tuple();

            if flux > 0 {
                // Adiciona a aresta invertida
                let inverted_edge = Edge::new_weighted(w, v, flux as i32);
                edges.push(inverted_edge.clone());
                map.insert(inverted_edge.clone(), true);
                original_edge.insert(inverted_edge, e.clone());
            }

            if capacity > flux {
                // Adiciona a aresta residual
                let residual_edge = Edge::new_weighted(v, w, (capacity - flux) as i32);
                edges.push(residual_edge.clone());
                map.insert(residual_edge.clone(), false);
                original_edge.insert(residual_edge, e.clone());
            }
        }

        let graph = DiGraph::from_edges(edges);
        ResidualGraph {
            graph,
            edge_inverted: map,
            original_edge,
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
        graph.add_edge(Edge::new_weighted(s, max_source, edge_weight));
    }

    for &t in terminals.iter() {
        let mut edge_weight = 0;
        for p in graph.predecessor_edges(t).iter().flatten() {
            edge_weight += p.weight()
        }
        graph.add_edge(Edge::new_weighted(t, max_terminal, edge_weight));
    }

    max_flux(&graph, max_source, max_terminal)
}

#[allow(unused)]
pub fn max_flux(g: &DiGraph, s: i32, t: i32) -> (FluxMap, ResidualGraph) {
    let mut flux_map = FluxMap::from_edges(&g.all_edges(), (s, t));

    let mut residual_graph = ResidualGraph::from_graph(g, &flux_map);

    let mut increasing_path = IncreasingPath::from_residual_graph(&residual_graph, s, t);

    // se tiver um caminho aumentante
    while let Some(ref path) = increasing_path {
        for e in &path.edges {
            let (v, w) = (e.v_w());
            //flag se a aresta residual é invertida
            let is_inverted = *residual_graph
                .edge_inverted
                .get(e)
                .expect("Aresta não encontrada no ResidualGraph");
            //buscar a aresta correspondente no grafo original
            let original_edge = residual_graph
                .original_edge
                .get(e)
                .expect("Aresta não encontrada no ResidualGraph");

            // pega o objeto atributo da aresta para modificações
            let att = flux_map
                .get_mut(&original_edge.v_w())
                .expect("Aresta original não encontrada no FluxMap");

            if is_inverted {
                // Reduz o fluxo na aresta original se for invertida
                att.set_flux(att.get_flux() - path.gargalo);
            } else {
                // Aumenta o fluxo na aresta original 
                att.set_flux(att.get_flux() + path.gargalo);
            }
        }
        residual_graph = ResidualGraph::from_graph(g, &flux_map);
        increasing_path = IncreasingPath::from_residual_graph(&residual_graph, s, t);
    }

    // Calcula o fluxo máximo a partir das arestas saindo da fonte
    let vertice = g
        .get_vertice_arc(s)
        .expect("Vértice de origem não encontrado");
    let edges = vertice.edges_vec_ref();
    for e in edges {
        let (v, w) = (e.origin_key(), e.destiny_key());
        if let Some(att) = flux_map.get(&(v, w)) {
            flux_map.set_max_flux(att.get_flux() as i32);
        }
    }

    (flux_map, residual_graph)
}

struct IncreasingPath {
    gargalo: u32,
    /// arestas do caminho
    edges: Vec<Edge>,
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
