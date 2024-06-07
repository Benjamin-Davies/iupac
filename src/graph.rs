use std::fmt;

use crate::{parser::AST, Element, Position};

#[derive(Debug, Default, Clone)]
pub struct Graph {
    atoms: Vec<Element>,
    bonds: Vec<(usize, usize)>,
    positions: Vec<(Position, usize)>,
}

impl From<&AST> for Graph {
    fn from(value: &AST) -> Self {
        match value {
            &AST::Alkane(n) => alkane(n as usize),
            AST::Base(_) => todo!(),
            AST::Isomer(_, _) => todo!(),
            AST::FreeValence(_) => todo!(),
            &AST::Unsaturated(n, ref base) => {
                let base = Graph::from(&**base);
                unsaturate(n as usize, base)
            }
            AST::Substitution(_, _, _) => todo!(),
        }
    }
}

pub fn alkane(n: usize) -> Graph {
    Graph {
        atoms: []
            .into_iter()
            .chain((0..n).map(|_| Element::Carbon))
            .chain((0..2 * n + 2).map(|_| Element::Hydrogen))
            .collect(),
        bonds: []
            .into_iter()
            .chain((0..n - 1).map(|i| (i, i + 1))) // C-C bonds
            .chain((0..n).flat_map(|i| [(i, n + 2 * i), (i, n + 2 * i + 1)])) // Regular C-H bonds
            .chain([(0, 3 * n), (n - 1, 3 * n + 1)].iter().copied()) // End C-H bonds
            .collect(),
        positions: (0..n).map(|i| (Position::Number(i as u8 + 1), i)).collect(),
    }
}

pub fn unsaturate(n: usize, base: Graph) -> Graph {
    let mut molecule = base;
    for _ in 0..n {
        let positions: [_; 2] = molecule.positions[..2].try_into().unwrap();
        for (_, i) in positions {
            let neighboring_hydrogen = molecule
                .neighbors(i)
                .filter(|&j| molecule.atoms[j] == Element::Hydrogen)
                .next()
                .unwrap();
            molecule.remove_atom(neighboring_hydrogen);
        }
    }

    molecule
}

impl Graph {
    fn neighbors(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.bonds.iter().filter_map(move |&(a, b)| {
            if a == i {
                Some(b)
            } else if b == i {
                Some(a)
            } else {
                None
            }
        })
    }

    fn remove_atom(&mut self, i: usize) {
        self.atoms.remove(i);

        self.bonds.retain(|&(a, b)| a != i && b != i);
        self.bonds.iter_mut().for_each(|(a, b)| {
            if *a > i {
                *a -= 1;
            }
            if *b > i {
                *b -= 1;
            }
        });

        self.positions.retain(|&(_, j)| j != i);
        self.positions.iter_mut().for_each(|(_, j)| {
            if *j > i {
                *j -= 1;
            }
        });
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "// Compile using `neato`")?;
        writeln!(f, "graph molecule {{")?;

        for (i, atom) in self.atoms.iter().enumerate() {
            let symbol = atom.symbol();
            writeln!(f, "  {i} [label=\"{symbol}\", shape=none];")?;
        }

        for &(a, b) in &self.bonds {
            writeln!(f, "  {a} -- {b};")?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}
