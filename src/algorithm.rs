use crate::{types::Atom, parser::{Expr, BiOp}};

/// Perform tseitin encoding of [`Expr`]. Returns a [`CNF`](https://en.wikipedia.org/wiki/Conjunctive_normal_form)-conform [`Expr`]. 
pub fn tseitin_encode(expr_input: Expr) -> Result<Expr, String> {
    let (c, optional_tseitin_expr) = tseitin_encode_inner(expr_input, &mut 0);

    let tseitin_expr = optional_tseitin_expr.ok_or("Tseitin Expression could not be created".to_string()); 
    return Ok(Expr::BiOp(Box::new(Expr::Atom(c)), BiOp::And, Box::new(tseitin_expr?)));
}

/// Inner implementation of [`tseitin_encode`].
/// Takes [`Expr`] and incrementable [`var_count`] to track extra variables used for tseiting encoding.\\
/// Returns [`Atom`], [`Option<Expr>`] . [`Atom`] is the identifier for the extra tseitin variable used to encode an boolean expression. [`Option<Expr>`] possibly contains the additional CNF expression introduced by the tseitin encoding, otherwise `None` (e.g. if [`tseitin_encode_inner`] is applied to [`Expr::Atom`]\([`Atom::Var`]\(x\)\))
fn tseitin_encode_inner(expr: Expr, var_count: &mut usize) -> (Atom, Option<Expr>) {
    *var_count += 1;
    let var_name = format!("EXTRA_VAR_{}", var_count);
    let atom_var = Atom::Var(var_name);
    match expr {
        Expr::BiOp(expr_left, op, expr_right) => {
            let (extra_atom_var_left, optional_expr_left) = tseitin_encode_inner(*expr_left, var_count);
            let (extra_atom_var_right, optional_expr_right) = tseitin_encode_inner(*expr_right, var_count);

            let mut tseitin_expr = match op {
		BiOp::And => equivalent_and_expr_cnf(
                    atom_var.clone(),
                    extra_atom_var_left,extra_atom_var_right),
		BiOp::Or => equivalent_or_expr_cnf(
                    atom_var.clone(),
                    extra_atom_var_left,extra_atom_var_right)
	    };

            if let Some(lexpr) = optional_expr_left {
                tseitin_expr = Expr::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(lexpr));
            };
	    
            if let Some(rexpr) = optional_expr_right {
                tseitin_expr = Expr::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(rexpr));
            };

            (atom_var.clone(), Some(tseitin_expr))
        }
        Expr::Not(expr_right) => {
            let (extra_var, optional_expr) = tseitin_encode_inner(*expr_right, var_count);

            let mut tseitin_expr = equivalent_not_expr_cnf(atom_var.clone(), extra_var);
	    if let Some(expr) = optional_expr {
                tseitin_expr = Expr::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr));
            }

            (atom_var.clone(), Some(tseitin_expr))
        }
        Expr::Atom(Atom::True) => (atom_var.clone(), Some(Expr::Atom(atom_var.clone()))),
        Expr::Atom(Atom::False) => (atom_var.clone(), Some(Expr::Not(Box::new(Expr::Atom(atom_var.clone()))))),
        Expr::Atom(x) => (x, None),
    }
}

/// Return the cnf-conform [`Expr`] for `c <==> a & b` ("`c` is true iff `a & b`)
pub fn equivalent_and_expr_cnf(c: Atom, a: Atom, b: Atom) -> Expr {
    let c = Expr::Atom(c.clone());
    let a = Expr::Atom(a.clone());
    let b = Expr::Atom(b.clone());
    Expr::BiOp(
        Box::new(Expr::BiOp(
	    Box::new(Expr::BiOp(
                Box::new(Expr::Not(Box::new(a.clone()))),
                BiOp::Or,
                Box::new(Expr::Not(Box::new(b.clone()))),
	    )),
	    BiOp::Or,
	    Box::new(c.clone()),
        )),
        BiOp::And,
        Box::new(Expr::BiOp(
	    Box::new(Expr::BiOp(
                Box::new(a.clone()),
                BiOp::Or,
                Box::new(Expr::Not(Box::new(c.clone()))),
	    )),
	    BiOp::And,
	    Box::new(Expr::BiOp(
                Box::new(b.clone()),
                BiOp::Or,
                Box::new(Expr::Not(Box::new(c.clone()))),
	    )),
        )),
    )
}

/// Return the cnf-conform [`Expr`] for `c <==> a | b` ("`c` is true iff `a | b`)
pub fn equivalent_or_expr_cnf(c: Atom, a: Atom, b: Atom) -> Expr {
    let c = Expr::Atom(c.clone());
    let a = Expr::Atom(a.clone());
    let b = Expr::Atom(b.clone());
    Expr::BiOp(
        Box::new(Expr::BiOp(
            Box::new(Expr::BiOp(
		Box::new(a.clone()),
		BiOp::Or,
		Box::new(b.clone())
	    )),
            BiOp::Or,
            Box::new(Expr::Not(
		Box::new(c.clone())
	    )),
        )),
        BiOp::And,
        Box::new(Expr::BiOp(
            Box::new(Expr::BiOp(
                Box::new(c.clone()),
                BiOp::Or,
                Box::new(Expr::Not(Box::new(a.clone()))),
            )),
            BiOp::And,
            Box::new(Expr::BiOp(
                Box::new(c.clone()),
                BiOp::Or,
                Box::new(Expr::Not(Box::new(b.clone()))),
            )),
        )),
    )
}
    
/// Return the cnf-conform [`Expr`] for `c <==> !a` (`c` is true iff `!a`)
fn equivalent_not_expr_cnf(c: Atom, a: Atom) -> Expr {
    let c = Expr::Atom(c.clone());
    let a = Expr::Atom(a.clone());
    Expr::BiOp(
        Box::new(Expr::BiOp(
            Box::new(Expr::Not(Box::new(a.clone()))),
            BiOp::Or,
            Box::new(Expr::Not(Box::new(c.clone()))),
        )),
        BiOp::And,
        Box::new(Expr::BiOp(Box::new(a.clone()), BiOp::Or, Box::new(c.clone()))),
    )
}

