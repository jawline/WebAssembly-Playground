use ast::*;

pub fn to_s(ast: &AST) -> String {
	let mut result = "(module \n".to_string();

	result += &ast.as_s();

	result += ")";
	result
}