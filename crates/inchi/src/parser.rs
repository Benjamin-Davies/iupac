use std::{collections::BTreeMap, str::FromStr};

use super::{
    scanner::{Scanner, Token},
    Connections, Formula, Hydrogens, InChI,
};

impl FromStr for InChI {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/').peekable();

        if parts.next() != Some("InChI=1S") {
            return Err("String does not contain InChI prefix");
        }

        let mut formula = Formula::default();
        if let Some(part) = parts.next() {
            formula = part.parse()?;
        }

        let mut connections = Connections::default();
        if let Some(part) = parts.next().and_then(|p| p.strip_prefix('c')) {
            connections = part.parse()?;
        }

        let mut hydrogens = Hydrogens::default();
        if let Some(part) = parts.next().and_then(|p| p.strip_prefix('h')) {
            hydrogens = part.parse()?;
        }

        Ok(Self {
            formula,
            connections,
            hydrogens,
        })
    }
}

impl FromStr for Formula {
    type Err = &'static str;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut atom_counts = BTreeMap::new();

        while !s.is_empty() {
            let len = s[1..]
                .find(|c: char| !c.is_ascii_lowercase())
                .map_or(s.len(), |i| i + 1);
            let element = s[..len].parse()?;
            s = &s[len..];

            let len = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
            let count;
            if len == 0 {
                count = 1;
            } else {
                count = s[..len].parse().unwrap();
                s = &s[len..];
            }

            atom_counts.insert(element, count);
        }

        Ok(Self { atom_counts })
    }
}

impl FromStr for Connections {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections = Vec::new();

        let scanner = Scanner::new(s);
        let mut last_index = None;
        let mut last_last_index = None;
        let mut stack = Vec::new();
        for token in scanner {
            match token {
                Token::Numeric(i) => {
                    if let Some(last_index) = last_index {
                        connections.push((last_index, i));
                    }
                    last_last_index = last_index;
                    last_index = Some(i);
                }
                Token::Hyphen => { /* used to separate consecutive numbers */ }
                Token::Comma => {
                    last_index = last_last_index;
                    last_last_index = None;
                }
                Token::LParen => {
                    stack.push(last_index);
                }
                Token::RParen => {
                    last_index = stack.pop().ok_or("Unmatched right parenthesis")?;
                }
                _ => return Err("Unexpected token in connections layer"),
            }
        }

        Ok(Self { connections })
    }
}

impl FromStr for Hydrogens {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut immobile_hydrogens = Vec::new();
        let mut mobile_hydrogens = Vec::new();

        let mut scanner = Scanner::new(s);
        loop {
            if scanner.lookahead_numeric() {
                // immobile hydrogens
                let mut ranges = Vec::new();
                loop {
                    let start = scanner.expect_numeric();

                    let end;
                    if scanner.lookahead_hyphen() {
                        scanner.expect(Token::Hyphen);
                        end = scanner.expect_numeric();
                    } else {
                        end = start;
                    }

                    ranges.push(start..=end);

                    if scanner.lookahead_comma() {
                        scanner.expect(Token::Comma);
                    } else {
                        break;
                    }
                }

                scanner.expect(Token::Alpha("H"));

                let count;
                if scanner.lookahead_numeric() {
                    count = scanner.expect_numeric();
                } else {
                    count = 1;
                }

                immobile_hydrogens.push((ranges, count));
            } else {
                // mobile hydrogens
                scanner.expect(Token::LParen);
                scanner.expect(Token::Alpha("H"));

                let count;
                if scanner.lookahead_numeric() {
                    count = scanner.expect_numeric();
                } else {
                    count = 1;
                }

                scanner.expect(Token::Comma);

                let mut indices = Vec::new();
                loop {
                    let index = scanner.expect_numeric();
                    indices.push(index);
                    if scanner.lookahead_comma() {
                        scanner.expect(Token::Comma);
                    } else {
                        break;
                    }
                }

                scanner.expect(Token::RParen);

                mobile_hydrogens.push((count, indices));
            }

            if scanner.lookahead_comma() {
                scanner.expect(Token::Comma);
            } else {
                break;
            }
        }

        Ok(Self {
            immobile_hydrogens,
            mobile_hydrogens,
        })
    }
}
