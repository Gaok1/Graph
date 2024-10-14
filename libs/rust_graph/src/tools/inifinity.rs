use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub enum Infinity {
    Infinite, // pode ser infinito
    Number(i32), // ou um número
}

#[allow(unused)]
impl Infinity {
    pub fn new(val: i32) -> Self {
        Infinity::Number(val)
    }

    pub fn infinite() -> Self {
        Infinity::Infinite
    }

    pub fn is_infinite(&self) -> bool {
        matches!(self, Infinity::Infinite)
    }

    pub fn unwrap(&self) -> i32 {
        match self {
            Infinity::Number(val) => *val,
            Infinity::Infinite => panic!("Cannot unwrap infinite value"),
        }
    }

    pub fn saturating_add(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, _) | (_, Infinity::Infinite) => Infinity::Infinite,
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a.saturating_add(b)),
        }
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, Infinity::Infinite) => Infinity::Number(0), // Custom behavior for ∞ - ∞
            (Infinity::Infinite, _) => Infinity::Infinite,
            (_, Infinity::Infinite) => Infinity::Number(0),
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a.saturating_sub(b)),
        }
    }
}

impl PartialEq for Infinity {
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
            (Infinity::Infinite, _) | (_, Infinity::Infinite) => Infinity::Infinite,
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a + b),
        }
    }
}

impl Sub for Infinity {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, Infinity::Infinite) => Infinity::Infinite,
            (Infinity::Infinite, _) => Infinity::Infinite,
            (_, Infinity::Infinite) => Infinity::Number(0),
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a - b),
        }
    }
}

impl Mul for Infinity {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, Infinity::Number(0)) | (Infinity::Number(0), Infinity::Infinite) => {
                Infinity::Number(0)
            }
            (Infinity::Infinite, _) | (_, Infinity::Infinite) => Infinity::Infinite,
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a * b),
        }
    }
}

impl Div for Infinity {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Infinity::Infinite, Infinity::Infinite) => panic!("Indeterminate form: ∞ / ∞"),
            (_, Infinity::Number(0)) => panic!("Division by zero"),
            (Infinity::Infinite, _) => Infinity::Infinite,
            (Infinity::Number(a), Infinity::Number(b)) => Infinity::Number(a / b),
            (Infinity::Number(_), Infinity::Infinite) => Self::Number(0),
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_addition() {
//         assert_eq!(Infinity::Infinite + Infinity::Number(5), Infinity::Infinite);
//         assert_eq!(Infinity::Number(3) + Infinity::Number(7), Infinity::Number(10));
//     }

//     #[test]
//     fn test_subtraction() {
//         assert_eq!(Infinity::Infinite - Infinity::Number(5), Infinity::Infinite);
//         assert_eq!(Infinity::Number(10) - Infinity::Number(7), Infinity::Number(3));
//     }

//     #[test]
//     fn test_multiplication() {
//         assert_eq!(Infinity::Infinite * Infinity::Number(5), Infinity::Infinite);
//         assert_eq!(Infinity::Number(3) * Infinity::Number(7), Infinity::Number(21));
//     }

//     #[test]
//     fn test_division() {
//         assert_eq!(Infinity::Infinite / Infinity::Number(5), Infinity::Infinite);
//         assert_eq!(Infinity::Number(10) / Infinity::Number(2), Infinity::Number(5));
//     }

//     #[test]
//     #[should_panic]
//     fn test_divide_by_zero() {
//         let _ = Infinity::Number(5) / Infinity::Number(0);
//     }

//     #[test]
//     #[should_panic]
//     fn test_div_infi_infi(){
//         let _ = Infinity::Infinite/Infinity::Infinite;
//     }
//     #[test]
//     fn test_zero(){
//         let _ = assert_eq!(Infinity::Number(100)/Infinity::Infinite, Infinity::Number(0));
//     }


// }