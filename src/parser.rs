use std::rc::Rc;

use crate::scanner::{scan, Base, Element, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Molecule {
    Base(Base),
    Alkane(u8),

    FreeValence(Rc<Molecule>),
    Unsaturated(u8, Rc<Molecule>),
    Substitution(Option<u8>, Option<Element>, Rc<Molecule>, Rc<Molecule>),
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

        while let Some(StackItem::Molecule(group)) = self.stack.last() {
            let group = group.clone();
            self.stack.pop();

            let multiplicity = if let Some(&StackItem::Multiple(num)) = self.stack.last() {
                self.stack.pop();
                num
            } else {
                1
            };

            for _ in 0..multiplicity {
                let mut num = None;
                let mut element = None;
                if let Some(StackItem::Position(n, e)) = self.stack.pop() {
                    num = Some(n);
                    element = e;
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
            parse("Hexamethylpentane"),
            Molecule::Substitution(
                None,
                None,
                Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                Molecule::Substitution(
                    None,
                    None,
                    Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                    Molecule::Substitution(
                        None,
                        None,
                        Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                        Molecule::Substitution(
                            None,
                            None,
                            Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                            Molecule::Substitution(
                                None,
                                None,
                                Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                                Molecule::Substitution(
                                    None,
                                    None,
                                    Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                                    Molecule::Alkane(5).into(),
                                )
                                .into(),
                            )
                            .into(),
                        )
                        .into(),
                    )
                    .into(),
                )
                .into(),
            ),
        );

        assert_eq!(
            parse("2,2-Dimethylpropane"),
            Molecule::Substitution(
                Some(2),
                None,
                Molecule::FreeValence(Molecule::Alkane(1).into()).into(),
                Molecule::Substitution(
                    Some(2),
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
