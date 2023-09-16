use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

// First order of business:
// balancing equations

#[derive(PartialEq, Eq, Hash)]
enum Element {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    // et plus si affinite
}

#[derive(PartialEq, Eq, Hash, Display)]
struct Molecule {
    elements: Vec<Element>,
    subscripts: Vec<u16>,
}

impl Molecule {
    fn is_valid(&self) -> bool {
        self.elements.len() == self.subscripts.len()
    }

    fn print(&self) {
        if !self.is_valid() {
            println!("invalid molecule");
        }
        let mut out = String::new();
        // TODO improve this for loop
        for i in (0..self.elements.len()) {
            out += self.elements[i].to_string();
            out += self.subscripts[i];
        }
        println!("{}", out);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, Molecule};
    #[test]
    fn test_is_valid() {
        use std::collections::HashMap;
        let mut tests = HashMap::new();
        tests.insert(
            Molecule {
                elements: vec![Element::H],
                subscripts: vec![2],
            },
            true,
        );
        tests.insert(
            Molecule {
                elements: vec![Element::H, Element::He],
                subscripts: vec![1],
            },
            false,
        );

        for (k, v) in tests {
            assert_eq!(k.is_valid(), v);
        }
    }
}
