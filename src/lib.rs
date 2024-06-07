pub mod dfa;
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

impl Element {
    fn symbol(&self) -> &'static str {
        match self {
            Element::Hydrogen => "H",
            Element::Boron => "B",
            Element::Carbon => "C",
            Element::Nitrogen => "N",
            Element::Oxygen => "O",
            Element::Fluorine => "F",
            Element::Aluminum => "Al",
            Element::Silicon => "Si",
            Element::Phosphorus => "P",
            Element::Sulfur => "S",
            Element::Chlorine => "Cl",
            Element::Gallium => "Ga",
            Element::Germanium => "Ge",
            Element::Arsenic => "As",
            Element::Selenium => "Se",
            Element::Bromine => "Br",
            Element::Indium => "In",
            Element::Tin => "Sn",
            Element::Antimony => "Sb",
            Element::Tellurium => "Te",
            Element::Iodine => "I",
            Element::Thallium => "Tl",
            Element::Lead => "Pb",
            Element::Bismuth => "Bi",
            Element::Polonium => "Po",
            Element::Astatine => "At",
        }
    }
}
