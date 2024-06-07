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

        dfa.insert(b"(", Token::OpenBracket);
        dfa.insert(b"[", Token::OpenBracket);
        dfa.insert(b")", Token::CloseBracket);
        dfa.insert(b"]", Token::CloseBracket);

        // P-14.2 MULTIPLICATIVE PREFIXES
        dfa.insert(b"mon", Token::Multiple(1));
        dfa.insert(b"hen", Token::Multiple(1));
        dfa.insert(b"di", Token::Multiple(2));
        dfa.insert(b"do", Token::Multiple(2));
        dfa.insert(b"tri", Token::Multiple(3));
        dfa.insert(b"tetr", Token::Multiple(4));
        dfa.insert(b"pent", Token::Multiple(5));
        dfa.insert(b"hex", Token::Multiple(6));
        dfa.insert(b"hept", Token::Multiple(7));
        dfa.insert(b"oct", Token::Multiple(8));
        dfa.insert(b"non", Token::Multiple(9));
        dfa.insert(b"dec", Token::Multiple(10));
        dfa.insert(b"undec", Token::Multiple(11));

        dfa.insert(b"meth", Token::Multiple(1));
        dfa.insert(b"eth", Token::Multiple(2));
        dfa.insert(b"prop", Token::Multiple(3));
        dfa.insert(b"but", Token::Multiple(4));

        // P-29.2 GENERAL METHODOLOGY FOR NAMING SUBSTITUENT GROUPS
        dfa.insert(b"yl", Token::FreeValence);

        dfa.insert(b"an", Token::Unsaturated(0));
        dfa.insert(b"en", Token::Unsaturated(1));
        dfa.insert(b"yn", Token::Unsaturated(2));

        dfa.insert(b"water", Token::Base(Base::Water));
        dfa.insert(b"ammonia", Token::Base(Base::Ammonia));
        dfa.insert(b"benzene", Token::Base(Base::Benzene));
        dfa.insert(b"phen", Token::Base(Base::Benzene));
        dfa.insert(b"purin", Token::Base(Base::Purine));

        dfa.insert(b"hydr", Token::Prefix(Base::Hydrogen));
        dfa.insert(b"oxy", Token::Prefix(Base::Oxygen));
        dfa.insert(b"hydroxy", Token::Prefix(Base::Water));
        dfa.insert(b"amino", Token::Prefix(Base::Ammonia));
        dfa.insert(b"tert-butyl", Token::Prefix(Base::Isobutane));

        dfa.insert(b"one", Token::Suffix(Base::Oxygen));
        dfa.insert(b"ol", Token::Suffix(Base::Water));

        dfa
    };

    static ref ELEMENTS: dfa::Automaton<Element> = {
        let mut dfa = dfa::Automaton::new();

        dfa.insert(b"H", Element::Hydrogen);

        dfa.insert(b"B", Element::Boron);
        dfa.insert(b"C", Element::Carbon);
        dfa.insert(b"N", Element::Nitrogen);
        dfa.insert(b"O", Element::Oxygen);
        dfa.insert(b"F", Element::Fluorine);

        dfa.insert(b"Al", Element::Aluminum);
        dfa.insert(b"Si", Element::Silicon);
        dfa.insert(b"P", Element::Phosphorus);
        dfa.insert(b"S", Element::Sulfur);
        dfa.insert(b"Cl", Element::Chlorine);

        dfa.insert(b"Ga", Element::Gallium);
        dfa.insert(b"Ge", Element::Germanium);
        dfa.insert(b"As", Element::Arsenic);
        dfa.insert(b"Se", Element::Selenium);
        dfa.insert(b"Br", Element::Bromine);

        dfa.insert(b"In", Element::Indium);
        dfa.insert(b"Sn", Element::Tin);
        dfa.insert(b"Sb", Element::Antimony);
        dfa.insert(b"Te", Element::Tellurium);
        dfa.insert(b"I", Element::Iodine);

        dfa.insert(b"Tl", Element::Thallium);
        dfa.insert(b"Pb", Element::Lead);
        dfa.insert(b"Bi", Element::Bismuth);
        dfa.insert(b"Po", Element::Polonium);
        dfa.insert(b"At", Element::Astatine);

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

            let element =
                if let Some((len, &element)) = ELEMENTS.get_by_prefix(self.input.as_bytes()) {
                    self.input = &self.input[len..];
                    Some(element)
                } else {
                    None
                };

            return Some(Token::Position(num, element));
        }

        if let Some((len, token)) = TOKENS.get_by_prefix(self.input.as_bytes()) {
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
            scan("butane").collect::<Vec<_>>(),
            vec![Token::Multiple(4), Token::Unsaturated(0)],
        );

        assert_eq!(
            scan("ethene").collect::<Vec<_>>(),
            vec![Token::Multiple(2), Token::Unsaturated(1)],
        );

        assert_eq!(
            scan("hexamethylpentane").collect::<Vec<_>>(),
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
            scan("methdiylmethane").collect::<Vec<_>>(),
            vec![
                Token::Multiple(1),
                Token::Multiple(2),
                Token::FreeValence,
                Token::Multiple(1),
                Token::Unsaturated(0),
            ],
        );

        assert_eq!(
            scan("pentyne").collect::<Vec<_>>(),
            vec![Token::Multiple(5), Token::Unsaturated(2)],
        );
    }

    #[test]
    fn test_complex() {
        // Dopamine
        assert_eq!(
            scan("4-(2-aminoethyl)benzene-1,2-diol").collect::<Vec<_>>(),
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
            scan("4-[2-(tert-butylamino)-1-hydroxyethyl]-2-(hydroxymethyl)phenol")
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
            scan("1,3,7-trimethyl-3,7-dihydro-1H-purine-2,6-dione").collect::<Vec<_>>(),
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
