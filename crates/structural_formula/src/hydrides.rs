use core::f32;

use blue_book::chapters::p_2_hydrides::{p_21_simple_hydrides::SimpleHydride, Hydride};
use glam::Vec2;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::structure::{Atom, Bond, Structure, ToStructure};

impl ToStructure for Hydride {
    fn to_structure(&self) -> Structure {
        match self {
            Hydride::Simple(hydride) => hydride.to_structure(),
            Hydride::Monocyclic(_) => todo!(),
            Hydride::FusedRing(_) => todo!(),
            Hydride::Isobutane => todo!(),
        }
    }
}

impl ToStructure for SimpleHydride {
    fn to_structure(&self) -> Structure {
        let element = self.element;
        let length = self.length as usize;

        let mut graph = UnGraph::new_undirected();
        for i in 0..length {
            let position = Vec2 {
                x: 0.5 * f32::sqrt(3.0) * i as f32,
                y: if i % 2 == 1 { -0.5 } else { 0.0 },
            };

            let hydrogen_count = if i == 0 || i == length - 1 {
                element.standard_bonding_number() - 1
            } else {
                element.standard_bonding_number() - 2
            };
            let atom = Atom {
                element,
                hydrogen_count,
                position,
            };
            graph.add_node(atom);
        }
        for i in 0..length - 1 {
            let a = NodeIndex::new(i);
            let b = NodeIndex::new(i + 1);
            let bond = Bond { bond_order: 1 };
            graph.add_edge(a, b, bond);
        }

        Structure {
            graph,
            ..Default::default()
        }
    }
}
