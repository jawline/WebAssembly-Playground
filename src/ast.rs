use std::io::Write;

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
	Add, Subtract, Multiply, Divide, Mod, GreaterThan, LessThan
}

impl BinaryOperation {
	fn instr(&self) -> String {
		match *self {
			BinaryOperation::Add => "add",
			BinaryOperation::Subtract => "sub",
			BinaryOperation::Multiply => "mul",
			BinaryOperation::Divide => "div",
			BinaryOperation::Mod => "mod",
			BinaryOperation::GreaterThan => "gt_s",
			BinaryOperation::LessThan => "lt_s"
		}.to_string()
	}
}

pub enum AST {
	Literal(Constant),
	Function(String, Args, Box<AST>),
	BinaryOp(BinaryOperation, Box<AST>, Box<AST>),
	Local(usize, Arg),
	If(Box<AST>, Box<AST>, Box<AST>),
	Call(String, Vec<AST>),
	Scope(Vec<AST>)
}

impl AST {

	pub fn lit(x: i32) -> AST {
		AST::Literal(Constant::Int32(x))
	}

	pub fn as_t(&self) -> Type {
		match self {
			&AST::Scope(_) => {
				Type::None
			},
			&AST::Call(_, _) => {
				warn!("Fn calls not TypeSafe");
				Type::Int32
			},
			&AST::If(_, ref left, ref right) => {
				if left.as_t() == right.as_t() { left.as_t() } else { Type::None }
			},
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
			&AST::Scope(ref functions) => {
				let function_asts = functions.iter().fold("".to_string(), |prev, next| prev + &next.as_s());
				format!("(module {})", function_asts)
			},
			&AST::Call(ref name, ref args) => {
				let arg_asts = args.iter().fold("".to_string(), |p, n| p + &n.as_s());
				format!("(call ${} {})", name, arg_asts)
			},
			&AST::If(ref cnd, ref left, ref right) => format!("(if {} {} {} {})", left.as_t().to_string(), cnd.as_s(), left.as_s(), right.as_s()),
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