#[cfg(test)]
mod tests {
    use tseitin::lexer::{Lexer, Token};

    
    #[test]
    fn test_lexer() {
	let input = " aob1 & (Av1d | 1) | 0 \n".to_string();
	let mut lexer = Lexer::new(input);
	
	let token_store = lexer.scan_complete().unwrap();

	let mut iter = token_store.iter();

	assert_eq!(*iter.next().unwrap(), Token::Var("aob1".to_string()));
	
    } 
}
