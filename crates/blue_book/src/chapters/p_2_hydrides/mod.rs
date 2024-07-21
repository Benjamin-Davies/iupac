//! # P-2 Parent Hydrides

use crate::{graph::Graph, parser};

pub mod p_21_simple_hydrides;
pub mod p_22_monocyclic_hydrides;
pub mod p_25_fused_ring_systems;
pub mod p_29_hydride_prefixes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hydride {
    Simple(p_21_simple_hydrides::SimpleHydride),
    Monocyclic(p_22_monocyclic_hydrides::MonocyclicHydride),
    FusedRing(p_25_fused_ring_systems::FusedRingSystem),
    Isobutane,
}

impl Hydride {
    pub fn to_graph(&self) -> Graph {
        match self {
            Hydride::Simple(ast) => ast.to_graph(),
            Hydride::Monocyclic(ast) => ast.to_graph(),
            Hydride::FusedRing(ast) => ast.to_graph(),
            Hydride::Isobutane => isobutane_graph(),
        }
    }
}

fn isobutane_graph() -> Graph {
    let ast = parser::parse("1,1-Dimethylethane");
    Graph::from(&*ast)
}
