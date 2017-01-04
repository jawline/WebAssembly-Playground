use ast::*;

pub fn to_s(ast: &AST) -> String {
	format!("(module {})", ast.as_s())
}