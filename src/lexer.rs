use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::types::{Atom, Op};

/// Representation of possible errors occuring during lexical analysis
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum LexerErrorCode {
    UnknownToken(String),
    NextCharNotPeekable,
}

/// Tokens used for lexing
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Token {
    Atom(Atom),
    Op(Op),
    LeftBracket,
    RightBracket,
    Eof,
}

fn scan_next_token<'a>(iter: &mut Peekable<Chars>) -> Result<Token,LexerErrorCode> {
    let next = iter.skip_while(|&x| x.is_ascii_whitespace() || x == '\n').next().ok_or(LexerErrorCode::NextCharNotPeekable)?;
    match next {
	'a' ..= 'z' | 'A' ..= 'Z' => {
	    let mut var_name: String = next.to_string();
	    while let Some(x) = iter.next_if(|x| x.is_ascii_alphanumeric() || (x == &'_')) {
		    var_name.push(x);
	    }
;	    return Ok(Token::Atom(Atom::Var(var_name)));
	},
	'(' => Ok(Token::LeftBracket),
	')' => Ok(Token::RightBracket),
	'&' => Ok(Token::Op(Op::And)),
	'|' => Ok(Token::Op(Op::Or)),
	'!' => Ok(Token::Op(Op::Not)),
	'0' => Ok(Token::Atom(Atom::False)),
	'1' => Ok(Token::Atom(Atom::True)),
	'\n' => Ok(Token::Eof),
	_ => {
	    let unknown_token = iter.collect();
	    return Err(LexerErrorCode::UnknownToken(unknown_token));
	}
    }
}

/// Perform a lexical analysis of a boolean equation represented by `String`.
pub fn lex(input: String) -> Result<VecDeque<Token>,LexerErrorCode> {
    let mut store = VecDeque::<Token>::new();


    let mut chars = input.chars().peekable();
    
    loop {
	let token_res = scan_next_token(&mut chars);
	match token_res {
	    Ok(token) => {
		// println!("Token: {:?}", token);
		match token {
		    Token::Eof => break,
		    _ => store.push_back(token),
		}
	    },
	    Err(LexerErrorCode::NextCharNotPeekable) => break,
	    Err(x) => return Err(x),
	}
    };
    Ok(store)
}

