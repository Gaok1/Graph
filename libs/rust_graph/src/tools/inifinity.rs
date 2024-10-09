use std::ops::Add;


#[derive(Debug, Clone, Copy)]
pub enum Infinity {
    Infinite,
    Number (i32),
}

#[allow(unused)]
impl Infinity {
    pub fn new(val: i32) -> Self {
        Infinity::Number(val)
    }

    fn infinite() -> Self {
        Infinity::Infinite
    }

    pub fn is_infinite(&self) -> bool {
        match self {
            Infinity::Infinite => true,
            _ => false,
        }
    }

    pub fn unwrap(&self) -> i32 {
        match self {
            Infinity::Number(val) => *val,
            Infinity::Infinite => panic!("Cannot unwrap infinite value"),
        }
    }
}

impl PartialEq for Infinity{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Infinity::Infinite, Infinity::Infinite) => true,
            (Infinity::Number(a), Infinity::Number(b)) => a == b,
            _ => false,
        }
    }
}


impl Eq for Infinity {}

impl Ord for Infinity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Infinity::Infinite, Infinity::Infinite) => std::cmp::Ordering::Equal,
            (Infinity::Infinite, Infinity::Number(_)) => std::cmp::Ordering::Greater,
            (Infinity::Number(_), Infinity::Infinite) => std::cmp::Ordering::Less,
            (Infinity::Number(a), Infinity::Number(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Infinity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Add for Infinity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, _) => Infinity::Infinite,
            (_, Infinity::Infinite) => Infinity::Infinite,
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a + b),
        }
    }
}

impl std::fmt::Display for Infinity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Infinity::Infinite => write!(f, "∞"),
            Infinity::Number(val) => write!(f, "{}", val),
        }
    }
}