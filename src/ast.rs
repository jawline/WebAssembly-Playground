use std::io::Write; //For warn!

#[derive(Clone, Copy)]
pub enum Constant {
	Int32(i32)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Type {
	None,
	Int32
}

pub type Arg = (String, Type);
pub type Args = Vec<Arg>;

impl ToString for Type {
	fn to_string(&self) -> String {
		match *self {
			Type::Int32 => "i32".to_string(),
			Type::None => "none".to_string()
		}
	}
}

#[derive(Clone, Copy)]
pub enum BinaryOperation {
	Add, Subtract, Multiply, Divide, Mod
}

impl BinaryOperation {
	fn instr(&self) -> String {
		match *self {
			BinaryOperation::Add => "add",
			BinaryOperation::Subtract => "sub",
			BinaryOperation::Multiply => "mul",
			BinaryOperation::Divide => "div",
			BinaryOperation::Mod => "mod"
		}.to_string()
	}
}

pub enum AST {
	Literal(Constant),
	Function(String, Args, Box<AST>),
	BinaryOp(BinaryOperation, Box<AST>, Box<AST>),
	Local(usize, Arg)
}

impl AST {

	pub fn lit(x: i32) -> AST {
		AST::Literal(Constant::Int32(x))
	}

	pub fn as_t(&self) -> Type {
		match self {
			&AST::Literal(ref x) => {
				match *x {
					Constant::Int32(_) => Type::Int32
				}
			},
			&AST::Function(_, _, ref body) => {
				body.as_t()
			},
			&AST::Local(_, ref arg) => {
				arg.1
			},
			&AST::BinaryOp(_, ref left, ref right) => if left.as_t() == right.as_t() { left.as_t() } else { Type::None }
		}
	}

	pub fn as_s(&self) -> String {
		match self {
			&AST::Literal(ref x) =>
				match *x {
					Constant::Int32(v) => format!("(i32.const {})", v)
				},
			&AST::Function(ref name, ref params, ref body) => {

				let mut params_text = "".to_string();

				let plen = params.len();

				for i in 0..plen {
					params_text += &format!("(param ${} {})", i, "i32");
				}

				let ret = format!("(result {})", body.as_t().to_string());

				let exp = format!("(export {} ${})", name, name);
				let func = format!("(func ${} {} {} {})", name, params_text, ret, body.as_s());

				format!("{} {}", exp, func)
			},
			&AST::Local(ref size, _) => {
				format!("(get_local ${})", size)
			}
			&AST::BinaryOp(ref op, ref left, ref right) => format!("({}.{} {} {})", left.as_t().to_string(), op.instr(), left.as_s(), right.as_s())
		}
	}
}