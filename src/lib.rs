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
