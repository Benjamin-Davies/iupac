use blue_book::{chapters::p_3_substituent_groups::CharacteristicGroup, Element};
use petgraph::graph::{NodeIndex, UnGraph};

use crate::structure::{Atom, Structure, ToStructure};

impl ToStructure for CharacteristicGroup {
    fn to_structure(&self) -> Structure {
        match self {
            CharacteristicGroup::Hydro => todo!(),
            CharacteristicGroup::Hydroxy => hydroxy(),
            CharacteristicGroup::Oxo => todo!(),
            CharacteristicGroup::Amino => todo!(),
        }
    }
}

fn hydroxy() -> Structure {
    let mut graph = UnGraph::new_undirected();
    graph.add_node(Atom {
        element: Element::Oxygen,
        hydrogen_count: 1,
    });

    Structure {
        graph,
        free_valences: vec![(NodeIndex::new(0), 1)],
    }
}
