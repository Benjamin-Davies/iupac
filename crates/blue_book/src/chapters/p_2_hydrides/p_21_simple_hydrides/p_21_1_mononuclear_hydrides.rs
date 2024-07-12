//! # P-21.1 Mononuclear Parent Hydrides

use parsing::dfa;
use paste::paste;

use crate::{plugin::Plugin, scanner::Token, Element};

use super::SimpleHydride;

pub struct MononuclearHydridesPlugin;

pub const fn mononuclear_hydride(element: Element) -> SimpleHydride {
    SimpleHydride { length: 1, element }
}

macro_rules! mononuclear_hydrides {
    ($($name:ident($element:ident),)*) => {
        paste! {
            $(
                pub const [<$name:upper>]: SimpleHydride = mononuclear_hydride(Element::[<$element:camel>]);
            )*

            pub const MONONUCLEAR_HYDRIDES: &[(&str, SimpleHydride)] = &[
                $((stringify!($name), [<$name:upper>]),)*
            ];
        }
    };
}

mononuclear_hydrides! {
    borane(boron),
    carbane(carbon),
    azane(nitrogen),
    oxidane(oxygen),
    fluorane(fluorine),
    alumane(aluminium),
    silane(silicon),
    phosphane(phosphorus),
    sulfane(sulfur),
    chlorane(chlorine),
    gallane(gallium),
    germane(germanium),
    arsane(arsenic),
    selane(selenium),
    bromane(bromine),
    iodane(iodine),
    thallane(thallium),
    plumbane(lead),
    bismuthane(bismuth),
    polane(polonium),
    astane(astatine),

    methane(carbon),
    ammonia(nitrogen),
    water(oxygen),
}

impl Plugin for MononuclearHydridesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        for &(mut name, structure) in MONONUCLEAR_HYDRIDES {
            if let Some(prefix) = name.strip_suffix("ane") {
                name = prefix;
            }
            dfa.insert(name, Token::Hydride(structure.into()));
        }
    }
}
