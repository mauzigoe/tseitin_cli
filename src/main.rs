use std::io::Write;

use crate::tseitin::tseitin_encoder;

mod cnf;
mod grammar;
mod transform;
mod tseitin;

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

        match grammar::grammar_bool::parse(input) {
            Ok(expr) => match tseitin_encoder(expr.clone()) {
                Ok(tseitin_expr) => {
                    let expr_string: String = expr.into();
                    let tseitin_expr_string: String = tseitin_expr.clone().into();
                    println!("expression with parenthesis: {}", expr_string);
                    println!("expression with parenthesis: {}", tseitin_expr_string);
                    println!("variables {:?}", tseitin_expr.variables());
                    let tseitin_is_cnf = tseitin_expr.is_cnf();
                    if tseitin_is_cnf {
                        tseitin_expr.to_cnf_file(&"test.cnf");
                    }
                }
                Err(errs) => {
                    println!("{:?}\n", errs);
                }
            },
            Err(errs) => {
                for error in errs {
                    println!("{:?}\n", error);
                }
            }
        };
    }
}
