pub mod chapters;
pub mod graph;
pub mod parser;
pub mod scanner;
pub mod test;

pub use chapters::p_1_general_principles::p_11_scope::{Element, ELEMENTS};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Unspecified,
    Number(u8),
    Element(u8, Element),
}

impl Base {
    pub fn has_isomers(&self) -> bool {
        matches!(self, Base::Purine)
    }
}
