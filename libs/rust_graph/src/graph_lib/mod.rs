pub mod busca;
pub mod graph;
pub mod kosaraju;
pub mod vertice;
pub mod edge;
pub mod minPath;
pub mod view;


struct Color {
    r : u8,
    g : u8,
    b : u8,
}

struct Vertice {
    key : i32,
    color : Color,
}