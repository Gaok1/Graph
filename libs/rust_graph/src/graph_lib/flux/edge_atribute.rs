use crate::graph_lib::edge::Edge;

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