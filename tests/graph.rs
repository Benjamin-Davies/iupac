use std::{fs, path::PathBuf, process::Command};

use iupac::{graph::Graph, parser::parse};

macro_rules! test_graph {
    ($name:ident, $molecule:ident) => {
        #[test]
        fn $name() {
            test_graph_impl(
                &stringify!($molecule).to_lowercase(),
                &iupac::test::$molecule,
            );
        }
    };
}

test_graph!(test_isopropanol_graph, ISOPROPANOL);
test_graph!(test_isobutane_graph, ISOBUTANE);

test_graph!(test_dopamine_graph, DOPAMINE);
test_graph!(test_salbutamol_graph, SALBUTAMOL);
test_graph!(test_caffeine_graph, CAFFEINE);

test_graph!(test_adenine_graph, ADENINE);
test_graph!(test_thymine_graph, THYMINE);
test_graph!(test_cytosine_graph, CYTOSINE);
test_graph!(test_guanine_graph, GUANINE);

fn test_graph_impl(name: &str, iupac_name: &str) {
    let ast = parse(iupac_name);
    let graph = Graph::from(&*ast);

    let path = PathBuf::from(format!("examples/{name}.dot"));
    if path.exists() {
        let expected = fs::read_to_string(&path).unwrap();
        assert_eq!(graph.to_string(), expected);
    } else {
        fs::write(&path, graph.to_string()).unwrap();
        Command::new("neato")
            .arg(&path)
            .arg("-O")
            .arg("-Tpng")
            .status()
            .unwrap();
    }
}
