//! # P-14.3 Locants

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locant {
    Unspecified,
    Number(u8),
    Element(u8, Element),
}
