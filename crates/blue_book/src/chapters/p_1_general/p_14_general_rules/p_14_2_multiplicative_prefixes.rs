//! # P-14.2 Multiplicative Prefixes

use crate::{parser, plugin::Plugin, scanner::Token};

pub struct MultiplicativePrefixesPlugin;

impl Plugin for MultiplicativePrefixesPlugin {
    fn init_tokens(&self, dfa: &mut parsing::dfa::Automaton<crate::scanner::Token>) {
        dfa.insert("mono", Token::Multiplicity(1));
        dfa.insert("hen", Token::Multiplicity(1));
        dfa.insert("di", Token::Multiplicity(2));
        dfa.insert("do", Token::Multiplicity(2));
        dfa.insert("tri", Token::Multiplicity(3));
        dfa.insert("tetr", Token::Multiplicity(4));
        dfa.insert("pent", Token::Multiplicity(5));
        dfa.insert("hex", Token::Multiplicity(6));
        dfa.insert("hept", Token::Multiplicity(7));
        dfa.insert("oct", Token::Multiplicity(8));
        dfa.insert("non", Token::Multiplicity(9));
        dfa.insert("dec", Token::Multiplicity(10));

        dfa.insert("undec", Token::Multiplicity(11));
        dfa.insert("icos", Token::Multiplicity(20));
        dfa.insert("cos", Token::Multiplicity(20));
        dfa.insert("triacont", Token::Multiplicity(30));
        dfa.insert("tetracont", Token::Multiplicity(40));
        dfa.insert("pentacont", Token::Multiplicity(50));
        dfa.insert("hexacont", Token::Multiplicity(60));
        dfa.insert("heptacont", Token::Multiplicity(70));
        dfa.insert("octacont", Token::Multiplicity(80));
        dfa.insert("nonacont", Token::Multiplicity(90));
        dfa.insert("hect", Token::Multiplicity(100));

        dfa.insert("henhect", Token::Multiplicity(101));
        dfa.insert("dict", Token::Multiplicity(200));
        dfa.insert("trict", Token::Multiplicity(300));
        dfa.insert("tetract", Token::Multiplicity(400));
        dfa.insert("pentact", Token::Multiplicity(500));
        dfa.insert("hexact", Token::Multiplicity(600));
        dfa.insert("heptact", Token::Multiplicity(700));
        dfa.insert("octact", Token::Multiplicity(800));
        dfa.insert("nonact", Token::Multiplicity(900));
        dfa.insert("kili", Token::Multiplicity(1000));

        dfa.insert("henkili", Token::Multiplicity(1001));
        dfa.insert("dili", Token::Multiplicity(2000));
        dfa.insert("trili", Token::Multiplicity(3000));
        dfa.insert("tetrali", Token::Multiplicity(4000));
        dfa.insert("pentali", Token::Multiplicity(5000));
        dfa.insert("hexali", Token::Multiplicity(6000));
        dfa.insert("heptali", Token::Multiplicity(7000));
        dfa.insert("octali", Token::Multiplicity(8000));
        dfa.insert("nonali", Token::Multiplicity(9000));
    }
}

impl parser::State {
    pub fn pop_multiplicity(&mut self) -> u16 {
        if !matches!(self.stack.last(), Some(parser::StackItem::Multiplicity(_))) {
            return 1;
        }

        let mut n = 0;
        while let Some(&parser::StackItem::Multiplicity(m)) = self.stack.last() {
            n += m;
            self.stack.pop();
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chapters::p_2_hydrides::p_21_simple_hydrides::p_21_2_acyclic_hydrides::alkane,
        parser::{parse, AST},
    };

    #[test]
    fn test_parse_composite_prefix() {
        for (n, prefix) in [
            (14, "tetradeca"),
            (21, "henicosa"),
            (22, "docosa"),
            (23, "tricosa"),
            (24, "tetracosa"),
            (41, "hentetraconta"),
            (52, "dopentaconta"),
            (111, "undecahecta"),
            (363, "trihexacontatricta"),
            (486, "hexaoctacontatetracta"),
        ] {
            let name = prefix.trim_end_matches('a').to_owned() + "ane";
            let ast = parse(&name);
            assert_eq!(*ast, AST::Hydride(alkane(n).into()));
        }
    }
}
