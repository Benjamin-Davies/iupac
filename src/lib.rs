use std::str::FromStr;

use paste::paste;
use serde::{Deserialize, Serialize};

pub mod dfa;
pub mod graph;
pub mod inchi;
pub mod parser;
pub mod scanner;
pub mod test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Hydrogen,
    Oxygen,
    Water,
    Ammonia,
    Isobutane,
    Benzene,
    Pyrimidine,
    Purine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Unspecified,
    Number(u8),
    Element(u8, Element),
}

impl Base {
    pub fn has_isomers(&self) -> bool {
        matches!(self, Base::Purine)
    }
}

macro_rules! elements {
    ($($symbol:ident $name:ident)*) => {
        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            }

            impl FromStr for Element {
                type Err = &'static str;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $(stringify!($symbol) => Ok(Element::[<$name:camel>]),)*
                        _ => Err("Unknown element"),
                    }
                }
            }
        }
    };
}

elements! {
    H hydrogen

    // P-11 SCOPE OF NOMENCLATURE FOR ORGANIC COMPOUNDS
    B boron
    C carbon
    N nitrogen
    O oxygen
    F fluorine

    Al aluminium
    Si silicon
    P phosphorus
    S sulfur
    Cl chlorine

    Ga gallium
    Ge germanium
    As arsenic
    Se selenium
    Br bromine

    In indium
    Tl thallium

    Sn tin
    Pb lead

    Sb antimony
    Bi bismuth

    Te tellurium
    Po polonium

    I iodine
    At astatine
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
