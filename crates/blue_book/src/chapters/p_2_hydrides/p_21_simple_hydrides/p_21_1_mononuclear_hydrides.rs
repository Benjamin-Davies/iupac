//! # P-21.1 Mononuclear Parent Hydrides

use parsing::dfa;
use paste::paste;

use crate::{
    graph::Graph, named_structure::NamedStructure, plugin::Plugin, scanner::Token, Element, Locant,
};

pub struct MononuclearHydridesPlugin;

#[derive(Debug)]
pub struct MononuclearHydride(pub Element);

macro_rules! mononuclear_hydrides {
    ($($name:ident($element:ident),)*) => {
        paste! {
            $(pub const [<$name:upper>]: MononuclearHydride = MononuclearHydride(Element::[<$element:camel>]);)*

            pub const MONONUCLEAR_HYDRIDES: &[(&str, MononuclearHydride)] = &[
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
}

impl Plugin for MononuclearHydridesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        for (name, structure) in MONONUCLEAR_HYDRIDES {
            let prefix = name.strip_suffix("ane").unwrap();
            dfa.insert(prefix, Token::Structure(structure.as_any()));
        }
    }
}

impl NamedStructure for MononuclearHydride {
    fn to_graph(&self) -> Graph {
        let hydrogen_count = self.0.standard_bonding_number() as usize;
        Graph {
            atoms: [self.0]
                .into_iter()
                .chain((0..hydrogen_count).map(|_| Element::Hydrogen))
                .collect(),
            bonds: (0..hydrogen_count).map(|i| (0, i + 1)).collect(),
            positions: vec![(Locant::Number(1), 0)],
            free_valences: Vec::new(),
        }
    }
}
