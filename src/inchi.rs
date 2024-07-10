use std::{collections::BTreeMap, str::FromStr};

use crate::Element;

#[derive(Default)]
pub struct InChI {
    formula: Formula,
}

#[derive(Default)]
pub struct Formula {
    atom_counts: BTreeMap<Element, u32>,
}

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

        Ok(Self { formula })
    }
}

impl FromStr for Formula {
    type Err = &'static str;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut atom_counts = BTreeMap::new();
        while !s.is_empty() {
            let len = s.find(char::is_numeric).unwrap_or(s.len());
            let element = s[..len].parse()?;
            s = &s[len..];

            let len = s.find(char::is_alphabetic).unwrap_or(s.len());
            let count;
            if len == 0 {
                count = 1;
            } else {
                count = s[..len].parse().map_err(|_| "Invalid count")?;
                s = &s[len..];
            }

            atom_counts.insert(element, count);
        }

        Ok(Self { atom_counts })
    }
}
