use std::io::Write;
use tseitin::{algorithm::tseitin_encode, lexer::lex, parser::{Expr, Parser}};
use clap::Parser as ClapParser;

#[derive(ClapParser)]
struct Args {
    /// Run benchmark
    #[arg(short,long)]
    output_cnf: Option<String>,
    /// Open a console session
    #[arg(short, long)]
    console: bool,
    #[arg(short, long)]
    input: Option<String>,
}

fn try_to_expr_from(input: String) -> Option<Expr> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

	let string = input.to_string();
	let token = lex(string).unwrap();
	
	let mut parser = Parser::new(token);
	Some(parser.process(0))

}

fn main() {
    
    let args = Args::parse();
    
    let stdin = std::io::stdin();

    let output: String = args.output_cnf.unwrap_or("test.cnf".to_string());

    if let Some(input) = args.input {
	if let Some(expr) = try_to_expr_from(input) {
	    match tseitin_encode(expr) {
		Ok(tseitin_expr) => {
		    let tseitin_is_cnf = tseitin_expr.is_cnf();
		    if tseitin_is_cnf {
			tseitin_expr.to_cnf_file(output.as_str());
		    }
		},
		Err(errs) => {
		    println!("{:?}\n", errs);
		},
	    };
	}
    }

    if args.console {
	loop {
	    print!("> ");
	    std::io::stdout().flush().unwrap();

	    let mut input = String::new();
	    stdin.read_line(&mut input).unwrap();

	    let ast = match try_to_expr_from(input) {
		Some(expr) => expr,
		None => break,
	    };
	    
	    match tseitin_encode(ast) {
		Ok(tseitin_expr) => {
		    let tseitin_is_cnf = tseitin_expr.is_cnf();
		    if tseitin_is_cnf {
			tseitin_expr.to_cnf_file(output.as_str());
		    }
		},
		Err(errs) => {
		    println!("{:?}\n", errs);
		},
	    };
	}
    }
}
