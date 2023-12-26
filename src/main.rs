use std::io::Write;

mod expression;
mod grammar;
//mod tseitin;

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
            Ok(expr) => println!("{:?}", expr),
            Err(errs) => {
                for error in errs {
                    println!("{:?}\n", error);
                }
            }
        };
    }
}
