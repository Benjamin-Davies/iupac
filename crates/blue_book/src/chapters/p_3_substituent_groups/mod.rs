//! # P-3 Characteristic (Functional) and Substituent Groups

use crate::{graph::Graph, Element, Locant};

pub mod p_33_suffixes;
pub mod p_35_characteristic_group_prefixes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacteristicGroup {
    Hydro,
    Hydroxy,
    Oxo,
    Amino,
}

impl CharacteristicGroup {
    pub fn to_graph(self) -> Graph {
        match self {
            CharacteristicGroup::Hydro => hydro_graph(),
            CharacteristicGroup::Hydroxy => hydroxy_graph(),
            CharacteristicGroup::Oxo => oxo_graph(),
            CharacteristicGroup::Amino => amino_graph(),
        }
    }
}

fn hydro_graph() -> Graph {
    Graph {
        atoms: vec![Element::Hydrogen],
        bonds: vec![],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![0],
    }
}

fn hydroxy_graph() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen, Element::Hydrogen],
        bonds: vec![(0, 1)],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![0],
    }
}

fn oxo_graph() -> Graph {
    Graph {
        atoms: vec![Element::Oxygen],
        bonds: vec![],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![0],
    }
}

fn amino_graph() -> Graph {
    Graph {
        atoms: vec![Element::Nitrogen, Element::Hydrogen, Element::Hydrogen],
        bonds: vec![(0, 1), (0, 2)],
        positions: vec![(Locant::Number(1), 0)],
        free_valences: vec![0],
    }
}
