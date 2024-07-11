//! # P-14.1 Bonding Number

use crate::Element;

impl Element {
    pub fn standard_bonding_number(self) -> u8 {
        match self.group() {
            1 => 1,
            13 => 3,
            14 => 4,
            15 => 3,
            16 => 2,
            17 => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Element;

    #[test]
    fn test_standard_bonding_number() {
        assert_eq!(Element::Hydrogen.standard_bonding_number(), 1);

        assert_eq!(Element::Boron.standard_bonding_number(), 3);
        assert_eq!(Element::Carbon.standard_bonding_number(), 4);
        assert_eq!(Element::Nitrogen.standard_bonding_number(), 3);
        assert_eq!(Element::Oxygen.standard_bonding_number(), 2);
        assert_eq!(Element::Fluorine.standard_bonding_number(), 1);
    }
}
