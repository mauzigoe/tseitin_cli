use crate::{parser::{BiOp, ParserAst}, types::Atom};

impl From<ParserAst> for String {
    fn from(value: ParserAst) -> Self {
        match value {
            ParserAst::BiOp(x, BiOp::And, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} & {})", x, y)
            }
            ParserAst::BiOp(x, BiOp::Or, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} | {})", x, y)
            }
            ParserAst::Not(x) => {
                let x: String = (*x).into();
                format!("!({})", x)
            }
            ParserAst::Atom(Atom::Var(x)) => x,
            ParserAst::Atom(Atom::False) => "False".to_string(),
            ParserAst::Atom(Atom::True) => "True".to_string(),
        }
    }
}
