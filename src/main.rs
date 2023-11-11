#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // This is going to have to be a web app at some point isn't it?
    // yes, but could we maybe do Erlang/Elixir + Rust instead of pure Rust?
}

// First order of business:
// balancing equations

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

fn is_balanced(
    reagents: Vec<(u16, HashMap<Element, u16>)>,
    products: Vec<(u16, HashMap<Element, u16>)>,
) -> bool {
    // Initialize
    let mut reagent_map: HashMap<Element, u16> = HashMap::new();
    let mut product_map: HashMap<Element, u16> = HashMap::new();
    for (coef, molecule) in reagents {
        for (element, subscript) in molecule {
            let count = reagent_map.entry(element).or_insert(0);
            *count += coef * subscript
        }
    }
    for (coef, molecule) in products {
        for (element, subscript) in molecule {
            let count = product_map.entry(element).or_insert(0);
            *count += coef * subscript
        }
    }
    println!("reagent map: {:?}", reagent_map);
    println!("product map: {:?}", product_map);
    for element in reagent_map.keys() {
        if reagent_map.get(element) != product_map.get(element) {
            return false;
        }
    }
    true
}

// Matrix represents a matrix.
// each sub vector is a row
// The element of index i,j is at row i, column j
type Matrix = Vec<Vec<i32>>;

trait MatrixTrait {
    fn valid(&self) -> bool;
}

impl MatrixTrait for Matrix {
    fn valid(&self) -> bool {
        let l = self.len();
        if l == 0 {
            return true;
        }
        let k = self[0].len();
        for i in 1..l {
            if self[i].len() != k {
                return false;
            }
        }
        true
    }
}

fn invert(mut m: Matrix) -> Matrix {
    m = create_rectangle_matrix_for_inversion(m);
    // Gauss-Jordan elimination
    // find pivot row (if in doubt pick first row)
    // use it to get rid of all elements below
    // rinse and repeat

    // find pivot row
    let mut pivot = 0;
    for row in 0..m.len() {
        if m[row][0] == 1 {
            pivot = row
        }
    }
    // normalize if need be
    let z = m[pivot][0];
    for i in 0..m[pivot].len() {
        m[pivot][i] = m[pivot][i] / z;
    }

    // swap pivot row with first row
    (m[0], m[pivot]) = (m[pivot].clone(), m[0].clone());

    for i in 1..m.len() {
        let factor = m[i][0];
        for j in 0..m[0].len() {
            m[i][j] = m[i][j] - (m[0][j] * factor);
        }
    }
    m
}

fn create_rectangle_matrix_for_inversion(mut m: Matrix) -> Matrix {
    let columns = m[0].len();
    // double the number of columns
    for i in 0..m.len() {
        for j in 0..columns {
            let value = if i == j { 1 } else { 0 };
            m[i].push(value);
        }
    }
    m
}

fn create_molecule(elements: Vec<(Element, u16)>) -> HashMap<Element, u16> {
    let mut molecule: HashMap<Element, u16> = HashMap::new();
    for (e, sub) in elements {
        molecule.insert(e, sub);
    }
    molecule
}

#[cfg(test)]
mod tests {
    use crate::Element;
    use crate::*;
    use std::collections::HashMap;

    #[test]
    fn test_is_balanced() {
        let mut methane = HashMap::new();
        methane.insert(Element::C, 1);
        methane.insert(Element::H, 4);
        let mut oxygen = HashMap::new();
        oxygen.insert(Element::O, 2);
        let reagents = vec![(1, methane), (2, oxygen)];
        let products = vec![
            (1, create_molecule(vec![(Element::C, 1), (Element::O, 2)])),
            (2, create_molecule(vec![(Element::H, 2), (Element::O, 1)])),
        ];
        assert_eq!(is_balanced(reagents, products), true);
    }

    #[test]
    fn test_invert() {
        let mut test_cases: HashMap<Vec<Vec<i32>>, Vec<Vec<i32>>> = HashMap::new();
        test_cases.insert(vec![vec![1, 0], vec![0, 1]], vec![vec![1, 0], vec![0, 1]]);
        for (key, value) in test_cases {
            assert_eq!(invert(key), value);
        }
    }
}
