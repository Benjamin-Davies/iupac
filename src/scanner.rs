use lazy_static::lazy_static;

use crate::dfa;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// "mono", "di", "tri", etc. and "meth", "eth", "prop", "but"
    Multiple(u8),
    /// "ane", "ene", "yne"
    Unsaturated(u8),
    /// "yl", "diyl", "triyl", "tetrayl"
    FreeValence(u8),
}

lazy_static! {
    static ref DFA: dfa::Automaton<Token> = {
        let mut dfa = dfa::Automaton::new();
        // Most vowel suffixes removed to avoid conflicts with "-ane", "-ol", etc.

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
        dfa.insert(b"yl", Token::FreeValence(1));
        dfa.insert(b"diyl", Token::FreeValence(2));
        dfa.insert(b"triyl", Token::FreeValence(3));
        dfa.insert(b"tetrayl", Token::FreeValence(4));

        dfa.insert(b"an", Token::Unsaturated(0));
        dfa.insert(b"en", Token::Unsaturated(1));
        dfa.insert(b"yn", Token::Unsaturated(2));

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
        if let Some((len, token)) = DFA.get_by_prefix(self.input.as_bytes()) {
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
    use super::{scan, Token};

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
                Token::FreeValence(1),
                Token::Multiple(5),
                Token::Unsaturated(0),
            ],
        );
        assert_eq!(
            // A.k.a. "ethene"
            scan("methdiylmethane").collect::<Vec<_>>(),
            vec![
                Token::Multiple(1),
                Token::FreeValence(2),
                Token::Multiple(1),
                Token::Unsaturated(0),
            ],
        );
        assert_eq!(
            scan("pentyne").collect::<Vec<_>>(),
            vec![Token::Multiple(5), Token::Unsaturated(2)],
        );
    }
}
