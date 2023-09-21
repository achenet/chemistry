use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
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

fn create_molecule(elements: Vec<(Element, u16)>) -> HashMap<Element, u16> {
    let mut molecule: HashMap<Element, u16> = HashMap::new();
    for (e, sub) in elements {
        molecule.insert(e, sub);
    }
    molecule
}

#[cfg(test)]
mod tests {
    use crate::create_molecule;
    use crate::is_balanced;
    use crate::Element;
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
}
