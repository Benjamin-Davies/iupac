//! # P-22 Monocyclic Parent Hydrides

use crate::graph::Graph;

pub mod p_22_1_monocyclic_hydocarbons;
pub mod p_22_2_heteromonocyclic_hydrides;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonocyclicHydride {
    Hydrocarbon(p_22_1_monocyclic_hydocarbons::MonocyclicHydrocarbon),
    Heterogeneous(p_22_2_heteromonocyclic_hydrides::HeteromonocyclicHydride),
}

impl MonocyclicHydride {
    pub fn to_graph(&self) -> Graph {
        match self {
            MonocyclicHydride::Hydrocarbon(ast) => ast.to_graph(),
            MonocyclicHydride::Heterogeneous(ast) => ast.to_graph(),
        }
    }
}
