use std::fs;

use blue_book::parser::parse;
use paste::paste;
use structural_formula::structure::ToStructure;

macro_rules! test_svg {
    ($name:ident) => {
        paste! {
            #[test]
            fn [<test_ $name _graph>]() {
                test_svg_impl(
                    &stringify!($name),
                    &blue_book::test::[<$name:upper>],
                );
            }
        }
    };
    ($($name:ident,)*) => {
        $(test_svg!($name);)*
    };
}

test_svg!(
    isopropanol,
    // isobutane,
    //
    // dopamine,
    // salbutamol,
    // caffeine,
    //
    // adenine,
    // thymine,
    // cytosine,
    // guanine,
);

fn test_svg_impl(name: &str, iupac_name: &str) {
    let ast = parse(iupac_name);
    let structure = ast.to_structure();

    let path = format!("examples/{name}.svg");
    let contents = structure.svg().to_string();
    if fs::read_to_string(&path).unwrap() != contents {
        fs::write(&path, contents).unwrap();
    }
}
