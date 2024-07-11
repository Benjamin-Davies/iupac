use std::{fs, path::PathBuf, process::Command};

use blue_book::{graph::Graph, parser::parse};
use paste::paste;
use petgraph::{algo::is_isomorphic_matching, graph::UnGraph};
use red_book::elements::Element;

macro_rules! test_graph {
    ($name:ident) => {
        paste! {
            #[test]
            fn [<test_ $name _graph>]() {
                test_graph_impl(
                    &stringify!($name),
                    &blue_book::test::[<$name:upper>],
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
        let expected: UnGraph<Element, ()> = serde_json::from_str(&json).unwrap();

        assert!(is_isomorphic_matching(&ungraph, &expected, eq, eq));
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

fn eq<T: Eq>(a: &T, b: &T) -> bool {
    a.eq(b)
}
