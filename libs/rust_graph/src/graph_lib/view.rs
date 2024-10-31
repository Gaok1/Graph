use crate::DiGraph;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    Orange,
    Purple,
    Pink,
    Brown,
    Lime,
    Indigo,
    Violet,
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
            Color::Orange => String::from("orange"),
            Color::Purple => String::from("purple"),
            Color::Pink => String::from("pink"),
            Color::Brown => String::from("brown"),
            Color::Lime => String::from("lime"),
            Color::Indigo => String::from("indigo"),
            Color::Violet => String::from("violet"),
        }
    }

    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 14] = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Cyan,
            Color::Magenta,
            Color::Yellow,
            Color::Grey,
            Color::Orange,
            Color::Purple,
            Color::Pink,
            Color::Brown,
            Color::Lime,
            Color::Indigo,
            Color::Violet,
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
    label: String,
    color: Color,
}

impl Vertice {
    pub fn new(key: i32, color: Color) -> Vertice {
        Vertice {
            key,
            label: key.to_string(), // Por padrão, a label é a key como String
            color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn get_key(&self) -> i32 {
        self.key
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}

struct Edge {
    origin_key: i32,
    destiny_key: i32,
    label: Option<String>,
    color: Color,
}

impl Edge {
    pub fn new_colored(origin_vertice: i32, destiny_vertice: i32, color: Color) -> Edge {
        Edge {
            origin_key: origin_vertice,
            destiny_key: destiny_vertice,
            label: None,
            color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn default_color() -> Color {
        Color::Black
    }

    pub fn get_origin(&self) -> i32 {
        self.origin_key
    }

    pub fn get_destiny(&self) -> i32 {
        self.destiny_key
    }

    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
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

    /// Adiciona um vértice ao grafo.
    pub fn add_vertice(&mut self, key: i32, color: Option<Color>) {
        self.vertices
            .insert(key, Vertice::new(key, color.unwrap_or_default()));
    }

    /// Atualiza o label de um vértice.
    pub fn update_vertice_label(&mut self, key: i32, label: String) {
        if let Some(vertice) = self.vertices.get_mut(&key) {
            vertice.set_label(label);
        }
    }

    /// Atualiza a cor de um vértice.
    pub fn update_vertice_color(&mut self, key: i32, color: Color) {
        if let Some(vertice) = self.vertices.get_mut(&key) {
            vertice.set_color(color);
        }
    }

    /// Adiciona uma aresta ao grafo.
    pub fn add_edge(
        &mut self,
        origin: i32,
        destiny: i32,
        label: Option<String>,
        color: Option<Color>,
    ) {
        if self.vertices.get(&origin).is_none() {
            self.add_vertice(origin, None);
        }
        if self.vertices.get(&destiny).is_none() {
            self.add_vertice(destiny, None);
        }

        let mut edge = Edge::new_colored(origin, destiny, color.unwrap_or(Edge::default_color()));
        if let Some(l) = label {
            edge.set_label(l);
        }
        self.edges.insert((origin, destiny), edge);
    }

    /// Atualiza a cor de uma aresta.
    pub fn update_edge_color(&mut self, origin: i32, destiny: i32, color: Color) {
        if let Some(edge) = self.edges.get_mut(&(origin, destiny)) {
            edge.set_color(color);
        }
    }

    /// Atualiza o label de uma aresta.
    pub fn update_edge_label(&mut self, origin: i32, destiny: i32, label: String) {
        if let Some(edge) = self.edges.get_mut(&(origin, destiny)) {
            edge.set_label(label);
        }
    }

    /// Remove todas as arestas com uma cor específica.
    pub fn remove_edges_by_color(&mut self, color: Color) {
        self.edges.retain(|_, edge| edge.color != color);
    }

    /// Remove um vértice e todas as arestas associadas a ele.
    pub fn remove_vertice(&mut self, key: i32) -> Result<(), &'static str> {
        if self.vertices.remove(&key).is_some() {
            self.edges
                .retain(|&(origin, destiny), _| origin != key && destiny != key);
            Ok(())
        } else {
            Err("Vértice não encontrado")
        }
    }

    /// Remove uma aresta específica.
    pub fn remove_edge(&mut self, origin: i32, destiny: i32) -> Result<(), &'static str> {
        if self.edges.remove(&(origin, destiny)).is_some() {
            Ok(())
        } else {
            Err("Aresta não encontrada")
        }
    }

    /// Gera a representação DOT do grafo.
    pub fn to_dot(&self, title: &str) -> String {
        let mut dot = String::from("digraph G {\n");
        dot.push_str("layout=dot;\n");
        dot.push_str("node [shape=circle];\n");
        dot.push_str("edge [dir=forward];\n");
        dot.push_str(format!("label=\"{}\";\n", title).as_str()); // Adiciona o título
        dot.push_str("labelloc=\"t\";\n"); // Posição do título no topo
        dot.push_str("fontsize=20;\n"); // Define o tamanho da fonte do título
        for vertice in self.vertices.values() {
            dot.push_str(&format!(
                "{} [label=\"{}\", style=filled, fillcolor=\"{}\"];\n",
                vertice.key,
                vertice.label,
                vertice.color.to_dot_color()
            ));
        }

        for edge in self.edges.values() {
            let label = edge.label.as_deref().unwrap_or("");
            dot.push_str(&format!(
                "{} -> {} [color=\"{}\", label=\"{}\"];\n",
                edge.origin_key,
                edge.destiny_key,
                edge.color.to_dot_color(),
                label
            ));
        }

        dot.push_str("}");
        dot
    }

    /// Gera uma imagem PNG a partir da representação DOT do grafo.
    pub fn to_png(&self, file_path: &str, title: &str) {
        let dot = self.to_dot(title);
        let dot_file = format!("{}.dot", file_path);
        let png_file = format!("{}.png", file_path);

        // Salva o arquivo DOT
        fs::write(&dot_file, dot).expect("Erro ao escrever arquivo DOT");

        // Usa o Graphviz para converter DOT em PNG
        let output = Command::new("dot")
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

    /// Cria um `GraphPainter` a partir de um `DiGraph`.
    pub fn from_digraph(g: &DiGraph) -> Self {
        let mut graph = Self::new();
        for v in g.iter_vertices() {
            graph.add_vertice(v.key(), None);
            for e in v.edges_vec() {
                graph.add_edge(v.key(), e.destiny_key(), Some(e.weight().to_string()), None);
            }
        }
        graph
    }
    /// Creates a `GraphPainter` from a `DiGraph` with a title and saves it as a PNG file.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to be drawn.
    ///
    /// * `file_path` - The path to save the PNG file.
    ///
    /// * `title` - The title of the graph.
    pub fn draw(graph: &DiGraph, file_path: &str, title: &str) {
        let painter = GraphPainter::from_digraph(graph);
        painter.to_png(file_path, title);
    }
}
