use iupac::inchi::InChI;
use paste::paste;

macro_rules! test_inchi {
    ($name:ident($inchi:literal)) => {
        paste! {
            #[test]
            fn [<test_ $name _inchi>]() {
                test_inchi_impl(
                    &iupac::test::[<$name:upper>],
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
}

fn test_inchi_impl(_iupac: &str, inchi: &str) {
    let inchi: InChI = inchi.parse().unwrap();
}
