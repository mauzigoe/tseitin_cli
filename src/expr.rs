use std::fmt::{Debug, Formatter};

use crate::types::Atom;

/// Representation of the binary operators.
#[derive(Clone, Copy, Debug,Eq,PartialEq)]
pub enum BiOp {
    And,
    Or,
}

/// Representation of a boolean expression with their sequence of operations.
#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    Atom(Atom),
    BiOp(Box<Self>, BiOp, Box<Self>),
    Not(Box<Self>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
	match self {
	    Expr::Atom(atom) => f
		.debug_tuple("Atom")
		.field(atom)
		.finish(),
	    Expr::BiOp(x, op, y) => f
		.debug_tuple("BiOp")
		.field(x)
		.field(op)
		.field(y)
		.finish(),
	    Expr::Not(x) => f
		.debug_tuple("Not")
		.field(x)
		.finish(),
	}
    }
}

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
	    // to be fixed
            Expr::Atom(Atom::Var(x)) => format!("{}",x),
            Expr::Atom(Atom::False) => "False".to_string(),
            Expr::Atom(Atom::True) => "True".to_string(),
        }
    }
}
