use std::collections::VecDeque;

use crate::{expr::{BiOp, Expr}, lexer::Token, types::Op};

pub struct Parser {
    input: VecDeque::<Token>,
}

impl Parser {

    pub fn new(input: VecDeque::<Token>) -> Parser {
	Parser {
	    input,
	}
    }
    
    pub fn process(&mut self, min_bp: u8) -> Expr {
	let val = self.input.pop_front().unwrap();
	let mut lhs = match val {
	    Token::Atom(x) => Expr::Atom(x),
	    Token::LeftBracket => {
		let lhs = self.process(0);
		assert_eq!(self.next(), Token::RightBracket);
		lhs
	    },
	    Token::Op(Op::Not) => {
		let ((), r_bp) = prefix_binding_power(Op::Not).unwrap();
 		let rhs = self.process(r_bp);
		Expr::Not(Box::new(rhs))
	    },
            t => panic!("bad token: {:?}", t),
	};

	loop {
            let op = match self.peek_front() {
		Token::Eof => break,
		Token::Op(op) => op,
		Token::RightBracket => {
		    break
		},
		t => panic!("bad token: {:?}", t),
            };
	    
            if let Some((l_bp, r_bp)) = infix_binding_power(op) {

		if l_bp < min_bp {
		    break;
		}

		self.next();
		let rhs = self.process(r_bp);
		lhs = match op {
		    Op::And => Expr::BiOp(Box::new(lhs),BiOp::And,Box::new(rhs)),
		    Op::Or => Expr::BiOp(Box::new(lhs),BiOp::Or,Box::new(rhs)),
		    _ => panic!("unexpected operator"),
		};
		
		continue;
	    }
	    
	    break;
	}
	lhs
    }
    fn next(&mut self) -> Token {
        self.input.pop_front().unwrap_or(Token::Eof)
    }
    fn peek_front(&mut self) -> Token {
        let ret = self.input.pop_front().unwrap_or(Token::Eof);
	self.input.push_front(ret.clone());
	ret
    }
}

pub fn infix_binding_power(token: Op) -> Option<(u8,u8)> {
    match token {
	Op::And => Some((3,4)),
	Op::Or => Some((1,2)),
	_ => None,
    }
}

pub fn prefix_binding_power(token: Op) -> Option<((),u8)> {
    match token {
	Op::Not => Some(((), 5)),
	_ => None,
    }
}
