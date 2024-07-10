use std::{fs, path::PathBuf, process::Command};

use iupac::{graph::Graph, parser::parse};
use paste::paste;
use petgraph::{algo::is_isomorphic, graph::UnGraph};

macro_rules! test_graph {
    ($name:ident) => {
        paste! {
            #[test]
            fn [<test_ $name _graph>]() {
                test_graph_impl(
                    &stringify!($name),
                    &iupac::test::[<$name:upper>],
                );
            }
        }
    };
    ($($name:ident,)*) => {
        $(test_graph!($name);)*
    };
}

test_graph!(
    isopropanol,
    isobutane,
    //
    dopamine,
    salbutamol,
    caffeine,
    //
    adenine,
    thymine,
    cytosine,
    guanine,
);

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
