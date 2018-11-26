#![feature(box_syntax, box_patterns)]
#[macro_use] extern crate lalrpop_util;

#[allow(non_snake_case)]
mod LET { pub mod ast; pub mod eval; pub mod parser; pub mod env; }

#[allow(non_snake_case)]
mod PROC { pub mod ast; pub mod eval; pub mod parser; pub mod env; }

#[allow(non_snake_case)]
mod LETREC { pub mod ast; pub mod eval; pub mod parser; pub mod env; }

fn main() {
 println!("hi");
}
