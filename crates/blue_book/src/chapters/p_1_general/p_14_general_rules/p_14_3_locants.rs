//! # P-14.3 Locants

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locant {
    Unspecified,
    Number(u16),
    Element(u16, Element),
}
