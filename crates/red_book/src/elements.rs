use std::str::FromStr;

use paste::paste;
use serde::{de, Deserialize, Serialize};

macro_rules! elements {
    ($($symbol:ident $name:ident $period:literal $group:literal)*) => {
        paste! {
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
        }
    };
}

elements! {
    H   hydrogen    1   1

    // Elements selected from Blue Book P-11 SCOPE OF NOMENCLATURE FOR ORGANIC COMPOUNDS
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
    /// Blue Book P-14.1 BONDING NUMBER
    pub fn standard_bonding_number(self) -> u8 {
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
