//! # P-25 Fused and Bridged Fused Ring Systems

use crate::graph::Graph;

pub mod p_25_2_heterocyclic_ring_components;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FusedRingSystem {
    Heterogeneous(p_25_2_heterocyclic_ring_components::HeterocyclicRing),
}

impl FusedRingSystem {
    pub fn to_graph(&self) -> Graph {
        match self {
            FusedRingSystem::Heterogeneous(ast) => ast.to_graph(),
        }
    }
}
