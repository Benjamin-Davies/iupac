use parsing::dfa;

use crate::{
    chapters::p_1_general::p_14_general_rules::p_14_2_multiplicative_prefixes::MultiplicativePrefixesPlugin,
    scanner::Token,
};

pub trait Plugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>);
}

pub const PLUGINS: &[&dyn Plugin] = &[&MultiplicativePrefixesPlugin];
