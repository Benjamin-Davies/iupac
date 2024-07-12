pub mod chapters;
pub mod graph;
pub mod parser;
pub mod plugin;
pub mod scanner;
pub mod test;

pub use chapters::p_1_general::{
    p_11_scope::{Element, ELEMENTS},
    p_14_general_rules::p_14_3_locants::Locant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Hydrogen,
    Oxygen,
    Water,
    Ammonia,
    Isobutane,
    Benzene,
    Pyrimidine,
    Purine,
}

impl Base {
    pub fn has_isomers(&self) -> bool {
        matches!(self, Base::Purine)
    }
}
