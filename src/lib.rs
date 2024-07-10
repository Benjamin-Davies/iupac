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
    ($($symbol:ident $name:ident $period:literal $group:literal)*) => {
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

                pub fn period(&self) -> u32 {
                    match self {
                        $(Element::[<$name:camel>] => $period,)*
                    }
                }

                pub fn group(&self) -> u32 {
                    match self {
                        $(Element::[<$name:camel>] => $group,)*
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
    H   hydrogen    1   1

    // P-11 SCOPE OF NOMENCLATURE FOR ORGANIC COMPOUNDS
    B   boron       2  13
    C   carbon      2  14
    N   nitrogen    2  15
    O   oxygen      2  16
    F   fluorine    2  17

    Al  aluminium   3  13
    Si  silicon     3  14
    P   phosphorus  3  15
    S   sulfur      3  16
    Cl  chlorine    3  17

    Ga  gallium     4  13
    Ge  germanium   4  14
    As  arsenic     4  15
    Se  selenium    4  16
    Br  bromine     4  17

    In  indium      5  13
    Sn  tin         5  14
    Sb  antimony    5  15
    Te  tellurium   5  16
    I   iodine      5  17

    Tl  thallium    6  13
    Pb  lead        6  14
    Bi  bismuth     6  15
    Po  polonium    6  16
    At  astatine    6  17
}

impl Element {
    // Private because this is a very naive way of generating these (i.e. a hack).
    fn standard_valence(&self) -> u8 {
        match self.group() {
            1 => 1,
            13 => 3,
            14 => 4,
            15 => 3,
            16 => 2,
            17 => 1,
            _ => 0,
        }
    }
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
