use std::io::Write;
use tseitin::{algorithm::tseitin_encode, var::VarStore, lexer::lex, expr::Expr, parser::Parser};
use clap::Parser as ClapParser;

/// Tseiting encoding for boolean expresions (e.g. `a & ( !b | c )`)
#[derive(ClapParser)]
struct Args {
    /// Store tseitin encoding in path `output_cnf`. Stored in Dimacs format.
    #[arg(short,long)]
    output_cnf: Option<String>,
    /// Store (Variable, Literal)-Map  in path `output_csv`.
    #[arg(long)]
    output_csv: Option<String>,
    /// Open a console session
    #[arg(short, long, default_value_t = true)]
    console: bool,
    /// Specify boolean expression for tseitin encoding 
    #[arg(short, long)]
    input: Option<String>,
}

fn try_to_expr_from(input: String) -> Option<(Expr, VarStore)> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

	let string = input.to_string();
	let (token, var_store) = lex(string).unwrap();
	
	let mut parser = Parser::new(token);
	Some((parser.process(0), var_store))
}

fn main() {
    
    let args = Args::parse();
    
    let stdin = std::io::stdin();

    let output_cnf: String = args.output_cnf.unwrap_or("test.cnf".to_string());
    let output_csv: String = args.output_csv.unwrap_or("test.csv".to_string());

    if let Some(input) = args.input {
	if let Some((expr,var_store)) = try_to_expr_from(input) {
	    // add var_store
	    let tseitin_expr =  tseitin_encode(&expr, var_store);
	    tseitin_expr.to_cnf_file(output_cnf.as_str());
	    tseitin_expr.var_store().to_csv_file(output_csv.as_str());
	}
    }

    if args.console {
	loop {
	    print!("> ");
	    std::io::stdout().flush().unwrap();

	    let mut input = String::new();
	    stdin.read_line(&mut input).unwrap();

	    let (ast, var_store) = match try_to_expr_from(input) {
		Some(expr) => expr,
		None => break,
	    };
	    
	    let tseitin_expr = tseitin_encode(&ast, var_store);		
	    tseitin_expr.to_cnf_file(output_cnf.as_str());
	    tseitin_expr.var_store().to_csv_file(output_csv.as_str());
	}
    }
}
