use parsing::dfa;

use crate::{
    chapters::{
        p_1_general::p_14_general_rules::p_14_2_multiplicative_prefixes,
        p_2_hydrides::p_21_simple_hydrides::{
            p_21_1_mononuclear_hydrides, p_21_2_acyclic_hydrides,
        },
    },
    scanner::Token,
};

pub trait Plugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>);
}

pub const PLUGINS: &[&dyn Plugin] = &[
    &p_14_2_multiplicative_prefixes::MultiplicativePrefixesPlugin,
    &p_21_1_mononuclear_hydrides::MononuclearHydridesPlugin,
    &p_21_2_acyclic_hydrides::AcyclicHydridesPlugin,
];
