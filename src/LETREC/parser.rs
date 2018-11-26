lalrpop_mod!(letrec_grammar, "/LETREC/letrec_grammar.rs"); // synthesized by LALRPOP
use LETREC::ast::Exp;

pub fn parse(s: &str) -> Option<Exp> {
  letrec_grammar::ProgramParser::new().parse(s).ok()
}

