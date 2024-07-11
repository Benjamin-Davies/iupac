//! # P-11 Scope of nomenclature for organic compounds

use std::str::FromStr;

use paste::paste;
use serde::{de, Deserialize, Serialize};

macro_rules! elements {
    (#![doc = $doc:literal] $($symbol:ident $name:ident $group:literal)*) => {
        paste! {
            #[doc = $doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Element {
                $([<$name:camel>],)*
            }

            pub const ELEMENTS: &[Element] = &[
                $(Element::[<$name:camel>],)*
            ];

            impl Element {
                pub fn symbol(&self) -> &'static str {
                    match self {
                        $(Element::[<$name:camel>] => stringify!($symbol),)*
                    }
                }

                pub fn from_symbol(symbol: &str) -> Option<Element> {
                    match symbol {
                        $(stringify!($symbol) => Some(Element::[<$name:camel>]),)*
                        _ => None,
                    }
                }

                pub fn group(&self) -> u32 {
                    match self {
                        $(Element::[<$name:camel>] => $group,)*
                    }
                }
            }
        }
    };
}

elements! {
    //! Table 1.1 Elements included in these recommendations

    // Include hydrogen to aid with graph construction
    H   hydrogen      1

    B   boron         13
    C   carbon        14
    N   nitrogen      15
    O   oxygen        16
    F   fluorine      17

    Al  aluminium     13
    Si  silicon       14
    P   phosphorus    15
    S   sulfur        16
    Cl  chlorine      17

    Ga  gallium       13
    Ge  germanium     14
    As  arsenic       15
    Se  selenium      16
    Br  bromine       17

    In  indium        13
    Sn  tin           14
    Sb  antimony      15
    Te  tellurium     16
    I   iodine        17

    Tl  thallium      13
    Pb  lead          14
    Bi  bismuth       15
    Po  polonium      16
    At  astatine      17
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// https://en.wikipedia.org/wiki/Chemical_formula#Hill_system
impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum Hill {
            C,
            H,
            Other(&'static str),
        }

        let hill = |element: &Element| match element {
            Element::Carbon => Hill::C,
            Element::Hydrogen => Hill::H,
            element => Hill::Other(element.symbol()),
        };

        hill(self).cmp(&hill(other))
    }
}

impl FromStr for Element {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_symbol(s).ok_or("Unknown element")
    }
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.symbol())
    }
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Element, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Element::from_str(s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::Element;

    #[test]
    fn test_sort_elements() {
        let mut elements = vec![
            Element::Hydrogen,
            Element::Oxygen,
            Element::Nitrogen,
            Element::Carbon,
        ];

        elements.sort();

        assert_eq!(
            elements,
            vec![
                Element::Carbon,
                Element::Hydrogen,
                Element::Nitrogen,
                Element::Oxygen,
            ]
        );
    }
}
