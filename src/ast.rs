pub enum Constant {
	None,
	Int32(i32)
}

pub enum BinaryOperation {
	Add
}

pub enum AST {
	Literal(Constant),
	Function(String, Vec<String>, String, Box<AST>),
	BinaryOp(BinaryOperation, Box<AST>, Box<AST>)
}

impl AST {

	pub fn as_s(&self) -> String {
		match self {
			&AST::Literal(x) =>
				match x {
					Constant::Int32(v) => ("(i32.const ".to_string() + &v.to_string() + ")").to_string(),
					Constant::None => "error".to_string()
				},
			&AST::Function(name, params, ret, body) => {

				let mut prelude = "(func ".to_string();
				let plen = params.len();

				for i in 0..plen {
					prelude += &("(param $".to_string() + &i.to_string() + " " + &params[i] + ") ");
				}

				prelude += &("(result ".to_string() + &ret + ") ");
				prelude += &body.as_s();
				prelude += ")";
				prelude
			}
		}
	}
}