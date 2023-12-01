#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
    // This is going to have to be a web app at some point isn't it?
    // yes, but could we maybe do Erlang/Elixir + Rust instead of pure Rust?
    println!("{:?}", invert(vec![vec![1.0, 2.0], vec![1.0, 1.0]]));
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
type Matrix = Vec<Vec<f32>>;

#[derive(Debug, PartialEq, Clone)]
struct Key {
    matrix: Vec<Vec<f32>>,
}

impl Eq for Key {
    // TODO
}

impl Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO
    }
}
// TODO
// impl Debug for Matrix {}

#[derive(Debug, PartialEq, Clone)]
struct RowKey {
    row: Vec<f32>,
}

impl Eq for RowKey {}

impl Hash for RowKey {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

trait MatrixTrait {
    fn valid(&self) -> bool;
    fn is_identity_matrix(&self) -> bool;
    fn is_square(&self) -> bool;
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

    fn is_identity_matrix(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        if !self.is_square() {
            return false;
        }
        for i in 0..self.len() {
            for j in 0..self.len() {
                if self[i][j] != 0.0 && !(i == j && self[i][j] == 1.0) {
                    return false;
                }
            }
        }
        true
    }

    fn is_square(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        for row in self {
            if row.len() != self.len() {
                return false;
            }
        }
        true
    }
}

fn find_pivot_row(m: &Matrix) -> usize {
    let mut out = 0;
    let mut index = m[0].lowest_non_zero_index();
    for i in 0..m.len() {
        let row = m[i].clone();
        if row.lowest_non_zero_index() < index {
            index = row.lowest_non_zero_index();
            out = i;
        }
    }
    out
}

trait Row {
    fn lowest_non_zero_index(&self) -> usize;
    fn normalize(&mut self);
}

impl Row for Vec<f32> {
    fn lowest_non_zero_index(&self) -> usize {
        for i in 0..self.len() {
            if self[i] != 0.0 {
                return i;
            }
        }
        self.len()
    }

    fn normalize(&mut self) {
        let factor = self[self.lowest_non_zero_index()];
        for i in 0..self.len() {
            self[i] /= factor;
        }
    }
}
fn invert(mut m: Matrix) -> Matrix {
    if m.len() == 0 {
        return m;
    }
    m = create_rectangle_matrix_for_inversion(m);
    // FIXME
    // this can probably be a finite for loop,
    // that does 2 * m.len() iterations
    // no reason to have an infinte loop
    // or perhaps to loops of m.len()

    for i in 0..m.len() {
        println!("m: {:?}", m);
        println!("row: {}", i);
        m[i].normalize();
        for j in i + 1..m.len() {
            for k in 0..m[j].len() {
                m[j][k] -= m[i][k];
            }
            println!("row {} after, substraction: {:?}", j, m[j]);
        }
    } // this algo will *FAIL* on certain cases, but still beats previous one.

    extract_right_hand_side(&m)
}

fn extract_right_hand_side(m: &Matrix) -> Matrix {
    let mut out = vec![];
    for i in 0..m.len() {
        // truncate row
        // FIXME this can  probably be done with list comphrensions/filters or something more elegant
        let mut new_row = vec![];
        for j in (m[i].len() / 2)..m[i].len() {
            new_row.push(m[i][j]);
        }
        out.push(new_row);
    }
    out
}

fn extract_left_hand_side(m: &Matrix) -> Matrix {
    let mut out = vec![];
    for i in 0..m.len() {
        let mut new_row = vec![];
        for j in 0..m[i].len() / 2 {
            new_row.push(m[i][j]);
        }
        out.push(new_row);
    }
    out
}

fn create_rectangle_matrix_for_inversion(mut m: Matrix) -> Matrix {
    let columns = m[0].len();
    // double the number of columns
    for i in 0..m.len() {
        for j in 0..columns {
            let value = if i == j { 1.0 } else { 0.0 };
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
        let mut test_cases: HashMap<Key, Vec<Vec<f32>>> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            },
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        );
        test_cases.insert(Key { matrix: vec![] }, vec![]);
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 2.0], vec![1.0, 1.0]],
            },
            vec![vec![-1.0, 2.0], vec![1.0, -1.0]],
        );
        //        test_cases.insert(vec![vec![1, 2], vec![3, 4]], vec![vec![], vec![]]); TODO refactor to use floats
        for (input, expect) in test_cases {
            let original = input.clone();
            let got = invert(input.matrix);
            println!(
                "input: {:?}\ngot: {:?}\nexpect: {:?}",
                original, got, expect
            );
            assert_eq!(got, expect);
        }
    }

    #[test]
    fn test_is_identity_matrix() {
        let mut test_cases: HashMap<Key, bool> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            },
            true,
        );
        test_cases.insert(Key { matrix: vec![] }, true);
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0]],
            },
            true,
        );
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 1.0]],
            },
            false,
        );
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 1.0], vec![1.0, 2.0]],
            },
            false,
        );
        for (input, expect) in test_cases {
            println!(
                "input: {:?}, input.is_identity_matrix: {}, expect: {}",
                input,
                input.matrix.is_identity_matrix(),
                expect
            );
            assert_eq!(input.matrix.is_identity_matrix(), expect);
        }
    }

    #[test]
    fn test_is_square() {
        let mut test_cases: HashMap<Key, bool> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            },
            true,
        );
        test_cases.insert(Key { matrix: vec![] }, true);
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0]],
            },
            true,
        );
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 1.0]],
            },
            false,
        );
        for (input, expect) in test_cases {
            println!(
                "input: {:?}, input.is_square: {}, expect: {}",
                input,
                input.matrix.is_square(),
                expect
            );
            assert_eq!(input.matrix.is_square(), expect);
        }
    }

    #[test]
    fn test_extract_right_hand_side() {
        let mut test_cases: HashMap<Key, Matrix> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 2.0, 3.0, 4.0], vec![0.0, 1.0, 2.0, 3.0]],
            },
            vec![vec![3.0, 4.0], vec![2.0, 3.0]],
        );
        for (input, expect) in test_cases {
            let got = extract_right_hand_side(&input.matrix);
            println!("input: {:?}\ngot: {:?}\nexpect: {:?}", input, got, expect);
            assert_eq!(got, expect);
        }
    }

    #[test]
    fn test_extract_left_hand_side() {
        let mut test_cases: HashMap<Key, Matrix> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![1.0, 2.0, 3.0, 4.0], vec![0.0, 1.0, 2.0, 3.0]],
            },
            vec![vec![1.0, 2.0], vec![0.0, 1.0]],
        );
        for (input, expect) in test_cases {
            let got = extract_left_hand_side(&input.matrix);
            println!(
                "input: {:?}\ngot: {:?}\nexpect: {:?}",
                input.matrix, got, expect
            );
            assert_eq!(got, expect);
        }
    }

    #[test]
    fn test_lowest_non_zero_index() {
        let mut test_cases: HashMap<RowKey, usize> = HashMap::new();
        test_cases.insert(
            RowKey {
                row: vec![1.0, 2.0],
            },
            0,
        );
        test_cases.insert(
            RowKey {
                row: vec![0.0, 2.0],
            },
            1,
        );
        test_cases.insert(
            RowKey {
                row: vec![0.0, 0.0],
            },
            2,
        );
        for (input, expect) in test_cases {
            let got = input.row.lowest_non_zero_index();
            println!("input: {:?}\ngot: {:?}\nexpect: {:?}", input, got, expect);
            assert_eq!(got, expect);
        }
    }

    #[test]
    fn test_find_pivot_row() {
        let mut test_cases: HashMap<Key, usize> = HashMap::new();
        test_cases.insert(
            Key {
                matrix: vec![vec![0.0, 1.0], vec![1.0, 0.0]],
            },
            1,
        );
        for (input, expect) in test_cases {
            let got = find_pivot_row(&input.matrix);
            println!("input: {:?}\ngot: {:?}\nexpect: {:?}", input, got, expect);
            assert_eq!(got, expect);
        }
    }
}
