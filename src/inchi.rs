use std::{collections::BTreeMap, iter, ops::RangeInclusive};

use petgraph::graph::UnGraph;

use crate::Element;

mod parser;
mod scanner;

#[derive(Debug, Default, Clone)]
pub struct InChI {
    formula: Formula,
    connections: Connections,
    hydrogens: Hydrogens,
}

#[derive(Debug, Default, Clone)]
struct Formula {
    atom_counts: BTreeMap<Element, usize>,
}

#[derive(Debug, Default, Clone)]
struct Connections {
    connections: Vec<(usize, usize)>,
}

#[derive(Debug, Default, Clone)]
struct Hydrogens {
    immobile_hydrogens: Vec<(Vec<RangeInclusive<usize>>, usize)>,
    mobile_hydrogens: Vec<(usize, Vec<usize>)>,
}

impl InChI {
    pub fn isomers(self) -> Vec<InChI> {
        let mut isomers = Vec::new();
        self.clone().collect_isomers(&mut isomers);
        isomers
    }

    fn collect_isomers(self, isomers: &mut Vec<InChI>) {
        let Some((count, possible_indices)) = self.hydrogens.mobile_hydrogens.first() else {
            isomers.push(self);
            return;
        };

        let mut mobile_hydrogens = self.hydrogens.mobile_hydrogens.clone();
        if *count == 1 {
            mobile_hydrogens.remove(0);
        } else {
            mobile_hydrogens[0].0 -= 1;
        }

        for &i in possible_indices {
            let mut new = self.clone();
            new.hydrogens.mobile_hydrogens = mobile_hydrogens.clone();
            new.hydrogens.immobile_hydrogens.push((vec![i..=i], 1));
            if new.plausible() {
                new.collect_isomers(isomers);
            }
        }
    }

    /// Quickly checks if a compound is plausible by checking if the number of
    /// bonds for each atom is less than its standard valence.
    fn plausible(&self) -> bool {
        let atom_count = self
            .formula
            .atom_counts
            .iter()
            .map(|(_, &count)| count)
            .sum();

        let mut degrees = vec![0; atom_count];
        for &(i, j) in &self.connections.connections {
            degrees[i - 1] += 1;
            degrees[j - 1] += 1;
        }
        for (ranges, count) in &self.hydrogens.immobile_hydrogens {
            for i in ranges.iter().cloned().flatten() {
                degrees[i - 1] += count;
            }
        }

        let elements = self
            .formula
            .atom_counts
            .iter()
            .filter(|(&element, _)| element != Element::Hydrogen)
            .flat_map(|(&element, &count)| iter::repeat(element).take(count));

        elements
            .zip(degrees)
            .all(|(element, degree)| degree <= element.standard_valence() as usize)
    }
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
