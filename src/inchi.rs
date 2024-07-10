use std::{collections::BTreeMap, ops::RangeInclusive};

use petgraph::graph::UnGraph;

use crate::Element;

mod parser;
mod scanner;

#[derive(Debug, Default)]
pub struct InChI {
    formula: Formula,
    connections: Connections,
    hydrogens: Hydrogens,
}

#[derive(Debug, Default)]
struct Formula {
    atom_counts: BTreeMap<Element, usize>,
}

#[derive(Debug, Default)]
struct Connections {
    connections: Vec<(usize, usize)>,
}

#[derive(Debug, Default)]
struct Hydrogens {
    immobile_hydrogens: Vec<(Vec<RangeInclusive<usize>>, usize)>,
    mobile_hydrogens: Vec<(usize, Vec<usize>)>,
}

impl From<&InChI> for UnGraph<Element, ()> {
    fn from(value: &InChI) -> Self {
        let mut graph = UnGraph::new_undirected();

        let mut terminal_indices = Vec::new();
        let mut skeletal_indices = Vec::new();
        for (&element, &count) in &value.formula.atom_counts {
            for _ in 0..count {
                let node = graph.add_node(element);

                if element == Element::Hydrogen {
                    terminal_indices.push(node);
                } else {
                    skeletal_indices.push(node);
                }
            }
        }

        for &(i, j) in &value.connections.connections {
            let a = skeletal_indices[i - 1];
            let b = skeletal_indices[j - 1];
            graph.add_edge(a, b, ());
        }

        let mut terminal_indices = terminal_indices.into_iter();
        for (ranges, count) in &value.hydrogens.immobile_hydrogens {
            for i in ranges.iter().cloned().flatten() {
                dbg!(i, count);
                let c = skeletal_indices[i - 1];
                for _ in 0..*count {
                    let h = terminal_indices.next().unwrap();
                    graph.add_edge(c, h, ());
                }
            }
        }

        for _ in &value.hydrogens.mobile_hydrogens {
            unimplemented!();
        }

        graph
    }
}
