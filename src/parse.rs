use ast::*;
use regex::Regex;

macro_rules! expect {
    ($thet:expr, $cur:expr) => {
        if try!(tok($cur, false)) != $thet {
			return Err(format!("Expected {:?} near {:?}", $thet, $cur));
		}
    };
}

type Arg = String;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Token {
	Function,
	LParen, RParen,
	LBrace, RBrace, Plus, Minus, Multiply, Divide, Mod,
	ID(String),
	Number(i32)
}

impl Token {

	fn is_tok(c: char) -> Option<Token> {
		match c {
			'(' => Some(Token::LParen),
			')' => Some(Token::RParen),
			'{' => Some(Token::LBrace),
			'}' => Some(Token::RBrace),
			'+' => Some(Token::Plus),
			'-' => Some(Token::Minus),
			'*' => Some(Token::Multiply),
			'/' => Some(Token::Divide),
			'%' => Some(Token::Mod),
			_ => { None }
		}
	}

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

	//If its a single character token match in O(1)
	let (tok, size) = if let Some(t) = Token::is_tok(cur.trim().chars().next().unwrap_or('\0')) {
		(Ok(t), 1)
	} else {
		//Section matches multi-character tokens
		let cur = cur.trim(); //Block scope rename cur to trimmed cur
		if cur.starts_with("fn") {
			(Ok(Token::Function), 2)
		} else if let Some((first, second)) = num_literal_regex.find(cur) {
			(Ok(Token::Number(cur[first..second].parse::<i32>().unwrap())), second)
		} else if let Some((first, second)) = name_regex.find(cur) {
			(Ok(Token::ID(cur[first..second].to_string())), second)
		} else {
			(Err(("No token at ".to_string() + cur).to_string()), 0)
		}
	};

	if !peek {
		*cur = cur.trim()[size..].to_string();
	}

	tok
}

fn parse_atom(cur: &mut String, args: &Vec<Arg>) -> Result<AST, String> {
	let atom_tok = try!(tok(cur, false));
	if let Token::Number(n) = atom_tok {
		Ok(AST::lit(n))
	} else if let Token::ID(s) = atom_tok {
		match args.iter().enumerate().find(|&r| r.1 == &s) {
			Some((size, _)) => Ok(AST::Local(size)),
			None => Err(format!("No variable named {}", s))
		}

	} else {
		Err(("unexpected token near ".to_string() + &cur).to_string())
	}
}

fn parse_expr(cur: &mut String, args: &Vec<String>) -> Result<AST, String> {
	let a1 = try!(parse_atom(cur, args));
	let peek = try!(tok(cur, true));

	match peek {
		Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Mod => {
			//Discard the peeked token
			try!(tok(cur, false));
			Ok(AST::BinaryOp(peek.op().unwrap(), Box::new(a1), Box::new(try!(parse_expr(cur, args)))))
		},
		_ => Ok(a1)
	}
}

fn parse_arg(cur: &mut String) -> Result<String, String> {

	let name;

	match try!(tok(cur, false)) {
		Token::ID(n) => { name = n; },
		_ => { return Err("oh no".to_string()); }
	}

	Ok(name)
}

fn parse_args(cur: &mut String) -> Result<Vec<String>, String> {
	let mut args = Vec::new();

	loop {
		if try!(tok(cur, true)) == Token::RParen {
			try!(tok(cur, false));
			break;
		} else {
			args.push(try!(parse_arg(cur)));
		}
	}

	Ok(args)
}

fn parse_fn(cur: &mut String) -> Result<AST, String> {
	let name;

	match try!(tok(cur, false)) {
		Token::ID(x) => { name = x; },
		_ => return Err(format!("expected ID near {:?}", cur))
	}

	expect!(Token::LParen, cur);

	let args = try!(parse_args(cur));

	expect!(Token::LBrace, cur);

	let parsed_expr = try!(parse_expr(cur, &args));

	let new_fn = AST::Function(name, args, Box::new(parsed_expr));

	expect!(Token::RBrace, cur);

	return Ok(new_fn);
}

pub fn parse_top(cur: &mut String) -> Result<AST, String> {
	expect!(Token::Function, cur);
	parse_fn(cur)
}