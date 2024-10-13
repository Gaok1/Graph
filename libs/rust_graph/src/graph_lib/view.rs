use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::slice::Iter;
use crate::DiGraph;
#[derive(Debug, PartialEq, Eq, Hash, Clone,Copy)]







pub enum Color {
    Rgb(u8, u8, u8),
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
    Grey,
    Black,
}

impl Color {
    pub fn to_dot_color(&self) -> String {
        match *self {
            Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            Color::Red => String::from("red"),
            Color::Green => String::from("green"),
            Color::Blue => String::from("blue"),
            Color::Cyan => String::from("cyan"),
            Color::Magenta => String::from("magenta"),
            Color::Yellow => String::from("yellow"),
            Color::White => String::from("white"),
            Color::Grey => String::from("grey"),
            Color::Black => String::from("black"),
        }
    }

    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 9] = [  // Exemplo para RGB
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Cyan,
            Color::Magenta,
            Color::Yellow,
            Color::White,
            Color::Grey,
            Color::Black,
        ];
        COLORS.iter()
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

struct Vertice {
    key: i32,
    color: Color,
}

impl Vertice {
    pub fn new(key: i32, color: Color) -> Vertice {
        Vertice { key, color }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

struct Edge {
    origin_key: i32,
    destiny_key: i32,
    weight: Option<i32>,
    color: Color,
}

impl Edge {
    pub fn new_colored(origin_vertice: i32, destiny_vertice: i32, color: Color) -> Edge {
        Edge {
            origin_key: origin_vertice,
            destiny_key: destiny_vertice,
            weight: None,
            color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_weight(&mut self, weight: i32) {
        self.weight = Some(weight);
    }

    pub fn default_color() -> Color {
        Color::Black
    }
}

pub struct GraphPainter {
    vertices: HashMap<i32, Vertice>,
    edges: HashMap<(i32, i32), Edge>,
}

impl GraphPainter {
    pub fn new() -> GraphPainter {
        GraphPainter {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertice(&mut self, key: i32, color: Option<Color>) {
        self.vertices
            .insert(key, Vertice::new(key, color.unwrap_or_default()));
    }

    pub fn add_edge(&mut self, origin: i32, destiny: i32, weight: Option<i32>, color: Option<Color>) {
        if self.vertices.get(&origin).is_none() {
            self.add_vertice(origin, None);
        }
        if self.vertices.get(&destiny).is_none() {
            self.add_vertice(destiny, None);
        }

        let mut edge = Edge::new_colored(origin, destiny, color.unwrap_or(Edge::default_color()));
        if let Some(w) = weight {
            edge.set_weight(w);
        }
        self.edges.insert((origin, destiny), edge);
    }

    // Método para atualizar a cor de um vértice
    pub fn update_vertice_color(&mut self, key: i32, color: Color) -> Result<(), &'static str> {
        if let Some(vertice) = self.vertices.get_mut(&key) {
            vertice.set_color(color);
            Ok(())
        } else {
            Err("Vértice não encontrado")
        }
    }

    // Método para atualizar a cor de uma aresta
    pub fn update_edge_color(&mut self, origin: i32, destiny: i32, color: Color) -> Result<(), &'static str> {
        if let Some(edge) = self.edges.get_mut(&(origin, destiny)) {
            edge.set_color(color);
            Ok(())
        } else {
            Err("Aresta não encontrada")
        }
    }

    // Função para remover todas as arestas de uma determinada cor
    pub fn remove_edges_by_color(&mut self, color: Color) {
        // Itera sobre as arestas e remove aquelas que possuem a cor fornecida
        self.edges.retain(|_, edge| edge.color != color);
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        dot.push_str("layout=dot;\n");  // Define o layout como 'dot' (árvore)
        dot.push_str("node [shape=circle];\n");
        dot.push_str("edge [dir=forward];\n");  // Define a direção das arestas
    
        // Adiciona vértices com cores
        for vertice in self.vertices.values() {
            dot.push_str(&format!(
                "{} [style=filled, fillcolor=\"{}\"];\n",
                vertice.key,
                vertice.color.to_dot_color()
            ));
        }
    
        // Adiciona arestas com cores e pesos (se houver)
        for edge in self.edges.values() {
            let weight_label = if let Some(weight) = edge.weight {
                format!("label=\"{}\"", weight)
            } else {
                String::new()
            };
            dot.push_str(&format!(
                "{} -> {} [color=\"{}\"{}];\n",
                edge.origin_key,
                edge.destiny_key,
                edge.color.to_dot_color(),
                if !weight_label.is_empty() {
                    format!(", {}", weight_label)
                } else {
                    String::new()
                }
            ));
        }
    
        dot.push_str("}");
        dot
    }
    

    pub fn to_dot_png(&self, file_path: &str) {
        let dot = self.to_dot();
        let dot_file = format!("{}.dot", file_path);
        let png_file = format!("{}.png", file_path);

        // Salva o arquivo DOT
        fs::write(&dot_file, dot).expect("Erro ao escrever arquivo DOT");

        // Usa o Graphviz para converter DOT em PNG
        let output = std::process::Command::new("dot")
            .arg("-Tpng")
            .arg(&dot_file)
            .arg("-o")
            .arg(&png_file)
            .output()
            .expect("Erro ao gerar imagem PNG");

        if !output.status.success() {
            eprintln!(
                "Erro ao gerar imagem PNG: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        
        // Deleta o arquivo DOT após gerar a imagem PNG
        fs::remove_file(&dot_file).expect("Erro ao deletar arquivo DOT");
    }

    pub fn from_digraph(g: &DiGraph) -> Self {
        let mut graph = Self::new();
        for v in g.iter_vertices() {
            let v = v.read().unwrap();
            graph.add_vertice(v.key(), None);
            for e in v.edges_borrow() {
                graph.add_edge(v.key(), e.destiny_key(), Some(e.weight()), None);
            }
        }
        graph
    }
}
