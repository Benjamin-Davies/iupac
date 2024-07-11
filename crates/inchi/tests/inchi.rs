use blue_book::{graph::Graph, parser::parse};
use inchi::InChI;
use paste::paste;
use petgraph::{algo::is_isomorphic_matching, graph::UnGraph};

macro_rules! test_inchi {
    ($name:ident($inchi:literal)) => {
        paste! {
            #[test]
            fn [<test_ $name _inchi>]() {
                test_inchi_impl(
                    &blue_book::test::[<$name:upper>],
                    $inchi,
                );
            }
        }
    };
    ($($name:ident($inchi:literal),)*) => {
        $(test_inchi!($name($inchi));)*
    };
}

test_inchi! {
    isopropanol("InChI=1S/C3H8O/c1-3(2)4/h3-4H,1-2H3"),
    isobutane("InChI=1S/C4H10/c1-4(2)3/h4H,1-3H3"),
    //
    dopamine("InChI=1S/C8H11NO2/c9-4-3-6-1-2-7(10)8(11)5-6/h1-2,5,10-11H,3-4,9H2"),
    salbutamol("InChI=1S/C13H21NO3/c1-13(2,3)14-7-12(17)9-4-5-11(16)10(6-9)8-15/h4-6,12,14-17H,7-8H2,1-3H3"),
    caffeine("InChI=1S/C8H10N4O2/c1-10-4-9-6-5(10)7(13)12(3)8(14)11(6)2/h4H,1-3H3"),
    //
    adenine("InChI=1S/C5H5N5/c6-4-3-5(9-1-7-3)10-2-8-4/h1-2H,(H3,6,7,8,9,10)"),
    thymine("InChI=1S/C5H6N2O2/c1-3-2-6-5(9)7-4(3)8/h2H,1H3,(H2,6,7,8,9)"),
    cytosine("InChI=1S/C4H5N3O/c5-3-1-2-6-4(8)7-3/h1-2H,(H3,5,6,7,8)"),
    guanine("InChI=1S/C5H5N5O/c6-5-9-3-2(4(11)10-5)7-1-8-3/h1H,(H4,6,7,8,9,10,11)"),
}

fn test_inchi_impl(iupac: &str, inchi: &str) {
    let iupac = parse(iupac);
    let iupac_graph = Graph::from(&*iupac);
    let iupac_graph = UnGraph::from(&iupac_graph);

    let inchi: InChI = inchi.parse().unwrap();
    let isomers = inchi.isomers();
    assert!(!isomers.is_empty());

    let any_match = isomers.iter().any(|isomer| {
        let inchi_graph = UnGraph::from(isomer);
        is_isomorphic_matching(&iupac_graph, &inchi_graph, eq, eq)
    });
    assert!(any_match);
}

fn eq<T: Eq>(a: &T, b: &T) -> bool {
    a.eq(b)
}
