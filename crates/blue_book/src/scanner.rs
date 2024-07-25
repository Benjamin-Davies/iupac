use lazy_static::lazy_static;

use parsing::dfa;

use crate::{
    chapters::{p_2_hydrides::Hydride, p_3_substituent_groups::CharacteristicGroup},
    plugin::PLUGINS,
    Element, Locant,
};

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
    /// "ane", "ene", "yne"
    Unsaturated(u8),
    /// "yl"
    FreeValence,

    /// A parent hydride: "borane", "ethane", "cyclohexane", etc.
    Hydride(Hydride),
    /// A named base in prefix form: "hydroxy", "amino", etc.
    Prefix(CharacteristicGroup),
    /// A named base in suffix form: "hydroxy", "amine", etc.
    Suffix(CharacteristicGroup),
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

/// Undoes the capitalization mentioned in P-16.0 (Name writing / Introduction).
///
/// # Examples
///
/// ```rust
/// use blue_book::scanner::uncapitalize;
///
/// assert_eq!(uncapitalize("Ethanol"), "ethanol");
/// assert_eq!(uncapitalize("N-Methylphenethylamine"), "N-methylphenethylamine");
/// ```
pub fn uncapitalize(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut i = 0;
    let mut j = input.chars().next().unwrap().len_utf8();
    while j < input.len() {
        let a = input[i..].chars().next().unwrap();
        let b = input[j..].chars().next().unwrap();

        if a.is_ascii_uppercase() && b.is_ascii_lowercase() {
            let mut result = String::with_capacity(input.len());
            result.push_str(&input[..i]);
            result.push(a.to_ascii_lowercase());
            result.push_str(&input[j..]);
            return result;
        }

        i = j;
        j = i + b.len_utf8();
    }

    input.to_owned()
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

        if let Some((len, &token)) = TOKENS.get_by_prefix(self.input) {
            self.input = &self.input[len..];
            return Some(token);
        }

        if self.input.starts_with(char::is_numeric) {
            let len = self
                .input
                .find(|c: char| !c.is_numeric())
                .unwrap_or(self.input.len());
            let (num, rest) = self.input.split_at(len);
            let num = num.parse::<u16>().unwrap();
            self.input = rest;

            let pos = if let Some((len, &element)) = ELEMENTS.get_by_prefix(self.input) {
                self.input = &self.input[len..];
                Locant::Element(num, element)
            } else {
                Locant::Number(num)
            };

            return Some(Token::Locant(pos));
        }

        if let Some(rest) = self.input.strip_prefix(is_vowel) {
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
        chapters::{
            p_2_hydrides::{
                p_21_simple_hydrides::{
                    p_21_1_mononuclear_hydrides::METHANE,
                    p_21_2_acyclic_hydrides::{BUTANE, ETHANE},
                },
                p_22_monocyclic_hydrides::p_22_1_monocyclic_hydocarbons::MonocyclicHydrocarbon::Benzene,
                p_25_fused_ring_systems::p_25_2_heterocyclic_ring_components::HeterocyclicRing::Purine,
                Hydride::Isobutane,
            },
            p_3_substituent_groups::CharacteristicGroup,
        },
        scanner::uncapitalize,
        test::{CAFFEINE, DOPAMINE, SALBUTAMOL},
        Locant,
    };

    use super::{scan, Token};

    #[test]
    fn test_scan_simple() {
        assert_eq!(
            scan("butane").collect::<Vec<_>>(),
            vec![Token::Hydride(BUTANE.into()), Token::Unsaturated(0)],
        );

        assert_eq!(
            scan("ethene").collect::<Vec<_>>(),
            vec![Token::Hydride(ETHANE.into()), Token::Unsaturated(1)],
        );

        assert_eq!(
            scan("hexamethylpentane").collect::<Vec<_>>(),
            vec![
                Token::Multiplicity(6),
                Token::Hydride(METHANE.into()),
                Token::FreeValence,
                Token::Multiplicity(5),
                Token::Unsaturated(0),
            ],
        );

        assert_eq!(
            scan("pentyne").collect::<Vec<_>>(),
            vec![Token::Multiplicity(5), Token::Unsaturated(2)],
        );
    }

    #[test]
    fn test_scan_complex() {
        assert_eq!(
            scan(&uncapitalize(DOPAMINE)).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(4)),
                Token::OpenBracket,
                Token::Locant(Locant::Number(2)),
                Token::Prefix(CharacteristicGroup::Amino),
                Token::Hydride(ETHANE.into()),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Hydride(Benzene.into()),
                Token::Locant(Locant::Number(1)),
                Token::Locant(Locant::Number(2)),
                Token::Multiplicity(2),
                Token::Suffix(CharacteristicGroup::Hydroxy),
            ],
        );

        assert_eq!(
            scan(&uncapitalize(SALBUTAMOL)).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(4)),
                Token::OpenBracket,
                Token::Locant(Locant::Number(2)),
                Token::OpenBracket,
                Token::Hydride(Isobutane),
                Token::FreeValence,
                Token::Prefix(CharacteristicGroup::Amino),
                Token::CloseBracket,
                Token::Locant(Locant::Number(1)),
                Token::Prefix(CharacteristicGroup::Hydroxy),
                Token::Hydride(ETHANE.into()),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Locant(Locant::Number(2)),
                Token::OpenBracket,
                Token::Prefix(CharacteristicGroup::Hydroxy),
                Token::Hydride(METHANE.into()),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Hydride(Benzene.into()),
                Token::Suffix(CharacteristicGroup::Hydroxy),
            ],
        );

        assert_eq!(
            scan(&uncapitalize(CAFFEINE)).collect::<Vec<_>>(),
            vec![
                Token::Locant(Locant::Number(1)),
                Token::Locant(Locant::Number(3)),
                Token::Locant(Locant::Number(7)),
                Token::Multiplicity(3),
                Token::Hydride(METHANE.into()),
                Token::FreeValence,
                Token::Locant(Locant::Number(3)),
                Token::Locant(Locant::Number(7)),
                Token::Multiplicity(2),
                Token::Prefix(CharacteristicGroup::Hydro),
                Token::Hydride(Purine(1).into()),
                Token::Locant(Locant::Number(2)),
                Token::Locant(Locant::Number(6)),
                Token::Multiplicity(2),
                Token::Suffix(CharacteristicGroup::Oxo),
            ],
        );
    }
}
