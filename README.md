# IUPAC Name Parser

Parses IUPAC names and generates diagrams of the molecules. So far this only supports a subset of IUPAC names, ignores double bonds and hides all the Hydrogen atoms.

## Usage

To generate diagrams using the built-in gradient-descent, run:

```sh
cargo run --bin structural_formula '1,3,7-Trimethyl-3,7-dihydro-1H-purine-2,6-dione'
```

To generate diagrams using Graphviz, run:

```sh
cargo run --bin dot 'Propan-2-ol' > propanol.dot
neato propanol.dot -Tpng -O
```

See examples folder for example graphviz output.
