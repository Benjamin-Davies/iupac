use crate::scanner::{scan, Base, Element, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Molecule {
    Base(Base),
    Alkane(u8),

    FreeValence(Box<Molecule>),
    Unsaturated(u8, Box<Molecule>),
    Substitution(u8, Option<Element>, Box<Molecule>, Box<Molecule>),
}

#[derive(Debug, Default)]
struct State {
    stack: Vec<StackItem>,
}

#[derive(Debug)]
enum StackItem {
    Molecule(Molecule),

    Position(u8, Option<Element>),
    Multiple(u8),
}

pub fn parse(name: &str) -> Molecule {
    let mut state = State::default();
    for token in scan(name) {
        match token {
            Token::OpenBracket => todo!(),
            Token::CloseBracket => todo!(),

            Token::Position(num, element) => {
                state.stack.push(StackItem::Position(num, element));
            }
            Token::Multiple(num) => {
                state.stack.push(StackItem::Multiple(num));
            }

            Token::Unsaturated(unsaturated) => {
                let mut molecule = state.pop_molecule();
                if unsaturated != 0 {
                    molecule = Molecule::Unsaturated(unsaturated, molecule.into());
                }
                state.stack.push(StackItem::Molecule(molecule));
            }
            Token::FreeValence => {
                let base = state.pop_molecule();
                let molecule = Molecule::FreeValence(base.into());
                state.stack.push(StackItem::Molecule(molecule));
            }

            Token::Base(_) => todo!(),
            Token::Prefix(_) => todo!(),
            Token::Suffix(_) => todo!(),
        }
    }

    let molecule = state.pop_molecule();
    assert!(state.stack.is_empty(), "unbalanced stack: {:?}", state);
    molecule
}

impl State {
    fn pop_molecule(&mut self) -> Molecule {
        let mut molecule = match self.stack.pop().unwrap() {
            StackItem::Molecule(molecule) => molecule,
            StackItem::Multiple(num) => Molecule::Alkane(num),
            _ => todo!(),
        };

        while let Some(StackItem::Molecule(_)) = self.stack.last() {
            let StackItem::Molecule(group) = self.stack.pop().unwrap() else {
                unreachable!();
            };

            let multiplicity = if let Some(&StackItem::Multiple(num)) = self.stack.last() {
                self.stack.pop();
                num
            } else {
                1
            };

            for _ in 0..multiplicity {
                let StackItem::Position(num, element) = self.stack.pop().unwrap() else {
                    panic!("expected position, found {:?}", self);
                };

                molecule =
                    Molecule::Substitution(num, element, group.clone().into(), molecule.into());
            }
        }

        molecule
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Molecule};

    #[test]
    fn test_parse_simple() {
        assert_eq!(parse("Butane"), Molecule::Alkane(4));

        assert_eq!(
            parse("Ethene"),
            Molecule::Unsaturated(1, Molecule::Alkane(2).into()),
        );

        assert_eq!(
            parse("2,2-Dimethylpropane"),
            Molecule::Substitution(
                2,
                None,
                Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                Molecule::Substitution(
                    2,
                    None,
                    Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                    Molecule::Alkane(3).into(),
                )
                .into(),
            ),
        );

        assert_eq!(
            parse("Pentyne"),
            Molecule::Unsaturated(2, Molecule::Alkane(5).into()),
        );
    }
}
