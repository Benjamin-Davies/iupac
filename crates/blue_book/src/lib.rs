use red_book::elements::Element;

pub mod graph;
pub mod parser;
pub mod scanner;
pub mod test;

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
