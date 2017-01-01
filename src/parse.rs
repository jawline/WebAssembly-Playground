use ast::*;
use regex::Regex;

#[derive(PartialEq, Eq)]
enum Token {
	Function,
	LParen, RParen,
	LBrace, RBrace,
	ID(String)
}

fn tok(cur: &mut String) -> Result<Token, String> {
	let name_regex: Regex = Regex::new("[:alnum:]+").unwrap();
	if cur.trim().starts_with("fn") {
		*cur = cur.trim()[2..].to_string();
		Ok(Token::Function)
	} else if cur.trim().starts_with("(") {
		Ok(Token::LParen)
	} else if cur.trim().starts_with(")") {
		Ok(Token::RParen)
	} else if cur.trim().starts_with("{") {
		Ok(Token::LBrace)
	} else if cur.trim().starts_with("}") {
		Ok(Token::RBrace)
	} else if let Some((first, second)) = name_regex.find(cur.trim()) {
		let name = Token::ID(cur[first..second].to_string());
		*cur = cur.trim()[second..].to_string();
		Ok(name)
	} else {
		Err(("No token at ".to_string() + cur.trim()).to_string())
	}
}

fn parseFn(cur: &mut String) -> Result<AST, String> {
	let nt = try!(tok(cur));
	let lp = try!(tok(cur));
	let rp = try!(tok(cur));
}

pub fn parseTop(cur: &mut String) -> Result<AST, String> {
	let first = try!(tok(cur));
	if first == Token::Function {
		parseFn(cur)
	} else {
		Err("First token should be fn".to_string())
	}
}