# IUPAC Name Parser

Parses IUPAC names and generates diagrams of the molecules. So far this only supports a subset of IUPAC names, ignores double bonds and hides all the Hydrogen atoms.

![Caffeine drawn using custom code](examples/Screenshot%20from%202024-07-02%2017-07-02.png)

## Usage

To generate diagrams using the built-in gradient-descent layout engine, run:

```sh
cargo run -p gradient_descent '1,3,7-Trimethyl-3,7-dihydro-1H-purine-2,6-dione'
```

To generate diagrams using Graphviz, run:

```sh
cargo run --bin dot 'Propan-2-ol' > propanol.dot
neato propanol.dot -Tpng -O
```

See examples folder for example graphviz output.
