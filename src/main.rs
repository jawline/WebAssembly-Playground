extern crate regex;

mod ast;
mod writer;
mod parse;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

	let mut f = File::open(env::args().nth(1).unwrap()).unwrap();
	let mut d = String::new();
	f.read_to_string(&mut d).unwrap();

   	match parse::parse_top(&mut d) {
   		Ok(ast) => println!("{}", writer::to_s(&ast)),
   		Err(msg) => println!("Err {}", msg)
   	}
}