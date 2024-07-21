//! # P-25.2 Heterocyclic Ring Components

use parsing::dfa;

use self::HeterocyclicRing::Purine;
use crate::{
    chapters::p_2_hydrides::Hydride, graph::Graph, plugin::Plugin, scanner::Token, Element, Locant,
};

use super::FusedRingSystem;

pub struct HeterocyclicRingPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeterocyclicRing {
    Purine(u8),
}

impl Plugin for HeterocyclicRingPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        for i in 1..=9 {
            let key = format!("{}H-purin", i).leak();
            dfa.insert(key, Token::Hydride(Purine(i).into()));
        }
    }
}

impl From<HeterocyclicRing> for Hydride {
    fn from(ast: HeterocyclicRing) -> Self {
        Hydride::FusedRing(FusedRingSystem::Heterogeneous(ast))
    }
}

impl HeterocyclicRing {
    pub fn to_graph(&self) -> Graph {
        match self {
            &Purine(isomer) => purine(isomer),
        }
    }
}

pub fn purine(isomer: u8) -> Graph {
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
        positions: (0..9).map(|i| (Locant::Number(i as u16 + 1), i)).collect(),
        free_valences: vec![],
    };

    // N-H bond
    graph.bonds.push((isomer as usize - 1, 12));

    graph
}
