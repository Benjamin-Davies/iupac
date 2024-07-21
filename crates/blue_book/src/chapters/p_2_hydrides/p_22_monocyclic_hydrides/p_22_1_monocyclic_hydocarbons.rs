//! # P-22.1 Monocyclic Hydrocarbons

use parsing::dfa;

use crate::{
    chapters::p_2_hydrides::Hydride, graph::Graph, plugin::Plugin, scanner::Token, Element, Locant,
};

use self::MonocyclicHydrocarbon::Benzene;
use super::MonocyclicHydride;

pub struct MonocyclicHydrocarbonsPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonocyclicHydrocarbon {
    Benzene,
}

impl Plugin for MonocyclicHydrocarbonsPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("benzen", Token::Hydride(Benzene.into()));
        dfa.insert("phen", Token::Hydride(Benzene.into()));
    }
}

impl From<MonocyclicHydrocarbon> for Hydride {
    fn from(ast: MonocyclicHydrocarbon) -> Self {
        Hydride::Monocyclic(MonocyclicHydride::Hydrocarbon(ast))
    }
}

impl MonocyclicHydrocarbon {
    pub fn to_graph(&self) -> Graph {
        match self {
            MonocyclicHydrocarbon::Benzene => benzene_graph(),
        }
    }
}

fn benzene_graph() -> Graph {
    Graph {
        atoms: []
            .into_iter()
            .chain((0..6).map(|_| Element::Carbon))
            .chain((0..6).map(|_| Element::Hydrogen))
            .collect(),
        bonds: (0..6)
            .flat_map(|i| [(i, i + 6), (i, (i + 1) % 6)])
            .collect(),
        positions: (0..6).map(|i| (Locant::Number(i as u16 + 1), i)).collect(),
        free_valences: vec![],
    }
}
