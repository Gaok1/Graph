use crate::{
    edge::Edge,
    graph,
    tools::inifinity::Infinity,
    vertice::{self, Vertice},
    DiGraph,
};
use std::{collections::HashMap, fmt::Alignment, hash::Hash};

pub struct MinPath {
    cost: HashMap<(i32, i32), Infinity>, // maps (v, w) to cost
    path: HashMap<(i32, i32), Vec<i32>>,
}
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Row, Table};
use Infinity::*;
impl MinPath {
    fn new(g: &DiGraph) -> Self {
        let vertices = g.get_vertice_key_array();
        let mut cost_map: HashMap<(i32, i32), Infinity> = HashMap::new();

        for &v in &vertices {
            for &w in &vertices {
                cost_map.insert((v, w), Infinite);

                if v == w {
                    cost_map.insert((v, w), Number(0));
                }
            }
        }
        for e in g.all_edges() {
            let (v, w) = (e.origin_key(), e.destiny_key());
            cost_map.insert((v, w), Infinity::Number(e.weight()));
        }
        let path = HashMap::new();
        MinPath {
            cost: cost_map,
            path,
        }
    }

    // "Ø";

    fn set_cost(&mut self, edge: (i32, i32), cost: Infinity) {
        self.cost.insert(edge, cost);
    }

    pub fn get_cost(&self, edge: (i32, i32)) -> Infinity {
        *self.cost.get(&edge).unwrap()
    }

    ///  Finds the minor cust to all vertices to each other
    /// # Example
    /// ````
    /// let graph = DiGraph::from_random(5, Some(19), true, true);
    ///
    /// let min_path = MinPath::from_digraph(&graph);
    ///
    ///let vertices = graph.get_vertice_key_array();
    ///for v in vertices.iter() {
    ///  for w in vertices.iter() {
    ///    println!(
    ///      "Menor custo de {v} -> {w} = {}",
    ///     min_path.get_cost((*v, *w))
    ///    );
    ///  }
    // }
    ///
    ///
    /// ```
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

    pub fn to_table(&self, g: &DiGraph) -> Table {
        // Cria uma tabela vazia
        let mut table: Table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // Obtém os vértices do grafo
        let mut vertices: Vec<i32> = g.get_vertice_key_array(); 
        vertices.sort();

        // Define o cabeçalho da tabela com os vértices
        let mut headers: Vec<Cell> = vec![Cell::new(" ").bg(Color::White)]; // Primeiro cabeçalho vazio para alinhar com os rótulos das linhas
        for v in &vertices {
            headers.push(Cell::new(v).fg(Color::Yellow));
        }
        table.set_header(headers);

        // Preenche a tabela com os custos dos caminhos mínimos
        for v in &vertices {
            let mut row = vec![Cell::new(v.to_string()).fg(Color::Cyan)]; // Primeira coluna como rótulo da linha
            for w in &vertices {
                let pair = (*v, *w);
                let cost = self.get_cost(pair);
                row.push(Cell::new(format!("{}", cost)));
            }
            table.add_row(Row::from(row));
        }
        table
    }
    
}