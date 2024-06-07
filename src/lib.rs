pub mod dfa;
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
pub enum Element {
    Hydrogen,

    // P-11 SCOPE OF NOMENCLATURE FOR ORGANIC COMPOUNDS
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,

    Aluminum,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,

    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,

    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,

    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
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
