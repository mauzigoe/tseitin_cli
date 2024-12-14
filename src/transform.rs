use crate::{parser::{BiOp, Expr}, types::Atom};

impl From<Expr> for String {
    fn from(value: Expr) -> Self {
        match value {
            Expr::BiOp(x, BiOp::And, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} & {})", x, y)
            }
            Expr::BiOp(x, BiOp::Or, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} | {})", x, y)
            }
            Expr::Not(x) => {
                let x: String = (*x).into();
                format!("!({})", x)
            }
            Expr::Atom(Atom::Var(x)) => x,
            Expr::Atom(Atom::False) => "False".to_string(),
            Expr::Atom(Atom::True) => "True".to_string(),
        }
    }
}
