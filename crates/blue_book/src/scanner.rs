use lazy_static::lazy_static;

use parsing::dfa;

use crate::{named_structure::AnyNamedStructure, plugin::PLUGINS, Base, Element, Locant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// "(", "["
    OpenBracket,
    /// ")", "]"
    CloseBracket,

    /// "1-", "2-", "3-", "1H-", etc.
    Locant(Locant),
    /// "mono", "di", "tri", etc.
    Multiplicity(u16),
    /// "meth", "eth", "prop", "but"
    Alkane(u16),
    /// "ane", "ene", "yne"
    Unsaturated(u8),
    /// "yl"
    FreeValence,

    /// A named structure: "borane", "methane", etc.
    Structure(AnyNamedStructure),
    /// A named base: "water", "ammonia", etc.
    Base(Base),
    /// A named base in prefix form: "hydroxy", "amino", etc.
    Prefix(Base),
    /// A named base in suffix form: "hydroxy", "amine", etc.
    Suffix(Base),
}

lazy_static! {
    static ref TOKENS: dfa::Automaton<Token> = {
        let mut dfa = dfa::Automaton::new();
        // Most vowel suffixes removed to avoid conflicts with "-ane", "-ol", etc.

        dfa.insert("(", Token::OpenBracket);
        dfa.insert("[", Token::OpenBracket);
        dfa.insert(")", Token::CloseBracket);
        dfa.insert("]", Token::CloseBracket);

        for plugin in PLUGINS {
            plugin.init_tokens(&mut dfa);
        }

        dfa.insert("eth", Token::Alkane(2));
        dfa.insert("prop", Token::Alkane(3));
        dfa.insert("but", Token::Alkane(4));

        // P-29.2 GENERAL METHODOLOGY FOR NAMING SUBSTITUENT GROUPS
        dfa.insert("yl", Token::FreeValence);

        dfa.insert("an", Token::Unsaturated(0));
        dfa.insert("en", Token::Unsaturated(1));
        dfa.insert("yn", Token::Unsaturated(2));

        dfa.insert("water", Token::Base(Base::Water));
        dfa.insert("ammonia", Token::Base(Base::Ammonia));
        dfa.insert("benzene", Token::Base(Base::Benzene));
        dfa.insert("phen", Token::Base(Base::Benzene));
        dfa.insert("pyrimidin", Token::Base(Base::Pyrimidine));
        dfa.insert("purin", Token::Base(Base::Purine));

        dfa.insert("hydr", Token::Prefix(Base::Hydrogen));
        dfa.insert("oxy", Token::Prefix(Base::Oxygen));
        dfa.insert("hydroxy", Token::Prefix(Base::Water));
        dfa.insert("amino", Token::Prefix(Base::Ammonia));
        dfa.insert("tert-butyl", Token::Prefix(Base::Isobutane));

        dfa.insert("one", Token::Suffix(Base::Oxygen));
        dfa.insert("ol", Token::Suffix(Base::Water));
        dfa.insert("amine", Token::Suffix(Base::Ammonia));

        dfa
    };

    static ref ELEMENTS: dfa::Automaton<Element> = {
        let mut dfa = dfa::Automaton::new();
        for &element in crate::ELEMENTS {
            dfa.insert(element.symbol(), element);
        }
        dfa
    };
}

#[derive(Debug)]
pub struct Scanner<'input> {
    input: &'input str,
}

pub fn scan(input: &str) -> Scanner {
    // Trim common stereochemistry prefixes
    let input = input.trim_start_matches("(RS)-");

    Scanner { input }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

impl<'input> Iterator for Scanner<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.input.chars().next() {
            let len = c.len_utf8();
            match c {
                '-' | ',' => {
                    self.input = &self.input[len..];
                }
                _ => break,
            }
        }

        if self.input.starts_with(char::is_numeric) {
            let len = self
                .input
                .find(|c: char| !c.is_numeric())
                .unwrap_or(self.input.len());
            let (num, rest) = self.input.split_at(len);
            let num = num.parse::<u8>().unwrap();
            self.input = rest;

            let pos = if let Some((len, &element)) = ELEMENTS.get_by_prefix(self.input) {
                self.input = &self.input[len..];
                Locant::Element(num, element)
            } else {
                Locant::Number(num)
            };

            return Some(Token::Locant(pos));
        }

        if let Some((len, token)) = TOKENS.get_by_prefix_ignore_case(self.input) {
            self.input = &self.input[len..];
            Some(*token)
        } else if let Some(rest) = self.input.strip_prefix(is_vowel) {
            self.input = rest;
            self.next()
        } else {
            assert!(
                self.input.is_empty(),
                "unrecognized input: {:?}",
                self.input
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chapters::p_2_hydrides::p_21_simple_hydrides::p_21_1_mononuclear_hydrides::METHANE,
        named_structure::NamedStructure,
        test::{CAFFEINE, DOPAMINE, SALBUTAMOL},
        Base, Element, Locant,
    };

    use super::{scan, Token};

    #[test]
    fn test_scan_simple() {
        assert_eq!(
            scan("Butane").collect::<Vec<_>>(),
            vec![Token::Alkane(4), Token::Unsaturated(0)],
        );

        assert_eq!(
            scan("Ethene").collect::<Vec<_>>(),
            vec![Token::Alkane(2), Token::Unsaturated(1)],
        );

        assert_eq!(
            scan("Hexamethylpentane").collect::<Vec<_>>(),
            vec![
                Token::Multiplicity(6),
                Token::Structure(METHANE.as_any()),
                Token::FreeValence,
                Token::Multiplicity(5),
                Token::Unsaturated(0),
            ],
        );

        assert_eq!(
            scan("Pentyne").collect::<Vec<_>>(),
            vec![Token::Multiplicity(5), Token::Unsaturated(2)],
        );
    }

    #[test]
    fn test_scan_complex() {
        assert_eq!(
            scan(DOPAMINE).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(4)),
                Token::OpenBracket,
                Token::Locant(Locant::Number(2)),
                Token::Prefix(Base::Ammonia),
                Token::Alkane(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Locant(Locant::Number(1)),
                Token::Locant(Locant::Number(2)),
                Token::Multiplicity(2),
                Token::Suffix(Base::Water),
            ],
        );

        assert_eq!(
            scan(SALBUTAMOL).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(4)),
                Token::OpenBracket,
                Token::Locant(Locant::Number(2)),
                Token::OpenBracket,
                Token::Prefix(Base::Isobutane),
                Token::Prefix(Base::Ammonia),
                Token::CloseBracket,
                Token::Locant(Locant::Number(1)),
                Token::Prefix(Base::Water),
                Token::Alkane(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Locant(Locant::Number(2)),
                Token::OpenBracket,
                Token::Prefix(Base::Water),
                Token::Structure(METHANE.as_any()),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Suffix(Base::Water),
            ],
        );

        assert_eq!(
            scan(CAFFEINE).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(1)),
                Token::Locant(Locant::Number(3)),
                Token::Locant(Locant::Number(7)),
                Token::Multiplicity(3),
                Token::Structure(METHANE.as_any()),
                Token::FreeValence,
                Token::Locant(Locant::Number(3)),
                Token::Locant(Locant::Number(7)),
                Token::Multiplicity(2),
                Token::Prefix(Base::Hydrogen),
                Token::Locant(Locant::Element(1, Element::Hydrogen)),
                Token::Base(Base::Purine),
                Token::Locant(Locant::Number(2)),
                Token::Locant(Locant::Number(6)),
                Token::Multiplicity(2),
                Token::Suffix(Base::Oxygen),
            ],
        );
    }
}
