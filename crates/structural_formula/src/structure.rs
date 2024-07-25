use blue_book::{parser::AST, Element, Locant};
use glam::Vec2;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::substitution::substitute;

#[derive(Debug, Clone)]
pub struct Atom {
    pub element: Element,
    pub hydrogen_count: u8,
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub struct Bond {
    /// https://en.wikipedia.org/wiki/Bond_order
    pub bond_order: u8,
}

#[derive(Debug, Default, Clone)]
pub struct Structure {
    pub graph: UnGraph<Atom, Bond>,
    pub free_valences: Vec<(NodeIndex, u8)>,
}

pub trait ToStructure {
    fn to_structure(&self) -> Structure;
}

impl ToStructure for AST {
    fn to_structure(&self) -> Structure {
        match self {
            AST::Hydride(hydride) => hydride.to_structure(),
            AST::Group(ast) => into_group(ast.to_structure()),
            AST::CharacteristicGroup(group) => group.to_structure(),
            AST::Unsaturated(_, _) => todo!(),
            AST::Substitution(locant, group, parent) => {
                substitute(*locant, group.to_structure(), parent.to_structure())
            }
        }
    }
}

fn into_group(mut structure: Structure) -> Structure {
    let id = structure.locate(Locant::Number(1)).unwrap();
    structure.graph[id].hydrogen_count -= 1;
    structure.free_valences.push((id, 1));
    structure
}

impl Structure {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn locate(&self, locant: Locant) -> Option<NodeIndex> {
        match locant {
            Locant::Unspecified => todo!(),
            Locant::Number(n) => self.nth_atom_of_element(n, Element::Carbon),
            Locant::Element(n, element) => self.nth_atom_of_element(n, element),
        }
    }

    fn nth_atom_of_element(&self, n: u16, element: Element) -> Option<NodeIndex> {
        let mut current_n = 1;
        for id in self.graph.node_indices() {
            if self.graph.node_weight(id).unwrap().element != element {
                continue;
            }
            if current_n == n {
                return Some(id);
            }
            current_n += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use blue_book::{parser::parse, test::ISOPROPANOL};

    use super::ToStructure;

    #[test]
    fn test_structure_simple() {
        let ast = parse(ISOPROPANOL);
        let structure = ast.to_structure();

        assert_eq!(structure.graph.node_count(), 4);
        assert_eq!(structure.graph.edge_count(), 3);
        assert!(structure.free_valences.is_empty());
    }
}
