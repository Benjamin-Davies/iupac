//! # P-29.2 General Methodology for Naming Substituent Groups

use parsing::dfa;

use crate::{plugin::Plugin, scanner::Token};

pub struct GeneralHydridePrefixesPlugin;

impl Plugin for GeneralHydridePrefixesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("yl", Token::FreeValence);
    }
}
