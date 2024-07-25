use std::fmt;

use blue_book::Element;
use petgraph::visit::EdgeRef;

use crate::structure::{Atom, Bond, Structure};

const SCALE: f32 = 30.0;
const PADDING: f32 = 15.0;
const FONT_SIZE: f32 = 16.0;
const TEXT_EXCLUSION_RADIUS: f32 = 6.0;

pub struct SVG<'a> {
    structure: &'a Structure,
}

impl Structure {
    pub fn svg(&self) -> SVG {
        SVG { structure: self }
    }
}

impl fmt::Display for SVG<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bounds = self.structure.bounds();
        writeln!(
            f,
            "<svg viewBox='{x} {y} {w} {h}' xmlns='http://www.w3.org/2000/svg'>",
            x = SCALE * bounds.min.x - PADDING,
            y = SCALE * bounds.min.y - PADDING,
            w = SCALE * bounds.width() + 2.0 * PADDING,
            h = SCALE * bounds.height() + 2.0 * PADDING,
        )?;

        write_styles(f)?;

        for atom in self.structure.graph.node_indices() {
            let atom = &self.structure.graph[atom];
            write_atom(f, atom)?;
        }

        for bond in self.structure.graph.edge_references() {
            let a = &self.structure.graph[bond.source()];
            let b = &self.structure.graph[bond.target()];
            let bond = bond.weight();
            write_bond(f, a, b, bond)?;
        }

        writeln!(f, "</svg>")?;

        Ok(())
    }
}

fn write_styles(f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(
        f,
        "<style>
            text {{
                font-family: serif;
                font-size: {FONT_SIZE}px;
            }}
        </style>",
    )
}

fn write_atom(f: &mut fmt::Formatter, atom: &Atom) -> fmt::Result {
    if atom.element == Element::Carbon {
        writeln!(
            f,
            "<circle cx='{x}' cy='{y}' r='0.5' fill='black' />",
            x = SCALE * atom.position.x,
            y = SCALE * atom.position.y,
        )?;
    } else {
        writeln!(
            f,
            "<text x='{x}' y='{y}' text-anchor='middle' dominant-baseline='middle'>{symbol}</text>",
            x = SCALE * atom.position.x,
            y = SCALE * atom.position.y,
            symbol = atom.element.symbol(),
        )?;
    }

    Ok(())
}

fn write_bond(f: &mut fmt::Formatter, a: &Atom, b: &Atom, _bond: &Bond) -> fmt::Result {
    let mut a_position = SCALE * a.position;
    let mut b_position = SCALE * b.position;
    let direction = (b_position - a_position).normalize();

    if a.element != Element::Carbon {
        a_position += direction * TEXT_EXCLUSION_RADIUS;
    }
    if b.element != Element::Carbon {
        b_position -= direction * TEXT_EXCLUSION_RADIUS;
    }

    writeln!(
        f,
        "<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='black' />",
        x1 = a_position.x,
        y1 = a_position.y,
        x2 = b_position.x,
        y2 = b_position.y,
    )?;

    Ok(())
}
