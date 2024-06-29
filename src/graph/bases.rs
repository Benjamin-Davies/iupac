use crate::{parser, Base, Element, Position};

use super::Graph;

pub fn base(base: &Base, isomer: Position) -> Graph {
    match base {
        Base::Hydrogen => hydrogen(),
        Base::Oxygen => oxygen(),
        Base::Water => water(),
        Base::Ammonia => ammonia(),
        Base::Isobutane => isobutane(),
        Base::Benzene => benzene(),
        Base::Pyrimidine => todo!(),
        Base::Purine => purine(isomer),
    }
}

pub fn hydrogen() -> Graph {
    Graph {
        atoms: vec![Element::Hydrogen, Element::Hydrogen],
        bonds: vec![(0, 1)],
        positions: vec![(Position::Number(1), 0), (Position::Number(2), 1)],
        free_valences: vec![],
    }
}

pub fn oxygen() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen, Element::Oxygen],
        bonds: vec![(0, 1)],
        positions: vec![(Position::Number(1), 0), (Position::Number(2), 1)],
        free_valences: vec![],
    }
}

pub fn water() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen, Element::Hydrogen, Element::Hydrogen],
        bonds: vec![(0, 1), (0, 2)],
        positions: vec![(Position::Number(1), 0)],
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
        positions: vec![(Position::Number(1), 0)],
        free_valences: vec![],
    }
}

pub fn isobutane() -> Graph {
    let ast = parser::parse("1,1-Dimethylethane");
    Graph::from(&*ast)
}

pub fn benzene() -> Graph {
    Graph {
        atoms: []
            .into_iter()
            .chain((0..6).map(|_| Element::Carbon))
            .chain((0..6).map(|_| Element::Hydrogen))
            .collect(),
        bonds: (0..6)
            .flat_map(|i| [(i, i + 6), (i, (i + 1) % 6)])
            .collect(),
        positions: (0..6).map(|i| (Position::Number(i as u8 + 1), i)).collect(),
        free_valences: vec![],
    }
}

pub fn purine(isomer: Position) -> Graph {
    let mut graph = Graph {
        atoms: vec![
            Element::Nitrogen,
            Element::Carbon,
            Element::Nitrogen,
            Element::Carbon,
            Element::Carbon,
            Element::Carbon,
            Element::Nitrogen,
            Element::Carbon,
            Element::Nitrogen,
            Element::Hydrogen,
            Element::Hydrogen,
            Element::Hydrogen,
            Element::Hydrogen,
        ],
        bonds: vec![
            // C-C & C-N
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 0),
            (4, 6),
            (6, 7),
            (7, 8),
            (8, 3),
            // C-H
            (1, 9),
            (5, 10),
            (7, 11),
        ],
        positions: (0..9).map(|i| (Position::Number(i as u8 + 1), i)).collect(),
        free_valences: vec![],
    };

    // N-H bond
    let Position::Element(i, Element::Hydrogen) = isomer else {
        panic!("Invalid purine isomer {isomer:?}");
    };
    graph.bonds.push((i as usize - 1, 12));

    graph
}
