extern crate regex;

mod ast;
mod writer;
mod parse;

fn main() {

	let mut test = "fn hi() { 5 }".to_string();

   	match parse::parseTop(&mut test) {
   		Ok(ast) => println!("{}", writer::to_s(&ast)),
   		Err(msg) => println!("Err {}", msg)
   	}
}