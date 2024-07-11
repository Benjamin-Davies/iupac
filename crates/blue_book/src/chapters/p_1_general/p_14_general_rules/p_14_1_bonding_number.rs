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
