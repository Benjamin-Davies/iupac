//! # P-21.2 Acyclic Polynuclear Parent Hydrides

use parsing::dfa;

use crate::{plugin::Plugin, scanner::Token, Element};

use super::SimpleHydride;

pub struct AcyclicHydridesPlugin;

pub const fn alkane(length: u16) -> SimpleHydride {
    SimpleHydride {
        length,
        element: Element::Carbon,
    }
}

pub const ETHANE: SimpleHydride = alkane(2);
pub const PROPANE: SimpleHydride = alkane(3);
pub const BUTANE: SimpleHydride = alkane(4);

impl Plugin for AcyclicHydridesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("eth", Token::Hydride(ETHANE.into()));
        dfa.insert("prop", Token::Hydride(PROPANE.into()));
        dfa.insert("but", Token::Hydride(BUTANE.into()));
    }
}
