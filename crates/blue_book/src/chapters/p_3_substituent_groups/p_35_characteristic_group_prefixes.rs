//! # P-35 Prefixes Corresponding to Characteristic Groups

use parsing::dfa;

use crate::{plugin::Plugin, scanner::Token};

use super::CharacteristicGroup;

pub struct CharacteristicGroupPrefixesPlugin;

impl Plugin for CharacteristicGroupPrefixesPlugin {
    fn init_tokens(&self, dfa: &mut dfa::Automaton<Token>) {
        dfa.insert("hydr", Token::Prefix(CharacteristicGroup::Hydro));
        dfa.insert("oxy", Token::Prefix(CharacteristicGroup::Hydroxy));
        dfa.insert("hydroxy", Token::Prefix(CharacteristicGroup::Hydroxy));
        dfa.insert("amino", Token::Prefix(CharacteristicGroup::Amino));
    }
}
