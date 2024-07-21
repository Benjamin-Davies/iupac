//! # P-22.1 Heteromonocyclic Parent Hydrides

use parsing::dfa;

use crate::{
    chapters::p_2_hydrides::Hydride, graph::Graph, plugin::Plugin, scanner::Token, Element, Locant,
};

use self::HeteromonocyclicHydride::Pyrimidine;
use super::MonocyclicHydride;

pub struct HeteromonocyclicHydridesPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeteromonocyclicHydride {
    Pyrimidine,
}

impl Plugin for HeteromonocyclicHydridesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("pyrimidin", Token::Hydride(Pyrimidine.into()))
    }
}

impl From<HeteromonocyclicHydride> for Hydride {
    fn from(ast: HeteromonocyclicHydride) -> Self {
        Hydride::Monocyclic(MonocyclicHydride::Heterogeneous(ast))
    }
}

impl HeteromonocyclicHydride {
    pub fn to_graph(&self) -> Graph {
        match self {
            HeteromonocyclicHydride::Pyrimidine => pyrimidine_graph(),
        }
    }
}

fn pyrimidine_graph() -> Graph {
    Graph {
        atoms: [
            Element::Nitrogen,
            Element::Carbon,
            Element::Nitrogen,
            Element::Carbon,
            Element::Carbon,
            Element::Carbon,
        ]
        .into_iter()
        .chain((0..4).map(|_| Element::Hydrogen))
        .collect(),
        bonds: (0..6)
            .map(|i| (i, (i + 1) % 6))
            .chain([(1, 6), (3, 7), (4, 8), (5, 9)])
            .collect(),
        positions: (0..6).map(|i| (Locant::Number(i as u16 + 1), i)).collect(),
        free_valences: vec![],
    }
}
