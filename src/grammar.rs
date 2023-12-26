#[rust_sitter::grammar("grammar_bool")]
pub mod grammar_bool {
    #[rust_sitter::language]
    #[derive(Debug, Clone)]
    pub enum Expr {
        Var(#[rust_sitter::leaf(pattern = r"[0-9a-zA-Z]+", transform = |v| v.to_string())] String),
        #[rust_sitter::prec_left(1)]
        Neg(#[rust_sitter::leaf(pattern = "!|not")] (), Box<Expr>),
        #[rust_sitter::prec_left(2)]
        And(
            Box<Expr>,
            #[rust_sitter::leaf(pattern = "&|&&|and")] (),
            Box<Expr>,
        ),
        #[rust_sitter::prec_left(3)]
        Or(
            Box<Expr>,
            #[rust_sitter::leaf(pattern = r"\||\|\||or|")] (),
            Box<Expr>,
        ),
        Bracket(
            #[rust_sitter::leaf(text = "(")] (),
            Box<Expr>,
            #[rust_sitter::leaf(text = ")")] (),
        ),
    }
    #[rust_sitter::extra]
    pub struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}
