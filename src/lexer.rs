use std::{iter::Peekable, str::Chars};

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum LexerErrorCode {
    UnknownToken(String),
    NextCharNotPeekable,
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Token {
    Var(String),
    True,
    False,
    And,
    Or,
    LeftBracket,
    RightBracket,
    Eof,
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer{
	Lexer { input }
    }
    fn scan_next_token<'a>(iter: &mut Chars<'a>) -> Result<Token, LexerErrorCode> {
	let mut binding = iter.skip_while(|x| x == &' ').peekable();
	let first_char = binding.peek().ok_or(LexerErrorCode::NextCharNotPeekable)?;
	match first_char {
	    'a' ..= 'z' | 'A' ..= 'Z' => {
		let var_name: String = binding.take_while(|x| x.is_ascii_alphanumeric()).collect();
		return Ok(Token::Var(var_name));
	    },
	    '(' => Ok(Token::LeftBracket),
	    ')' => Ok(Token::RightBracket),
	    '&' => Ok(Token::And),
	    '0' => Ok(Token::False),
	    '1' => Ok(Token::True),
	    '|' => Ok(Token::Or),
	    '\n' => Ok(Token::Eof),
	    _ => {
		let unknown_token = iter.take_while(|x| !x.is_ascii_whitespace()).collect();
		return Err(LexerErrorCode::UnknownToken(unknown_token));
	    }
	}
    }
    pub fn scan_complete(&mut self) -> Result<Vec<Token>,LexerErrorCode> {
	let mut store = Vec::<Token>::new();
	let mut chars = self.input.chars();
	loop {
	    let token = Self::scan_next_token(&mut chars)?;
	    println!("Token: {:?}", token);
	    match token {
		Token::Eof => break,
		_ => store.push(token),
	    }
	};
	Ok(store)
    }
}
