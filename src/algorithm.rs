use crate::{cnf::Cnf, var::VarStore, expr::{BiOp, Expr}, types::Atom};

/// Perform tseitin encoding of [`Expr`]. Returns a [`CNF`](https://en.wikipedia.org/wiki/Conjunctive_normal_form)-conform [`Expr`]. 
pub fn tseitin_encode(expr_input: &Expr, var_store: VarStore) -> Cnf {
    let mut cnf = Cnf::new(var_store);
    let c = tseitin_encode_inner(expr_input, &mut cnf);
    cnf.add_clause(vec![c]);
    return cnf;
}

/// Inner implementation of [`tseitin_encode`].
/// Takes [`Expr`] and incrementable [`var_count`] to track extra variables used for tseiting encoding.\\
/// Returns [`Atom`], [`Option<Expr>`] . [`Atom`] is the identifier for the extra tseitin variable used to encode an boolean expression. [`Option<Expr>`] possibly contains the additional CNF expression introduced by the tseitin encoding, otherwise `None` (e.g. if [`tseitin_encode_inner`] is applied to [`Expr::Atom`]\([`Atom::Var`]\(x\)\))
fn tseitin_encode_inner(expr: &Expr, cnf: &mut Cnf) -> i32 {
    match expr {
        Expr::BiOp(expr_left, op, expr_right) => {
            let extra_atom_var_left = tseitin_encode_inner(expr_left, cnf);
            let extra_atom_var_right = tseitin_encode_inner(expr_right, cnf);

            match op {
		BiOp::And => and_cnf(cnf, extra_atom_var_left, extra_atom_var_right),
		BiOp::Or => or_cnf(cnf, extra_atom_var_left, extra_atom_var_right),
	    }
        }
        Expr::Not(expr_right) => {
            let extra_var = tseitin_encode_inner(expr_right, cnf);
            not_cnf(cnf, extra_var)
        }
        Expr::Atom(Atom::True) => true_cnf(cnf),
        Expr::Atom(Atom::False) => false_cnf(cnf),
        Expr::Atom(Atom::Var(x)) => *x as i32,
    }
}


pub fn and_cnf(cnf: &mut Cnf, a: i32, b: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit, a]);
    cnf.add_clause(vec![-extra_lit, b]);
    cnf.add_clause(vec![extra_lit, -a,-b]);
    extra_lit
} 

pub fn or_cnf(cnf: &mut Cnf, a: i32, b: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![extra_lit, -a]);
    cnf.add_clause(vec![extra_lit, -b]);
    cnf.add_clause(vec![-extra_lit, a, b]);
    extra_lit
} 

pub fn not_cnf(cnf: &mut Cnf, a: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit, -a]);
    cnf.add_clause(vec![extra_lit, a]);
    extra_lit
}

pub fn true_cnf(cnf: &mut Cnf) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![extra_lit]);
    extra_lit
}

pub fn false_cnf(cnf: &mut Cnf) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit]);
    extra_lit
}
