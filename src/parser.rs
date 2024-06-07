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

#[derive(Debug, PartialEq, Eq)]
enum StackItem {
    Molecule(Rc<Molecule>),

    OpenBracket,
    Position(u8, Option<Element>),
    Multiple(u8),
}

pub fn parse(name: &str) -> Rc<Molecule> {
    let mut state = State::default();
    for token in scan(name) {
        match token {
            Token::OpenBracket => {
                state.stack.push(StackItem::OpenBracket);
            }
            Token::CloseBracket => {
                let molecule = state.pop_molecule();
                assert_eq!(
                    state.stack.pop(),
                    Some(StackItem::OpenBracket),
                    "unbalanced brackets"
                );
                state.stack.push(StackItem::Molecule(molecule));
            }

            Token::Position(num, element) => {
                state.stack.push(StackItem::Position(num, element));
            }
            Token::Multiple(num) => {
                state.stack.push(StackItem::Multiple(num));
            }

            Token::Unsaturated(unsaturated) => {
                let mut molecule = state.pop_molecule();
                if unsaturated != 0 {
                    molecule = Molecule::Unsaturated(unsaturated, molecule).into();
                }
                state.stack.push(StackItem::Molecule(molecule));
            }
            Token::FreeValence => {
                let base = state.pop_molecule();
                let molecule = Molecule::FreeValence(base).into();
                state.stack.push(StackItem::Molecule(molecule));
            }

            Token::Base(base) => {
                let molecule = Molecule::Base(base).into();
                state.stack.push(StackItem::Molecule(molecule));
            }
            Token::Prefix(base) => {
                let base = Molecule::Base(base).into();
                let group = Molecule::FreeValence(base).into();
                state.stack.push(StackItem::Molecule(group));
            }
            Token::Suffix(base) => {
                let base = Molecule::Base(base).into();
                let group: Rc<_> = Molecule::FreeValence(base).into();

                let positions = state.pop_multiplicity_and_positions().collect::<Vec<_>>();
                let mut molecule = state.pop_molecule();
                for (num, element) in positions {
                    molecule = Molecule::Substitution(num, element, group.clone(), molecule).into();
                }

                state.stack.push(StackItem::Molecule(molecule));
            }
        }
    }

    let molecule = state.pop_molecule();
    assert!(state.stack.is_empty(), "unbalanced stack: {:?}", state);
    molecule
}

impl State {
    fn pop_molecule(&mut self) -> Rc<Molecule> {
        let mut molecule = match self.stack.pop().unwrap() {
            StackItem::Molecule(molecule) => molecule,
            StackItem::Multiple(num) => Molecule::Alkane(num).into(),
            _ => todo!(),
        };

        while let Some(StackItem::Molecule(group)) = self.stack.last() {
            let group = group.clone();
            self.stack.pop();

            for (num, element) in self.pop_multiplicity_and_positions() {
                molecule = Molecule::Substitution(num, element, group.clone(), molecule).into();
            }
        }

        molecule
    }

    fn pop_multiplicity_and_positions(
        &mut self,
    ) -> impl Iterator<Item = (Option<u8>, Option<Element>)> + '_ {
        let multiplicity = if let Some(&StackItem::Multiple(num)) = self.stack.last() {
            self.stack.pop();
            num
        } else {
            1
        };

        (0..multiplicity).map(|_| {
            let mut num = None;
            let mut element = None;
            if let Some(StackItem::Position(n, e)) = self.stack.pop() {
                num = Some(n);
                element = e;
            };

            (num, element)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner::Base, test::DOPAMINE};

    use super::{parse, Molecule};

    #[test]
    fn test_parse_simple() {
        assert_eq!(parse("Butane"), Molecule::Alkane(4).into());

        assert_eq!(
            parse("Ethene"),
            Molecule::Unsaturated(1, Molecule::Alkane(2).into()).into(),
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
            )
            .into(),
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
            )
            .into(),
        );

        assert_eq!(
            parse("Pentyne"),
            Molecule::Unsaturated(2, Molecule::Alkane(5).into()).into(),
        );
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse(DOPAMINE),
            Molecule::Substitution(
                Some(1),
                None,
                Molecule::FreeValence(Molecule::Base(Base::Water).into()).into(),
                Molecule::Substitution(
                    Some(2),
                    None,
                    Molecule::FreeValence(Molecule::Base(Base::Water).into()).into(),
                    Molecule::Substitution(
                        Some(4),
                        None,
                        Molecule::FreeValence(
                            Molecule::Substitution(
                                Some(2),
                                None,
                                Molecule::FreeValence(Molecule::Base(Base::Ammonia).into()).into(),
                                Molecule::Alkane(2).into(),
                            )
                            .into(),
                        )
                        .into(),
                        Molecule::Base(Base::Benzene).into(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        );
    }
}
