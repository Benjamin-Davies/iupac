use std::env;

use iupac::{graph::Graph, parser::parse};

fn main() {
    let name = env::args().nth(1).unwrap();
    let ast = parse(&name);
    let graph = Graph::from(&*ast);
    print!("{graph}");
}
