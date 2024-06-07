use lazy_static::lazy_static;

use crate::dfa;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// "(", "["
    OpenBracket,
    /// ")", "]"
    CloseBracket,

    /// "1-", "2-", "3-", "1H-", etc.
    Position(u8, Option<Element>),
    /// "mono", "di", "tri", etc. and "meth", "eth", "prop", "but"
    Multiple(u8),
    /// "ane", "ene", "yne"
    Unsaturated(u8),
    /// "yl"
    FreeValence,

    /// A named base: "water", "ammonia", etc.
    Base(Base),
    /// A named base in prefix form: "hydroxy", "amino", etc.
    Prefix(Base),
    /// A named base in suffix form: "hydroxy", "amine", etc.
    Suffix(Base),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Hydrogen,
    Oxygen,
    Water,
    Ammonia,
    Isobutane,
    Benzene,
    Purine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Hydrogen,

    // P-11 SCOPE OF NOMENCLATURE FOR ORGANIC COMPOUNDS
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,

    Aluminum,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,

    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,

    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,

    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
}

lazy_static! {
    static ref TOKENS: dfa::Automaton<Token> = {
        let mut dfa = dfa::Automaton::new();
        // Most vowel suffixes removed to avoid conflicts with "-ane", "-ol", etc.

        dfa.insert("(", Token::OpenBracket);
        dfa.insert("[", Token::OpenBracket);
        dfa.insert(")", Token::CloseBracket);
        dfa.insert("]", Token::CloseBracket);

        // P-14.2 MULTIPLICATIVE PREFIXES
        dfa.insert("mon", Token::Multiple(1));
        dfa.insert("hen", Token::Multiple(1));
        dfa.insert("di", Token::Multiple(2));
        dfa.insert("do", Token::Multiple(2));
        dfa.insert("tri", Token::Multiple(3));
        dfa.insert("tetr", Token::Multiple(4));
        dfa.insert("pent", Token::Multiple(5));
        dfa.insert("hex", Token::Multiple(6));
        dfa.insert("hept", Token::Multiple(7));
        dfa.insert("oct", Token::Multiple(8));
        dfa.insert("non", Token::Multiple(9));
        dfa.insert("dec", Token::Multiple(10));
        dfa.insert("undec", Token::Multiple(11));

        dfa.insert("meth", Token::Multiple(1));
        dfa.insert("eth", Token::Multiple(2));
        dfa.insert("prop", Token::Multiple(3));
        dfa.insert("but", Token::Multiple(4));

        // P-29.2 GENERAL METHODOLOGY FOR NAMING SUBSTITUENT GROUPS
        dfa.insert("yl", Token::FreeValence);

        dfa.insert("an", Token::Unsaturated(0));
        dfa.insert("en", Token::Unsaturated(1));
        dfa.insert("yn", Token::Unsaturated(2));

        dfa.insert("water", Token::Base(Base::Water));
        dfa.insert("ammonia", Token::Base(Base::Ammonia));
        dfa.insert("benzene", Token::Base(Base::Benzene));
        dfa.insert("phen", Token::Base(Base::Benzene));
        dfa.insert("purin", Token::Base(Base::Purine));

        dfa.insert("hydr", Token::Prefix(Base::Hydrogen));
        dfa.insert("oxy", Token::Prefix(Base::Oxygen));
        dfa.insert("hydroxy", Token::Prefix(Base::Water));
        dfa.insert("amino", Token::Prefix(Base::Ammonia));
        dfa.insert("tert-butyl", Token::Prefix(Base::Isobutane));

        dfa.insert("one", Token::Suffix(Base::Oxygen));
        dfa.insert("ol", Token::Suffix(Base::Water));

        dfa
    };

    static ref ELEMENTS: dfa::Automaton<Element> = {
        let mut dfa = dfa::Automaton::new();

        dfa.insert("H", Element::Hydrogen);

        dfa.insert("B", Element::Boron);
        dfa.insert("C", Element::Carbon);
        dfa.insert("N", Element::Nitrogen);
        dfa.insert("O", Element::Oxygen);
        dfa.insert("F", Element::Fluorine);

        dfa.insert("Al", Element::Aluminum);
        dfa.insert("Si", Element::Silicon);
        dfa.insert("P", Element::Phosphorus);
        dfa.insert("S", Element::Sulfur);
        dfa.insert("Cl", Element::Chlorine);

        dfa.insert("Ga", Element::Gallium);
        dfa.insert("Ge", Element::Germanium);
        dfa.insert("As", Element::Arsenic);
        dfa.insert("Se", Element::Selenium);
        dfa.insert("Br", Element::Bromine);

        dfa.insert("In", Element::Indium);
        dfa.insert("Sn", Element::Tin);
        dfa.insert("S", Element::Antimony);
        dfa.insert("Te", Element::Tellurium);
        dfa.insert("I", Element::Iodine);

        dfa.insert("Tl", Element::Thallium);
        dfa.insert("P", Element::Lead);
        dfa.insert("Bi", Element::Bismuth);
        dfa.insert("Po", Element::Polonium);
        dfa.insert("At", Element::Astatine);

        dfa
    };
}

#[derive(Debug)]
pub struct Scanner<'input> {
    input: &'input str,
}

pub fn scan(input: &str) -> Scanner {
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

            let element = if let Some((len, &element)) = ELEMENTS.get_by_prefix(self.input) {
                self.input = &self.input[len..];
                Some(element)
            } else {
                None
            };

            return Some(Token::Position(num, element));
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
    use crate::scanner::Element;

    use super::{scan, Base, Token};

    #[test]
    fn test_parse_simple() {
        assert_eq!(
            scan("Butane").collect::<Vec<_>>(),
            vec![Token::Multiple(4), Token::Unsaturated(0)],
        );

        assert_eq!(
            scan("Ethene").collect::<Vec<_>>(),
            vec![Token::Multiple(2), Token::Unsaturated(1)],
        );

        assert_eq!(
            scan("Hexamethylpentane").collect::<Vec<_>>(),
            vec![
                Token::Multiple(6),
                Token::Multiple(1),
                Token::FreeValence,
                Token::Multiple(5),
                Token::Unsaturated(0),
            ],
        );

        // Ethene
        assert_eq!(
            scan("Methdiylmethane").collect::<Vec<_>>(),
            vec![
                Token::Multiple(1),
                Token::Multiple(2),
                Token::FreeValence,
                Token::Multiple(1),
                Token::Unsaturated(0),
            ],
        );

        assert_eq!(
            scan("Pentyne").collect::<Vec<_>>(),
            vec![Token::Multiple(5), Token::Unsaturated(2)],
        );
    }

    #[test]
    fn test_complex() {
        // Dopamine
        assert_eq!(
            scan("4-(2-Aminoethyl)benzene-1,2-diol").collect::<Vec<_>>(),
            vec![
                Token::Position(4, None),
                Token::OpenBracket,
                Token::Position(2, None),
                Token::Prefix(Base::Ammonia),
                Token::Multiple(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Position(1, None),
                Token::Position(2, None),
                Token::Multiple(2),
                Token::Suffix(Base::Water),
            ],
        );

        // Salbutamol
        assert_eq!(
            scan("4-[2-(tert-Butylamino)-1-hydroxyethyl]-2-(hydroxymethyl)phenol")
                .collect::<Vec<_>>(),
            vec![
                Token::Position(4, None),
                Token::OpenBracket,
                Token::Position(2, None),
                Token::OpenBracket,
                Token::Prefix(Base::Isobutane),
                Token::Prefix(Base::Ammonia),
                Token::CloseBracket,
                Token::Position(1, None),
                Token::Prefix(Base::Water),
                Token::Multiple(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Position(2, None),
                Token::OpenBracket,
                Token::Prefix(Base::Water),
                Token::Multiple(1),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Suffix(Base::Water),
            ],
        );

        // Caffeine
        assert_eq!(
            scan("1,3,7-Trimethyl-3,7-dihydro-1H-purine-2,6-dione").collect::<Vec<_>>(),
            vec![
                Token::Position(1, None),
                Token::Position(3, None),
                Token::Position(7, None),
                Token::Multiple(3),
                Token::Multiple(1),
                Token::FreeValence,
                Token::Position(3, None),
                Token::Position(7, None),
                Token::Multiple(2),
                Token::Prefix(Base::Hydrogen),
                Token::Position(1, Some(Element::Hydrogen)),
                Token::Base(Base::Purine),
                Token::Position(2, None),
                Token::Position(6, None),
                Token::Multiple(2),
                Token::Suffix(Base::Oxygen),
            ],
        );
    }
}
