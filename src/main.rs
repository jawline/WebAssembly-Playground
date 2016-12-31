mod ast;
mod writer;
mod parse;

use ast::{Constant, AST};

fn main() {
    let a = AST::Function("hi".to_string(), Vec::new(), "i32".to_string(), Box::new(AST::Literal(Constant::Int32(10))));

    println!("{}", writer::to_s(&a));
}