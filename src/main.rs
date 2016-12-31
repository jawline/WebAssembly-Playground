mod ast;
mod writer;
mod parse;

use ast::{Constant, AST};

fn main() {
    let a = AST::Function("hi".to_string(), Vec::new(), AST::add(AST::lit(5), AST::lit(4)));

    println!("{}", writer::to_s(&a));
}