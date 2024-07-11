//! Generates a DOT file from an IUPAC name.
//!
//! # Usage
//!
//! ```sh
//! cargo run --bin dot 'Propan-2-ol'
//! ```

use std::env;

use blue_book::{graph::Graph, parser::parse};

fn main() {
    let name = env::args().nth(1).unwrap();
    let ast = parse(&name);
    let graph = Graph::from(&*ast);
    print!("{graph}");
}
