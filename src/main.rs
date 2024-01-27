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
            Ok(expr) => match tseitin_encoder(expr) {
                Ok(tseitin_expr) => {
                    println!("variables {:?}", tseitin_expr.variables());
                    let output: String = tseitin_expr.clone().into();
                    println!("{}", output);
                    println!("{:?}", tseitin_expr);
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
