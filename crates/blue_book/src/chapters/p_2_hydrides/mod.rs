//! # P-2 Parent Hydrides

use crate::graph::Graph;

pub mod p_21_simple_hydrides;
pub mod p_22_monocyclic_hydrides;
pub mod p_25_fused_ring_systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hydride {
    Simple(p_21_simple_hydrides::SimpleHydride),
    Monocyclic(p_22_monocyclic_hydrides::MonocyclicHydride),
    FusedRing(p_25_fused_ring_systems::FusedRingSystem),
}

impl Hydride {
    pub fn to_graph(&self) -> Graph {
        match self {
            Hydride::Simple(ast) => ast.to_graph(),
            Hydride::Monocyclic(ast) => ast.to_graph(),
            Hydride::FusedRing(ast) => ast.to_graph(),
        }
    }
}
