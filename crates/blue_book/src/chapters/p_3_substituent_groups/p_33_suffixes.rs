//! # P-35 Prefixes Corresponding to Characteristic Groups

use parsing::dfa;

use crate::{plugin::Plugin, scanner::Token};

use super::CharacteristicGroup;

pub struct SuffixesPlugin;

impl Plugin for SuffixesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("one", Token::Suffix(CharacteristicGroup::Oxo));
        dfa.insert("ol", Token::Suffix(CharacteristicGroup::Hydroxy));
        dfa.insert("amine", Token::Suffix(CharacteristicGroup::Amino));
    }
}
