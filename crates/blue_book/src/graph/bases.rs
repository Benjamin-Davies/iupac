use crate::{parser, Base, Element, Locant};

use super::Graph;

pub fn base(base: &Base) -> Graph {
    match base {
        Base::Hydrogen => hydrogen(),
        Base::Oxygen => oxygen(),
        Base::Water => water(),
        Base::Ammonia => ammonia(),
        Base::Isobutane => isobutane(),
    }
}

pub fn hydrogen() -> Graph {
    Graph {
        atoms: vec![Element::Hydrogen, Element::Hydrogen],
        bonds: vec![(0, 1)],
        positions: vec![(Locant::Number(1), 0), (Locant::Number(2), 1)],
        free_valences: vec![],
    }
}

pub fn oxygen() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen, Element::Oxygen],
        bonds: vec![(0, 1)],
        positions: vec![(Locant::Number(1), 0), (Locant::Number(2), 1)],
        free_valences: vec![],
    }
}

pub fn water() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen, Element::Hydrogen, Element::Hydrogen],
        bonds: vec![(0, 1), (0, 2)],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![],
    }
}

pub fn ammonia() -> Graph {
    Graph {
        atoms: vec![
            Element::Nitrogen,
            Element::Hydrogen,
            Element::Hydrogen,
            Element::Hydrogen,
        ],
        bonds: vec![(0, 1), (0, 2), (0, 3)],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![],
    }
}

pub fn isobutane() -> Graph {
    let ast = parser::parse("1,1-Dimethylethane");
    Graph::from(&*ast)
}
