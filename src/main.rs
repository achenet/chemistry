#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
    // This is going to have to be a web app at some point isn't it?
    // yes, but could we maybe do Erlang/Elixir + Rust instead of pure Rust?
    //    println!("{:?}", invert(vec![vec![1.0, 2.0], vec![1.0, 1.0]]));
    let mut test = vec![vec![1.0, 2.0], vec![1.0, 1.0]];
    // test.triangularize();
    // println!("Triangularize [[1 2], [1 1]]: {:?}", test.clone());
    // test = vec![
    //     vec![1.0, 2.0, 1.0],
    //     vec![1.0, 1.0, 0.0],
    //     vec![0.0, 3.0, 2.0],
    // ];
    // test.triangularize();
    // println!("Triangularize [[1 2 1], [1 1 0], [0 3 2]]: {:?}", test);
    println!("{:?}", invert(test));
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
        for row in &self.matrix {
            for n in row {
                state.write_i32(*n as i32);
            }
            state.finish();
        }
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
        for n in &self.row {
            state.write_i32(*n as i32);
        }
        state.finish();
    }
}

trait MatrixTrait {
    fn valid(&self) -> bool;
    fn is_identity_matrix(&self) -> bool;
    fn is_square(&self) -> bool;
    fn find_pivot_row(&self) -> usize;
    fn extract_right_hand_side(&self) -> Matrix;
    fn extract_left_hand_side(&self) -> Matrix;
    fn triangularize(&mut self);
    fn is_triangular(&self) -> bool;
    fn extract_submatrix(&self) -> Matrix;
    fn replace_submatrix(&mut self, submatrix: Matrix);
    fn swap_rows(&self, i: usize, j: usize) -> Matrix;
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

    fn find_pivot_row(&self) -> usize {
        let mut out = 0;
        let mut index = self[0].lowest_non_zero_index();
        for i in 0..self.len() {
            let row = self[i].clone();
            if row.lowest_non_zero_index() < index {
                index = row.lowest_non_zero_index();
                out = i;
            }
        }
        out
    }
    fn extract_right_hand_side(&self) -> Matrix {
        let mut out = vec![];
        for i in 0..self.len() {
            // truncate row
            // FIXME this can  probably be done with list comphrensions/filters or something more elegant
            let mut new_row = vec![];
            for j in (self[i].len() / 2)..self[i].len() {
                new_row.push(self[i][j]);
            }
            out.push(new_row);
        }
        out
    }

    fn extract_left_hand_side(&self) -> Matrix {
        let mut out = vec![];
        for i in 0..self.len() {
            let mut new_row = vec![];
            for j in 0..self[i].len() / 2 {
                new_row.push(self[i][j]);
            }
            out.push(new_row);
        }
        out
    }

    fn triangularize(&mut self) {
        if self.len() < 2 {
            return;
        }
        while !self.is_triangular() {
            let pivot = self.find_pivot_row();
            let tmp = self[0].clone();
            self[0] = self[pivot].clone();
            self[pivot] = tmp;
            for i in 1..self.len() {
                let factor = self[i][0] / self[0][0];
                for j in 0..self[i].len() {
                    self[i][j] -= self[0][j] * factor;
                }
            }
            // create submatrix
            let mut sub_matrix = self.extract_submatrix();
            // triangularize that
            sub_matrix.triangularize();
            self.replace_submatrix(sub_matrix);
            println!("{:?}", self);
        }
    }

    fn is_triangular(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 1..self.len() {
            for j in 0..i {
                if self[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    fn extract_submatrix(&self) -> Matrix {
        let mut out = vec![];
        for i in 1..self.len() {
            let mut row = vec![];
            for j in 1..self[i].len() {
                row.push(self[i][j]);
            }
            out.push(row);
        }
        out
    }

    fn replace_submatrix(&mut self, submatrix: Matrix) {
        for i in 1..self.len() {
            for j in 1..self.len() {
                self[i][j] = submatrix[i - 1][j - 1];
            }
        }
    }

    fn swap_rows(&self, i: usize, j: usize) -> Matrix {
        let mut new = self.clone();
        let tmp = self[i].clone();
        new[i] = self[j].clone();
        new[j] = tmp;
        new
    }
}

trait Row {
    fn lowest_non_zero_index(&self) -> usize;
    fn normalize(&mut self);
    fn find_factor(&self) -> f32;
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

    fn find_factor(&self) -> f32 {
        let i = self.lowest_non_zero_index();
        self[i]
    }
}

fn invert(mut m: Matrix) -> Matrix {
    if m.len() == 0 {
        return m;
    }
    if m.len() == 1 {
        return vec![vec![1.0 / m[0][0]]];
    }
    m = create_rectangle_matrix_for_inversion(m);

    // FIXME
    while !m.extract_left_hand_side().is_triangular() {
        let pivot = m.find_pivot_row();
        m = m.swap_rows(0, pivot);
        m[0].normalize();
        for i in 1..m.len() {
            let factor = m[i].find_factor();
            for j in 0..m[i].len() {
                m[i][j] -= m[0][j] * factor;
            }
        }
        println!("{:?}", m);
    }
    while !m.extract_left_hand_side().is_identity_matrix() {
        let old = m.clone();
        let  l = 0..m.len() ;
        let b: Vec<usize> = l.map(|x| m.len() -1 -x).collect();
            println!("b:{:?}", b);
        for i in b {
            // take row i
            // subtract it from every row above
            let n = 1..i+1;
            let c: Vec<usize> = n.map(|x| i - x).collect();
            println!("c:{:?}", c);
            for k in c {
                let factor = m[k].find_factor();
                for j in 0..m[i].len() {
                    m[k][j] -= m[i][j] * factor;
                }
            }
        println!("i: {}\nm: {:?}", i, m);
        }
        if old == m {
            break; // avoid infinite loops
        }
    }

    m.extract_right_hand_side()
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
            let got = input.matrix.extract_right_hand_side();
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
            let got = input.matrix.extract_left_hand_side();
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
            let got = input.matrix.find_pivot_row();
            println!("input: {:?}\ngot: {:?}\nexpect: {:?}", input, got, expect);
            assert_eq!(got, expect);
        }
    }

    struct RowTestCase {
        input: Vec<f32>,
        expect: Vec<f32>,
    }

    #[test]
    fn test_normalize() {
        let test_cases: Vec<RowTestCase> = vec![
            RowTestCase {
                input: vec![2.0, 2.0],
                expect: vec![1.0, 1.0],
            },
            RowTestCase {
                input: vec![1.0, 2.0],
                expect: vec![1.0, 2.0],
            },
            RowTestCase {
                input: vec![0.0, 2.0, 1.0],
                expect: vec![0.0, 1.0, 0.5],
            },
        ];
        for tc in test_cases {
            let mut got = tc.input.clone();
            got.normalize();
            println!(
                "input: {:?}\ngot: {:?}\nexpect: {:?}",
                tc.input, got, tc.expect
            );
            assert_eq!(got, tc.expect);
        }
    }

    struct IsTriangularTestCase {
        input: Matrix,
        expect: bool,
    }

    #[test]
    fn test_is_triangular() {
        let test_cases: Vec<IsTriangularTestCase> = vec![
            IsTriangularTestCase {
                input: vec![],
                expect: true,
            },
            IsTriangularTestCase {
                input: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
                expect: true,
            },
            IsTriangularTestCase {
                input: vec![vec![1.0, 0.0], vec![1.0, 1.0]],
                expect: false,
            },
        ];
        for tc in test_cases {
            let got = tc.input.is_triangular();
            println!(
                "input: {:?}\ngot: {:?}\nexpect: {:?}",
                tc.input, got, tc.expect
            );
            assert_eq!(got, tc.expect);
        }
    }

    struct MatrixTestCase {
        input: Matrix,
        expect: Matrix,
    }

    #[test]
    fn test_triangularize() {
        let test_cases: Vec<Matrix> = vec![vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, 2.0],
            vec![0.0, 0.0, 1.0],
        ]];

        for tc in test_cases {
            let mut got = tc.clone();
            got.triangularize();
            println!("input: {:?}\ngot: {:?}\n", tc, got);
            assert_eq!(got.is_triangular(), true);
        }
    }

    struct FindFactorTestCase {
        input: Vec<f32>,
        expect: f32,
    }

    #[test]
    fn test_find_factor() {
        let test_cases: Vec<FindFactorTestCase> = vec![
            FindFactorTestCase {
                input: vec![0.0, 1.0],
                expect: 1.0,
            },
            FindFactorTestCase {
                input: vec![0.0, 0.0, -1.5],
                expect: -1.5,
            },
        ];
        for tc in test_cases {
            let got = tc.input.find_factor();
            println!(
                "input: {:?}\ngot: {}\nexpect: {}\n",
                tc.input, got, tc.expect
            );
            assert_eq!(got, tc.expect);
        }
    }
}
