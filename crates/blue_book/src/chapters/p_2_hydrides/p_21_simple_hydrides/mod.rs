//! # P-21 Mononuclear and Acyclic Polynuclear Parent Hydrides

use crate::{graph::Graph, Element, Locant};

use super::Hydride;

pub mod p_21_1_mononuclear_hydrides;
pub mod p_21_2_acyclic_hydrides;

/// A homogenous acyclic (mono- or poly-nuclear) parent hydride.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleHydride {
    pub length: u16,
    pub element: Element,
}

impl From<SimpleHydride> for Hydride {
    fn from(ast: SimpleHydride) -> Self {
        Hydride::Simple(ast)
    }
}

impl SimpleHydride {
    pub fn to_graph(&self) -> Graph {
        let element = self.element;

        let length = self.length as usize;
        let bonding_number = element.standard_bonding_number() as usize;
        let hydrogens = length * (bonding_number - 2) + 2;

        Graph {
            atoms: []
                .into_iter()
                .chain((0..length).map(|_| element))
                .chain((0..hydrogens).map(|_| Element::Hydrogen))
                .collect(),
            bonds: []
                .into_iter()
                .chain((0..length - 1).map(|i| (i, i + 1)))
                .chain((0..length).flat_map(|i| {
                    (0..bonding_number - 2).map(move |j| (i, length + j * length + i))
                }))
                .chain([
                    (0, length + hydrogens - 2),
                    (length - 1, length + hydrogens - 1),
                ])
                .collect(),
            positions: (0..length)
                .map(|i| (Locant::Number(i as u16 + 1), i))
                .collect(),
            free_valences: Vec::new(),
        }
    }
}
