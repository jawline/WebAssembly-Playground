extern crate regex;

#[macro_use] mod warn;
mod ast;
mod parse;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

	if env::args().len() != 2 {
		println!("Invalid usage");
		std::process::exit(1);
	} else {
		let mut f = File::open(env::args().nth(1).unwrap()).unwrap();
		let mut d = String::new();
		f.read_to_string(&mut d).unwrap();

	   	match parse::parse_top(&mut d) {
	   		Ok(ast) => println!("{}", ast.as_s()),
	   		Err(msg) => println!("Err {}", msg)
	   	};

	   	std::process::exit(0);
   }
}