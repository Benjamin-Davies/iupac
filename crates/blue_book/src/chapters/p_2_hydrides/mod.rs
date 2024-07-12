//! # P-2 Parent Hydrides

use crate::graph::Graph;

pub mod p_21_simple_hydrides;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hydride {
    Simple(p_21_simple_hydrides::SimpleHydride),
}

impl Hydride {
    pub fn to_graph(&self) -> Graph {
        match self {
            Hydride::Simple(hydride) => hydride.to_graph(),
        }
    }
}
