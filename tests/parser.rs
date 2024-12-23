#[cfg(test)]
mod tests {
    use tseitin::{types::Atom, lexer::lex, parser::{Parser, BiOp, Expr}};

    
    #[test]
    fn test_parser_1() {
	let input = " aob_1 & ( !Av1d | 1 ) | 0".to_string();
	let (token_store, _) = lex(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert_eq!(ast,
		   Expr::BiOp(
		       Box::new(Expr::BiOp(
			   Box::new(Expr::Atom(Atom::Var(1))),
			   BiOp::And,
			   Box::new(Expr::BiOp(
			       Box::new(Expr::Not(
				   Box::new(Expr::Atom(Atom::Var(2))))
			       ),
			       BiOp::Or,
			       Box::new(Expr::Atom(Atom::True))
			   )))),
		       BiOp::Or,
		       Box::new(Expr::Atom(Atom::False))
		   ));
		   } 
    #[test]
    fn test_parser_2() {
	let input = " aB2_1 | !(Z1d & 0)".to_string();
	let (token_store, _) = lex(input).unwrap();
	println!("token_store {:?}", token_store.clone());
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert_eq!(ast,
		   Expr::BiOp(
		       Box::new(Expr::Atom(Atom::Var(1))),
		       BiOp::Or,
		       Box::new(Expr::Not(
			   Box::new(Expr::BiOp(
			       Box::new(Expr::Atom(Atom::Var(2))),
			       BiOp::And,
			       Box::new(Expr::Atom(Atom::False))
		       ))))));
		   } 
}

















