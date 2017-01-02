use ast::*;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
enum Token {
	Function,
	LParen, RParen,
	LBrace, RBrace,
	ID(String),
	Number(i32)
}

fn tok(cur: &mut String, peek: bool) -> Result<Token, String> {
	let name_regex: Regex = Regex::new("^[:alnum:]+").unwrap();
	let num_literal_regex: Regex = Regex::new("^[:digit:]+").unwrap();
	if cur.trim().starts_with("fn") {
		if !peek {
			*cur = cur.trim()[2..].to_string();
		}
		Ok(Token::Function)
	} else if cur.trim().starts_with("(") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::LParen)
	} else if cur.trim().starts_with(")") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::RParen)
	} else if cur.trim().starts_with("{") {
		
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}

		Ok(Token::LBrace)
	} else if cur.trim().starts_with("}") {
		
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}

		Ok(Token::RBrace)
	} else if let Some((first, second)) = num_literal_regex.find(cur.trim()) {
		let num = Token::Number(cur.trim()[first..second].parse::<i32>().unwrap()); //TODO: Ignore parsing error potential here, could it ever happen (I dont think so)

		if !peek {
			*cur = cur.trim()[second..].to_string();
		}

		Ok(num)
	} else if let Some((first, second)) = name_regex.find(cur.trim()) {
		let name = Token::ID(cur.trim()[first..second].to_string());

		if !peek {
			*cur = cur.trim()[second..].to_string();
		}

		Ok(name)
	} else {
		Err(("No token at ".to_string() + cur.trim()).to_string())
	}
}

fn parse_expr(cur: &mut String) -> Result<AST, String> {
	let n1 = try!(tok(cur, false));

	if let Token::Number(n) = n1 {
		Ok(AST::lit(n))
	} else {
		Err(("unexpected token near ".to_string() + &cur).to_string())
	}
}

fn parse_fn(cur: &mut String) -> Result<AST, String> {
	let nt = try!(tok(cur, false));

	match nt {
		Token::ID(_) => {},
		_ => return Err(format!("expected ID when {:?}", nt))
	}

	if try!(tok(cur, false)) != Token::LParen {
		return Err("expected LP".to_string());
	}

	if try!(tok(cur, false)) != Token::RParen {
		return Err("expected RP".to_string());
	}

	if try!(tok(cur, false)) != Token::LBrace {
		return Err("expected LB".to_string());
	}

	if try!(tok(cur, true)) == Token::RBrace {
		return Err("Functions cannot be empty".to_string())
	}

	let newFn = try!(parse_expr(cur));

	if try!(tok(cur, false)) != Token::RBrace {
		return Err("expected RB".to_string());
	}

	return Ok(newFn);
}

pub fn parse_top(cur: &mut String) -> Result<AST, String> {
	let first = try!(tok(cur, false));
	if first == Token::Function {
		parse_fn(cur)
	} else {
		Err("First token should be fn".to_string())
	}
}