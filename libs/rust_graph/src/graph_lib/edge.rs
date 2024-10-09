use std::sync::atomic::AtomicUsize;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    destiny_key: i32,
    origin_key: i32,
    weight: i32,
    id: usize,
}
#[allow(unused)]
impl Edge {
    // Utilizando AtomicUsize para garantir a thread-safety do contador de arestas
    fn next_id() -> usize {
        static EDGE_COUNTER: AtomicUsize = AtomicUsize::new(0);
        EDGE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn new(origin_vertice: i32, destiny_vertice: i32) -> Edge {
        Edge {
            id: Self::next_id(),
            destiny_key: destiny_vertice,
            origin_key: origin_vertice,
            weight: 1,
        }
    }

    pub fn new_weighted(origin_vertice: i32, destiny_vertice: i32, weight: i32) -> Edge {
        Edge {
            id: Self::next_id(),
            destiny_key: destiny_vertice,
            origin_key: origin_vertice,
            weight,
        }
    }

    pub fn destiny_key(&self) -> i32 {
        self.destiny_key
    }
    pub fn origin_key(&self) -> i32 {
        self.origin_key
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn origin_key_ref(&self) -> &i32 {
        &self.origin_key
    }
    pub fn destiny_key_ref(&self) -> &i32 {
        &self.destiny_key
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }

}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.origin_key, self.destiny_key)
    }
}
