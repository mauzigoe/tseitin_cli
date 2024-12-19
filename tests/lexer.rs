#[cfg(test)]
mod tests {
    use tseitin::lexer::{lex, Token};
    use tseitin::types::{Op, Atom};

    
    #[test]
    fn test_lexer() {
	let input = " aob_1 &(!Av1d | 1) | 0 \n".to_string();
	let token_store = lex(input).unwrap();

	let mut iter = token_store.iter();

	assert_eq!(*iter.next().unwrap(), Token::Atom(Atom::Var("aob_1".to_string())));
	assert_eq!(*iter.next().unwrap(), Token::Op(Op::And));
	assert_eq!(*iter.next().unwrap(), Token::LeftBracket);
	assert_eq!(*iter.next().unwrap(), Token::Op(Op::Not));
	assert_eq!(*iter.next().unwrap(), Token::Atom(Atom::Var("Av1d".to_string())));
	assert_eq!(*iter.next().unwrap(), Token::Op(Op::Or));
	assert_eq!(*iter.next().unwrap(), Token::Atom(Atom::True));
	assert_eq!(*iter.next().unwrap(), Token::RightBracket);
	assert_eq!(*iter.next().unwrap(), Token::Op(Op::Or));
	assert_eq!(*iter.next().unwrap(), Token::Atom(Atom::False));
    } 
}
