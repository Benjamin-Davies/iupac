use parsing::dfa;

use crate::{
    chapters::{
        p_1_general::p_14_general_rules::p_14_2_multiplicative_prefixes,
        p_2_hydrides::{
            p_21_simple_hydrides::{p_21_1_mononuclear_hydrides, p_21_2_acyclic_hydrides},
            p_22_monocyclic_hydrides::{
                p_22_1_monocyclic_hydocarbons, p_22_2_heteromonocyclic_hydrides,
            },
            p_25_fused_ring_systems::p_25_2_heterocyclic_ring_components,
            p_29_hydride_prefixes::p_29_2_general_names,
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
    &p_22_1_monocyclic_hydocarbons::MonocyclicHydrocarbonsPlugin,
    &p_22_2_heteromonocyclic_hydrides::HeteromonocyclicHydridesPlugin,
    &p_25_2_heterocyclic_ring_components::HeterocyclicRingPlugin,
    &p_29_2_general_names::GeneralHydridePrefixesPlugin,
];
