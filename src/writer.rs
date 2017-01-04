use ast::*;

pub fn to_s(ast: &Vec<AST>) -> String {
	format!("(module {})", ast.iter().fold("".to_string(), |p, ref x| p + &x.as_s()))
}