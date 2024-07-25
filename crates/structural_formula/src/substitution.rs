use blue_book::Locant;
use glam::{Affine2, Vec2};
use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::structure::{Bond, Structure};

pub fn substitute(locant: Locant, mut group: Structure, mut parent: Structure) -> Structure {
    let &[(group_atom, bond_order)] = group.free_valences.as_slice().try_into().unwrap();
    let parent_atom = parent.locate(locant).unwrap();

    let offset = Vec2::NEG_Y;
    let parent_atom_position = parent.graph[parent_atom].position;
    let group_atom_position = group.graph[group_atom].position;
    let translation = parent_atom_position - group_atom_position + offset;
    group.transform(Affine2::from_translation(translation));

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
    parent.graph[parent_atom].hydrogen_count -= bond_order;
    parent
        .graph
        .add_edge(parent_atom, translate_id(group_atom), Bond { bond_order });

    parent
}
