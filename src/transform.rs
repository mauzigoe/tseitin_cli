use crate::grammar::grammar_bool::Expr;

impl From<Expr> for String {
    fn from(value: Expr) -> Self {
        match value {
            Expr::And(x, _, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} & {})", x, y)
            }
            Expr::Or(x, _, y) => {
                let x: String = (*x).into();
                let y: String = (*y).into();
                format!("({} | {})", x, y)
            }
            Expr::Bracket(_, x, _) => {
                let x: String = (*x).into();
                format!("({})", x)
            }
            Expr::Neg(_, x) => {
                let x: String = (*x).into();
                format!("!({})", x)
            }
            Expr::Var(x) => x,
        }
    }
}
