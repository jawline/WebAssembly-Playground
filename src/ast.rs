#[derive(Clone, Copy)]
pub enum Constant {
	Int32(i32)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Type {
	None,
	Int32
}

impl ToString for Type {
	fn to_string(&self) -> String {
		match self {
			Int32 => "i32".to_string()
		}
	}
}

#[derive(Clone, Copy)]
pub enum BinaryOperation {
	Add
}

pub enum AST {
	Literal(Constant),
	Function(String, Vec<String>, Box<AST>),
	BinaryOp(BinaryOperation, Box<AST>, Box<AST>),
}

impl AST {

	pub fn lit(x: i32) -> AST {
		AST::Literal(Constant::Int32(x))
	}

	pub fn add(l: Box<AST>, r: Box<AST>) -> Box<AST> {
		Box::new(AST::BinaryOp(BinaryOperation::Add, l, r))
	}

	pub fn as_t(&self) -> Type {
		match self {
			&AST::Literal(ref x) => {
				match *x {
					Int32 => Type::Int32
				}
			},
			&AST::Function(_, _, ref body) => {
				body.as_t()
			},
			&AST::BinaryOp(ref op, ref left, ref right) =>
				match op {
					Add => if left.as_t() == right.as_t() { left.as_t() } else { Type::None }
				}
		}
	}

	pub fn as_s(&self) -> String {
		match self {
			&AST::Literal(ref x) =>
				match *x {
					Constant::Int32(v) => ("(i32.const ".to_string() + &v.to_string() + ")").to_string()
				},
			&AST::Function(ref name, ref params, ref body) => {

				let mut prelude = "(func $".to_string();
				prelude += &(name.to_string() + " ");
				let plen = params.len();

				for i in 0..plen {
					prelude += &("(param $".to_string() + &i.to_string() + " " + &params[i] + ") ");
				}

				prelude += &("(result ".to_string() + &body.as_t().to_string() + ") ");
				prelude += &body.as_s();
				prelude += ")";
				prelude
			},
			&AST::BinaryOp(ref op, ref left, ref right) =>
				match op {
					Add => {
						("(".to_string() + &left.as_t().to_string() + ".add " + &left.as_s() + " " + &right.as_s() + ")").to_string()
					}
				}
		}
	}
}