use std::{fs, path::PathBuf, process::Command};

use iupac::{graph::Graph, parser::parse};
use petgraph::{algo::is_isomorphic, graph::UnGraph};

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
    let ungraph = UnGraph::from(&graph);

    let json_path = PathBuf::from(format!("examples/{name}.json"));
    let dot_path = PathBuf::from(format!("examples/{name}.dot"));
    if json_path.exists() {
        let json = fs::read_to_string(&json_path).unwrap();
        let expected: UnGraph<&str, &str> = serde_json::from_str(&json).unwrap();

        assert!(is_isomorphic(&ungraph, &expected));
    } else {
        let json = serde_json::to_string(&ungraph).unwrap();
        fs::write(&json_path, json).unwrap();

        fs::write(&dot_path, graph.to_string()).unwrap();
        Command::new("neato")
            .arg(&dot_path)
            .arg("-O")
            .arg("-Tpng")
            .status()
            .unwrap();
    }
}
