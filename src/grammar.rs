#[derive(Debug, Clone)]
pub enum BinaryOperation {
    And,
    Or,
    Xor,
    Eq,
}

#[rust_sitter::grammar("grammar_bool")]
pub mod grammar_bool {
    use super::BinaryOperation;

    #[rust_sitter::language]
    #[derive(Debug, Clone)]
    pub enum Expr {
        PosLit(
            #[rust_sitter::leaf(pattern = r"[0-9a-zA-Z]+", transform = |v| v[0..].to_string())]
            String,
        ),
        NegLit(
            #[rust_sitter::leaf(pattern = r"-[0-9a-zA-Z]+", transform = |v| v[1..].to_string())]
            String,
        ),
        #[rust_sitter::prec_left(1)]
        BinOp(
            Box<Expr>,
            #[
                rust_sitter::leaf(
                    text = "&",
                    transform = |v| match v {
                        "|" => BinaryOperation::Or,
                        "&" => BinaryOperation::And ,
                        "^" => BinaryOperation::Xor,
                        "=" => BinaryOperation::Eq,
                        _ => {
                            panic!("Binary Operator not known")
                        }
                    }
                )
            ]
            BinaryOperation,
            Box<Expr>,
        ),
        Bracket(
            #[rust_sitter::leaf(text = "(")] (),
            Box<Expr>,
            #[rust_sitter::leaf(text = ")")] (),
        ),
    }
    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}
