lalrpop_mod!(let_grammar, "/LET/let_grammar.rs"); // synthesized by LALRPOP
use LET::ast::Exp;

pub fn parse(s: &str) -> Option<Exp> {
  let_grammar::ProgramParser::new().parse(s).ok()
}

