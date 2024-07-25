use blue_book::Locant;
use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::structure::{Bond, Structure};

pub fn substitute(locant: Locant, group: &Structure, mut parent: Structure) -> Structure {
    let &[(group_atom, bond_order)] = group.free_valences.as_slice().try_into().unwrap();
    let location = parent.locate(locant).unwrap();

    // Combine the graphs
    let index_offset = parent.graph.node_count();
    let translate_id = |id: NodeIndex| NodeIndex::new(id.index() + index_offset);

    for atom in group.graph.node_weights() {
        parent.graph.add_node(atom.clone());
    }
    for bond in group.graph.edge_references() {
        parent.graph.add_edge(
            translate_id(bond.source()),
            translate_id(bond.target()),
            bond.weight().clone(),
        );
    }

    // Perform the substitution
    parent
        .graph
        .node_weight_mut(location)
        .unwrap()
        .hydrogen_count -= bond_order;
    parent
        .graph
        .add_edge(location, translate_id(group_atom), Bond { bond_order });

    parent
}
