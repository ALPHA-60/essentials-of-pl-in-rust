lalrpop_mod!(proc_grammar, "/PROC/proc_grammar.rs"); // synthesized by LALRPOP
use PROC::ast::Exp;

pub fn parse(s: &str) -> Option<Exp> {
  proc_grammar::ProgramParser::new().parse(s).ok()
}

