#[cfg(test)]
mod tests {
    use tseitin::{lexer::lex, parser::Parser};

    #[test]
    fn is_cnf_1() {
	let input = " aob_1 & ( !Av1d | 1 ) ".to_string();
	let token_store = lex(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert!(ast.is_cnf());
    }
    #[test]
    fn is_not_cnf_1() {
	let input = " aob_1 | ( !(Av1d | 1) ) & b ".to_string();
	let token_store = lex(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert!(!ast.is_cnf());
    }
    #[test]
    fn is_not_cnf_2() {
	let input = " !(aob_1 | ( Av1d & 1) ) & b ".to_string();
	let token_store = lex(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert!(!ast.is_cnf());
    }
    #[test]
    fn is_not_cnf_3() {
	let input = " !(aob_1 | ( Av1d & 1) ) & b ".to_string();
	let token_store = lex(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert!(!ast.is_cnf());
    }

}
