use std::rc::Rc;

use crate::{
    scanner::{scan, Token},
    Base, Element, Locant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Alkane(u16),
    Base(Base),
    Isomer(Locant, Base),

    Group(Rc<AST>),
    Unsaturated(u8, Rc<AST>),
    Substitution(Locant, Rc<AST>, Rc<AST>),
}

#[derive(Debug, Default)]
pub(crate) struct State {
    pub stack: Vec<StackItem>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StackItem {
    Molecule(Rc<AST>),

    OpenBracket,
    Locant(Locant),
    Multiplicity(u16),
}

pub fn parse(name: &str) -> Rc<AST> {
    let mut state = State::default();
    for token in scan(name) {
        match token {
            Token::OpenBracket => {
                state.stack.push(StackItem::OpenBracket);
            }
            Token::CloseBracket => {
                let mut hydride_positions = Vec::new();
                while let Some(&StackItem::Locant(position)) = state.stack.last() {
                    state.stack.pop();
                    // In molecules such as Thymine, this token is used to
                    // indicate which atoms receive hydrogen atoms.
                    let Locant::Element(n, Element::Hydrogen) = position else {
                        panic!("Unexpected position in brackets (expected a hydride marker): {position:?}")
                    };
                    hydride_positions.push(Locant::Number(n));
                }

                if matches!(state.stack.last(), Some(StackItem::OpenBracket)) {
                    state.stack.pop();
                } else {
                    let molecule = state.pop_molecule();
                    assert_eq!(
                        state.stack.pop(),
                        Some(StackItem::OpenBracket),
                        "unbalanced brackets: {state:?}\n{molecule:?}",
                    );
                    state.stack.push(StackItem::Molecule(molecule));
                }

                if !hydride_positions.is_empty() {
                    let StackItem::Molecule(molecule) = state
                        .stack
                        .iter_mut()
                        .rfind(|i| matches!(i, StackItem::Molecule(_)))
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    for position in hydride_positions {
                        *molecule = AST::Substitution(
                            position,
                            AST::Group(AST::Base(Base::Hydrogen).into()).into(),
                            molecule.clone(),
                        )
                        .into();
                    }
                }
            }

            Token::Locant(pos) => {
                state.stack.push(StackItem::Locant(pos));
            }
            Token::Multiplicity(num) => {
                state.stack.push(StackItem::Multiplicity(num));
            }

            Token::Alkane(n) => {
                state.stack.push(StackItem::Molecule(AST::Alkane(n).into()));
            }
            Token::Unsaturated(unsaturated) => {
                let mut molecule = state.pop_molecule();
                if unsaturated != 0 {
                    molecule = AST::Unsaturated(unsaturated, molecule).into();
                }
                state.stack.push(StackItem::Molecule(molecule));
            }
            Token::FreeValence => {
                let base = state.pop_molecule();
                let molecule = AST::Group(base).into();
                state.stack.push(StackItem::Molecule(molecule));
            }

            Token::Base(base) => {
                let molecule;
                if base.has_isomers() {
                    let &StackItem::Locant(pos) = state.stack.last().unwrap() else {
                        panic!("missing position for isomer: {base:?}, {state:?}");
                    };
                    state.stack.pop();

                    molecule = AST::Isomer(pos, base).into();
                } else {
                    molecule = AST::Base(base).into();
                }

                state.stack.push(StackItem::Molecule(molecule));
            }
            Token::Prefix(base) => {
                let base = AST::Base(base).into();
                let group = AST::Group(base).into();
                state.stack.push(StackItem::Molecule(group));
            }
            Token::Suffix(base) => {
                let base = AST::Base(base).into();
                let group: Rc<_> = AST::Group(base).into();

                let positions = state.pop_multiplicity_and_positions().collect::<Vec<_>>();
                let mut molecule = state.pop_molecule();
                for pos in positions {
                    molecule = AST::Substitution(pos, group.clone(), molecule).into();
                }

                state.stack.push(StackItem::Molecule(molecule));
            }
        }
    }

    let molecule = state.pop_molecule();
    assert!(state.stack.is_empty(), "unbalanced stack: {state:?}");
    molecule
}

impl State {
    fn pop_molecule(&mut self) -> Rc<AST> {
        let mut molecule;
        match self.stack.last().unwrap() {
            StackItem::Molecule(mol) => {
                molecule = mol.clone();
                self.stack.pop();
            }
            StackItem::Multiplicity(_) => {
                let num = self.pop_multiplicity();
                molecule = AST::Alkane(num).into();
            }
            _ => todo!(),
        };

        while let Some(StackItem::Molecule(group)) = self.stack.last() {
            let group = group.clone();
            self.stack.pop();

            for pos in self.pop_multiplicity_and_positions() {
                molecule = AST::Substitution(pos, group.clone(), molecule).into();
            }
        }

        molecule
    }

    fn pop_multiplicity_and_positions(&mut self) -> impl Iterator<Item = Locant> + '_ {
        let multiplicity = self.pop_multiplicity();

        (0..multiplicity).map(|_| {
            if let Some(&StackItem::Locant(pos)) = self.stack.last() {
                self.stack.pop();
                pos
            } else {
                Locant::Unspecified
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test::{ADENINE, CAFFEINE, CYTOSINE, DOPAMINE, GUANINE, SALBUTAMOL, THYMINE},
        Base, Element, Locant,
    };

    use super::{parse, AST};

    #[test]
    fn test_parse_simple() {
        assert_eq!(parse("Butane"), AST::Alkane(4).into());

        assert_eq!(
            parse("Ethene"),
            AST::Unsaturated(1, AST::Alkane(2).into()).into(),
        );

        assert_eq!(
            parse("Hexamethylpentane"),
            AST::Substitution(
                Locant::Unspecified,
                AST::Group(AST::Alkane(1).into()).into(),
                AST::Substitution(
                    Locant::Unspecified,
                    AST::Group(AST::Alkane(1).into()).into(),
                    AST::Substitution(
                        Locant::Unspecified,
                        AST::Group(AST::Alkane(1).into()).into(),
                        AST::Substitution(
                            Locant::Unspecified,
                            AST::Group(AST::Alkane(1).into()).into(),
                            AST::Substitution(
                                Locant::Unspecified,
                                AST::Group(AST::Alkane(1).into()).into(),
                                AST::Substitution(
                                    Locant::Unspecified,
                                    AST::Group(AST::Alkane(1).into()).into(),
                                    AST::Alkane(5).into(),
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
            AST::Substitution(
                Locant::Number(2),
                AST::Group(AST::Alkane(1).into()).into(),
                AST::Substitution(
                    Locant::Number(2),
                    AST::Group(AST::Alkane(1).into()).into(),
                    AST::Alkane(3).into(),
                )
                .into(),
            )
            .into(),
        );

        assert_eq!(
            parse("Pentyne"),
            AST::Unsaturated(2, AST::Alkane(5).into()).into(),
        );
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse(DOPAMINE),
            AST::Substitution(
                Locant::Number(1),
                AST::Group(AST::Base(Base::Water).into()).into(),
                AST::Substitution(
                    Locant::Number(2),
                    AST::Group(AST::Base(Base::Water).into()).into(),
                    AST::Substitution(
                        Locant::Number(4),
                        AST::Group(
                            AST::Substitution(
                                Locant::Number(2),
                                AST::Group(AST::Base(Base::Ammonia).into()).into(),
                                AST::Alkane(2).into(),
                            )
                            .into(),
                        )
                        .into(),
                        AST::Base(Base::Benzene).into(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        );

        assert_eq!(
            parse(SALBUTAMOL),
            AST::Substitution(
                Locant::Unspecified,
                AST::Group(AST::Base(Base::Water).into()).into(),
                AST::Substitution(
                    Locant::Number(4),
                    // 2-(tert-Butylamino)-1-hydroxyethyl
                    AST::Group(
                        AST::Substitution(
                            Locant::Number(2),
                            // tert-Butylamino
                            AST::Substitution(
                                Locant::Unspecified,
                                AST::Group(AST::Base(Base::Isobutane).into()).into(),
                                AST::Group(AST::Base(Base::Ammonia).into()).into(),
                            )
                            .into(),
                            // 1-Hydroxyethane
                            AST::Substitution(
                                Locant::Number(1),
                                AST::Group(AST::Base(Base::Water).into()).into(),
                                AST::Alkane(2).into(),
                            )
                            .into(),
                        )
                        .into(),
                    )
                    .into(),
                    // 2-(Hydroxymethyl)benzene
                    AST::Substitution(
                        Locant::Number(2),
                        AST::Group(
                            // Hydroxymethane
                            AST::Substitution(
                                Locant::Unspecified,
                                AST::Group(AST::Base(Base::Water).into()).into(),
                                AST::Alkane(1).into(),
                            )
                            .into(),
                        )
                        .into(),
                        AST::Base(Base::Benzene).into(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        );

        assert_eq!(
            parse(CAFFEINE),
            AST::Substitution(
                Locant::Number(2),
                AST::Group(AST::Base(Base::Oxygen).into()).into(),
                AST::Substitution(
                    Locant::Number(6),
                    AST::Group(AST::Base(Base::Oxygen).into()).into(),
                    // 1,3,7-Trimethyl-3,7-dihydro-1H-purine
                    AST::Substitution(
                        Locant::Number(1),
                        AST::Group(AST::Alkane(1).into()).into(),
                        AST::Substitution(
                            Locant::Number(3),
                            AST::Group(AST::Alkane(1).into()).into(),
                            AST::Substitution(
                                Locant::Number(7),
                                AST::Group(AST::Alkane(1).into()).into(),
                                // 3,7-Dihydro-1H-purine
                                AST::Substitution(
                                    Locant::Number(3),
                                    AST::Group(AST::Base(Base::Hydrogen).into()).into(),
                                    AST::Substitution(
                                        Locant::Number(7),
                                        AST::Group(AST::Base(Base::Hydrogen).into()).into(),
                                        // 1H-Purine
                                        AST::Isomer(
                                            Locant::Element(1, Element::Hydrogen),
                                            Base::Purine,
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
                )
                .into(),
            )
            .into(),
        );
    }

    #[test]
    fn test_parse_dna_bases() {
        parse(ADENINE);
        parse(THYMINE);
        parse(CYTOSINE);
        parse(GUANINE);
    }
}
