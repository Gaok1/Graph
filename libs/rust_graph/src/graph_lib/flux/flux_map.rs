use std::collections::HashMap;

use crate::graph_lib::{edge::Edge, view::{self, GraphPainter}};

use super::edge_atribute::EdgeAtt;


// Implementação do mapa de fluxo
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

    pub fn set_max_flux(&mut self, max_flux: i32) {
        self.max_flux = max_flux;
    }

    /// Converte o mapa de fluxo para um `GraphPainter` para visualização
    pub fn to_graph_painter(&self) -> GraphPainter {
        let mut painter = GraphPainter::new();
        let (base, antibase) = self.s_t;

        for ((v, w), att) in self.map.iter() {
            painter.add_edge(*v, *w, None, None);
            if att.get_flux() != 0 {
                painter.update_vertice_color(*v, view::Color::Yellow);
                let vertice_label = format!("{}", v);
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