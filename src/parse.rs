use ast::*;
use regex::Regex;

macro_rules! expect {
    ($thet:expr, $cur:expr) => {
        if try!(tok($cur, false)) != $thet {
			return Err(format!("Expected {:?} near {:?}", $thet, $cur));
		}
    };
}

macro_rules! next {
    ($cur:expr) => {
        try!(tok($cur, false))
    };
}

macro_rules! push {
    ($a:expr, $b:expr) => {{
    	let mut t = $a; t.push($b); t
    }};
}

macro_rules! peek {
	($cur:expr) => {
		try!(tok($cur, true))
    };
	($thet:expr, $cur:expr) => {
		if let Ok(n) = tok($cur, true) {
			if n == $thet {
				true
			} else {
				false
			}
		} else {
			false
		}
    };
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Token {
	Function, If, Then, Else,
	LParen, RParen, Comma,
	LBrace, RBrace, Plus, Minus, Equals, Multiply, Divide, Mod, GreaterThan, LessThan,
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
			'=' => Some(Token::Equals),
			',' => Some(Token::Comma),
			'+' => Some(Token::Plus),
			'-' => Some(Token::Minus),
			'*' => Some(Token::Multiply),
			'/' => Some(Token::Divide),
			'>' => Some(Token::GreaterThan),
			'<' => Some(Token::LessThan),
			'%' => Some(Token::Mod),
			_ => { None }
		}
	}

	fn is_word_tok(cur: &str) -> Option<(Token, usize)> {
		if cur.starts_with("fn") {
			Some((Token::Function, 2))
		} else if cur.starts_with("if") {
			Some((Token::If, 2))
		} else if cur.starts_with("then") {
			Some((Token::Then, 4))
		} else if cur.starts_with("else") {
			Some((Token::Else, 4))
		} else {
			None
		}
	}

	fn is_regex_tok(cur: &str) -> Option<(Token, usize)> {
		let name_regex: Regex = Regex::new("^([:alnum:]|_)+").unwrap();
		let num_literal_regex: Regex = Regex::new("^[:digit:]+").unwrap();

		if let Some((first, second)) = num_literal_regex.find(cur) {
			Some((Token::Number(cur[first..second].parse::<i32>().unwrap()), second))
		} else if let Some((first, second)) = name_regex.find(cur) {
			Some((Token::ID(cur[first..second].to_string()), second))
		} else {
			None
		}
	}

	fn op(&self) -> Option<BinaryOperation> {
		match *self {
			Token::Plus => Some(BinaryOperation::Add),
			Token::Minus => Some(BinaryOperation::Subtract),
			Token::Multiply => Some(BinaryOperation::Multiply),
			Token::Divide => Some(BinaryOperation::Divide),
			Token::Mod => Some(BinaryOperation::Mod),
			Token::Equals => Some(BinaryOperation::Equals),
			Token::GreaterThan => Some(BinaryOperation::GreaterThan),
			Token::LessThan => Some(BinaryOperation::LessThan),
			_ => None
		}
	}
}

fn tok(cur: &mut String, peek: bool) -> Result<Token, String> {

	//If its a single character token match in O(1)
	let (tok, size) = if let Some(t) = Token::is_tok(cur.trim().chars().next().unwrap_or('\0')) {
		(Ok(t), 1)
	} else {
		//Section matches multi-character tokens
		let cur = cur.trim(); //Block scope rename cur to trimmed cur

		if let Some((token, size)) = Token::is_word_tok(cur) {
			(Ok(token), size)
		} else if let Some((token, size)) = Token::is_regex_tok(cur) {
			(Ok(token), size)
		} else {
			(Err(("No token at ".to_string() + cur).to_string()), 0)
		}
	};

	if !peek {
		*cur = cur.trim()[size..].to_string();
	}

	tok
}

fn parse_fn_args(cur: &mut String, args: &Args) -> Result<Vec<AST>, String> {
	let mut res = Vec::new();

	//TODO: Find a concise way of disallowing ,)
	while !peek!(Token::RParen, cur) {
		res.push(try!(parse_expr(cur, args)));
		if !(peek!(Token::Comma, cur) || peek!(Token::RParen, cur)) {
			return Err(format!("unexpected toke near {}", cur));
		}
	}

	expect!(Token::RParen, cur);
	Ok(res)
}

fn parse_atom(cur: &mut String, args: &Args) -> Result<AST, String> {
	let atom_tok = next!(cur);
	if let Token::Number(n) = atom_tok {
		Ok(AST::lit(n))
	} else if let Token::ID(s) = atom_tok {
		if peek!(Token::LParen, cur) { //Peek => Function call
			expect!(Token::LParen, cur);
			let args = try!(parse_fn_args(cur, args));
			Ok(AST::Call(s, args))
		} else {
			match args.iter().enumerate().find(|&r| (r.1).0 == s) {
				Some((size, item)) => Ok(AST::Local(size, item.clone())),
				None => Err(format!("No variable named {}", s))
			}
		}
	} else {
		Err(("unexpected token near ".to_string() + &cur).to_string())
	}
}

fn parse_maybe_arith(cur: &mut String, args: &Args) -> Result<AST, String> {
	let a1 = try!(parse_atom(cur, args));

	//If the next token is an operation character then do Atom op ParseExpr(cur)
	if peek!(cur).op().is_some() {
		Ok(AST::BinaryOp(next!(cur).op().unwrap(), Box::new(a1), Box::new(try!(parse_expr(cur, args)))))
	} else {
		Ok(a1)
	}
}

fn parse_maybe_if(cur: &mut String, args: &Args) -> Result<AST, String> {

	//if peek and If then expect If cnd Then truepath Else falsepath. Else parse arith
	if peek!(Token::If, cur) {
		expect!(Token::If, cur); //Discard If
		let cnd = try!(parse_expr(cur, args));
		expect!(Token::Then, cur);
		let true_path = try!(parse_expr(cur, args));
		expect!(Token::Else, cur);
		let false_path = try!(parse_expr(cur, args));
		Ok(AST::If(Box::new(cnd), Box::new(true_path), Box::new(false_path)))
	} else {
		parse_maybe_arith(cur, args)
	}
}

fn parse_expr(cur: &mut String, args: &Args) -> Result<AST, String> {
	if peek!(Token::LParen, cur) {
		expect!(Token::LParen, cur);
		let r = parse_expr(cur, args);
		expect!(Token::RParen, cur);
		r
	} else {
		parse_maybe_if(cur, args)
	}
}

fn parse_arg(cur: &mut String) -> Result<String, String> {

	let name;

	match next!(cur) {
		Token::ID(n) => { name = n; },
		_ => { return Err("oh no".to_string()); }
	}

	Ok(name)
}

fn parse_args(cur: &mut String) -> Result<Args, String> {
	let mut args = Args::new();

	while !peek!(Token::RParen, cur) {
		args.push((try!(parse_arg(cur)), Type::Int32));
		if !(peek!(Token::RParen, cur) || peek!(Token::Comma, cur)) {
			return Err(format!("Unexpected token near {}", cur));
		}
	}

	expect!(Token::RParen, cur);

	Ok(args)
}

fn parse_fn(cur: &mut String) -> Result<AST, String> {
	let name;

	match next!(cur) {
		Token::ID(x) => { name = x; },
		_ => return Err(format!("expected ID near {:?}", cur))
	}

	expect!(Token::LParen, cur);

	let args = try!(parse_args(cur));

	expect!(Token::LBrace, cur);

	let parsed_expr = try!(parse_expr(cur, &args));
	let new_fn = AST::Function(name, args, Type::Int32, Box::new(parsed_expr));

	expect!(Token::RBrace, cur);

	return Ok(new_fn);
}

//Top = Fn = Top Fn
fn parse_top(cur: &mut String) -> Result<Vec<AST>, String> {
	expect!(Token::Function, cur);
	let new_fn = try!(parse_fn(cur));

	if peek!(Token::Function, cur) {
		Ok(push!(try!(parse_top(cur)), new_fn))
	} else {
		Ok(vec!(new_fn))
	}
}

pub fn parse(cur: &mut String) -> Result<AST, String> {
	let parsed = try!(parse_top(cur));
	Ok(AST::Scope(parsed))
}