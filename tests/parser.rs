#[cfg(test)]
mod tests {
    use tseitin::{types::Atom, lexer::scan_complete, parser::{Parser, BiOp, ParserAst}};

    
    #[test]
    fn test_parser_1() {
	let input = " aob_1 & ( !Av1d | 1 ) | 0".to_string();
	let token_store = scan_complete(input).unwrap();
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert_eq!(ast,
		   ParserAst::BiOp(
		       Box::new(ParserAst::BiOp(
			   Box::new(ParserAst::Atom(Atom::Var("aob_1".to_string()))),
			   BiOp::And,
			   Box::new(ParserAst::BiOp(
			       Box::new(ParserAst::Not(
				   Box::new(ParserAst::Atom(Atom::Var("Av1d".to_string()))))
			       ),
			       BiOp::Or,
			       Box::new(ParserAst::Atom(Atom::True))
			   )))),
		       BiOp::Or,
		       Box::new(ParserAst::Atom(Atom::False))
		   ));
		   } 
    #[test]
    fn test_parser_2() {
	let input = " aB2_1 | !(Z1d & 0)".to_string();
	let token_store = scan_complete(input).unwrap();
	println!("token_store {:?}", token_store.clone());
	let mut parser = Parser::new(token_store);
	let ast = parser.process(0);

	assert_eq!(ast,
		   ParserAst::BiOp(
		       Box::new(ParserAst::Atom(Atom::Var("aB2_1".to_string()))),
		       BiOp::Or,
		       Box::new(ParserAst::Not(
			   Box::new(ParserAst::BiOp(
			       Box::new(ParserAst::Atom(Atom::Var("Z1d".to_string()))),
			       BiOp::And,
			       Box::new(ParserAst::Atom(Atom::False))
		       ))))));
		   } 
}

















