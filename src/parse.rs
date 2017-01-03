use ast::*;
use regex::Regex;

macro_rules! expect {
    ($thet:expr, $cur:expr) => {
        if try!(tok($cur, false)) != $thet {
			return Err(format!("Expected {:?} near {:?}", $thet, $cur));
		}
    };
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Token {
	Function,
	LParen, RParen,
	LBrace, RBrace,
	Comma, Plus, Minus, Multiply, Divide, Mod,
	ID(String),
	Number(i32)
}

impl Token {
	fn op(&self) -> Option<BinaryOperation> {
		match *self {
			Token::Plus => Some(BinaryOperation::Add),
			Token::Minus => Some(BinaryOperation::Subtract),
			Token::Multiply => Some(BinaryOperation::Multiply),
			Token::Divide => Some(BinaryOperation::Divide),
			Token::Mod => Some(BinaryOperation::Mod),
			_ => None
		}
	}
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
	} else if cur.trim().starts_with("+") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Plus)
	} else if cur.trim().starts_with(",") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Comma)
	} else if cur.trim().starts_with("%") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Mod)
	} else if cur.trim().starts_with("-") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Minus)
	} else if cur.trim().starts_with("*") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Multiply)
	} else if cur.trim().starts_with("/") {
		if !peek {
			*cur = cur.trim()[1..].to_string();
		}
		Ok(Token::Divide)
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

fn parse_atom(cur: &mut String) -> Result<AST, String> {
	let n1 = try!(tok(cur, false));
	if let Token::Number(n) = n1 {
		Ok(AST::lit(n))
	} else {
		Err(("unexpected token near ".to_string() + &cur).to_string())
	}
}

fn parse_expr(cur: &mut String) -> Result<AST, String> {
	let a1 = try!(parse_atom(cur));
	let peek = try!(tok(cur, true));

	match peek {
		Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Mod => {
			//Discard the peeked token
			try!(tok(cur, false));
			let e2 = try!(parse_expr(cur));
			Ok(AST::BinaryOp(peek.op().unwrap(), Box::new(a1), Box::new(e2)))
		},
		_ => Ok(a1)
	}
}

fn parse_args(cur: &mut String) -> Result<Vec<String>, String> {
	let mut args = Vec::new();

	if try!(tok(cur, true)) == Token::RParen {
		Ok(args)
	} else {
		loop {

			match try!(tok(cur, false)) {
				Token::ID(n) => { args.push(n); },
				_ => { return Err("oh no".to_string()); }
			}

			if try!(tok(cur, true)) == Token::Comma {
				try!(tok(cur, false));
			} else if try!(tok(cur, true)) == Token::RParen {
				try!(tok(cur, false));
				break;
			} else {
				return Err("bugger expected Comma or RParen".to_string());
			}
		}

		Ok(args)
	}
}

fn parse_fn(cur: &mut String) -> Result<AST, String> {
	let nt = try!(tok(cur, false));
	let name;

	match nt {
		Token::ID(x) => { name = x; },
		_ => return Err(format!("expected ID when {:?}", nt))
	}

	expect!(Token::LParen, cur);

	let args = try!(parse_args(cur));

	expect!(Token::LBrace, cur);

	let new_fn = AST::Function(name, args, Box::new(try!(parse_expr(cur))));

	expect!(Token::RBrace, cur);

	return Ok(new_fn);
}

pub fn parse_top(cur: &mut String) -> Result<AST, String> {
	let first = try!(tok(cur, false));
	if first == Token::Function {
		parse_fn(cur)
	} else {
		Err("First token should be fn".to_string())
	}
}