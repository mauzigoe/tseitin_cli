use std::io::Write;

use tseitin::{lexer::scan_complete, parser::Parser, algorithm::tseitin_encoder};

fn main() {
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }

	let string = input.to_string();
	let token = scan_complete(string).unwrap();
	
	let mut parser = Parser::new(token);
	let ast = parser.process(0);
	
	println!("{:#?}",ast);

        match tseitin_encoder(ast.clone()) {
            Ok(tseitin_expr) => {
                println!("expression with parenthesis: {:?}", ast);
                println!("expression with parenthesis: {:?}", tseitin_expr);
                // let tseitin_is_cnf = tseitin_expr.is_cnf();
                // if tseitin_is_cnf {
                //     tseitin_expr.to_cnf_file(&"test.cnf");
                // }
            }
            Err(errs) => {
                println!("{:?}\n", errs);
            }
        };
    }
}
