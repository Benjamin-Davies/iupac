use lazy_static::lazy_static;

use parsing::dfa;

use crate::{Base, Element, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// "(", "["
    OpenBracket,
    /// ")", "]"
    CloseBracket,

    /// "1-", "2-", "3-", "1H-", etc.
    Position(Position),
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
                Position::Element(num, element)
            } else {
                Position::Number(num)
            };

            return Some(Token::Position(pos));
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
        test::{CAFFEINE, DOPAMINE, SALBUTAMOL},
        Base, Element, Position,
    };

    use super::{scan, Token};

    #[test]
    fn test_scan_simple() {
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

        assert_eq!(
            scan("Pentyne").collect::<Vec<_>>(),
            vec![Token::Multiple(5), Token::Unsaturated(2)],
        );
    }

    #[test]
    fn test_scan_complex() {
        assert_eq!(
            scan(DOPAMINE).collect::<Vec<_>>(),
            vec![
                Token::Position(Position::Number(4)),
                Token::OpenBracket,
                Token::Position(Position::Number(2)),
                Token::Prefix(Base::Ammonia),
                Token::Multiple(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Position(Position::Number(1)),
                Token::Position(Position::Number(2)),
                Token::Multiple(2),
                Token::Suffix(Base::Water),
            ],
        );

        assert_eq!(
            scan(SALBUTAMOL).collect::<Vec<_>>(),
            vec![
                Token::Position(Position::Number(4)),
                Token::OpenBracket,
                Token::Position(Position::Number(2)),
                Token::OpenBracket,
                Token::Prefix(Base::Isobutane),
                Token::Prefix(Base::Ammonia),
                Token::CloseBracket,
                Token::Position(Position::Number(1)),
                Token::Prefix(Base::Water),
                Token::Multiple(2),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Position(Position::Number(2)),
                Token::OpenBracket,
                Token::Prefix(Base::Water),
                Token::Multiple(1),
                Token::FreeValence,
                Token::CloseBracket,
                Token::Base(Base::Benzene),
                Token::Suffix(Base::Water),
            ],
        );

        assert_eq!(
            scan(CAFFEINE).collect::<Vec<_>>(),
            vec![
                Token::Position(Position::Number(1)),
                Token::Position(Position::Number(3)),
                Token::Position(Position::Number(7)),
                Token::Multiple(3),
                Token::Multiple(1),
                Token::FreeValence,
                Token::Position(Position::Number(3)),
                Token::Position(Position::Number(7)),
                Token::Multiple(2),
                Token::Prefix(Base::Hydrogen),
                Token::Position(Position::Element(1, Element::Hydrogen)),
                Token::Base(Base::Purine),
                Token::Position(Position::Number(2)),
                Token::Position(Position::Number(6)),
                Token::Multiple(2),
                Token::Suffix(Base::Oxygen),
            ],
        );
    }
}
