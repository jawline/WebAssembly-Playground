#[derive(Clone, Copy)]
pub enum Constant {
	None,
	Int32(i32)
}

#[derive(Clone, Copy)]
pub enum BinaryOperation {
	Add
}

pub enum AST {
	Literal(Constant),
	Function(String, Vec<String>, String, Box<AST>),
	BinaryOp(BinaryOperation, Box<AST>, Box<AST>),
}

impl AST {

	pub fn lit(x: i32) -> Box<AST> {
		Box::new(AST::Literal(Constant::Int32(5)))
	}

	pub fn add(l: Box<AST>, r: Box<AST>) -> Box<AST> {
		Box::new(AST::BinaryOp(BinaryOperation::Add, l, r))
	}

	pub fn as_s(&self) -> String {
		match self {
			&AST::Literal(ref x) =>
				match *x {
					Constant::Int32(v) => ("(i32.const ".to_string() + &v.to_string() + ")").to_string(),
					Constant::None => "error".to_string()
				},
			&AST::Function(ref name, ref params, ref ret, ref body) => {

				let mut prelude = "(func $".to_string();
				prelude += &(name.to_string() + " ");
				let plen = params.len();

				for i in 0..plen {
					prelude += &("(param $".to_string() + &i.to_string() + " " + &params[i] + ") ");
				}

				prelude += &("(result ".to_string() + &ret + ") ");
				prelude += &body.as_s();
				prelude += ")";
				prelude
			},
			&AST::BinaryOp(ref op, ref left, ref right) =>
				match op {
					Add => {
						("(i32.add ".to_string() + &left.as_s() + " " + &right.as_s() + ")").to_string()
					}
				}
		}
	}
}