extern crate regex;

mod ast;
mod writer;
mod parse;

fn main() {

	let mut test = "fn hi() { 5 + 4 - 3 + 2 + 1 }".to_string();

   	match parse::parse_top(&mut test) {
   		Ok(ast) => println!("{}", writer::to_s(&ast)),
   		Err(msg) => println!("Err {}", msg)
   	}
}