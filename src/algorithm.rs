use crate::{cnf::Cnf, var::VarStore, expr::{BiOp, Expr}, types::Atom};

/// Perform tseitin encoding of [`Expr`]. Returns the [`CNF`](https://en.wikipedia.org/wiki/Conjunctive_normal_form)-conform tseitin transformation. 
pub fn tseitin_encode(expr_input: &Expr, var_store: VarStore) -> Cnf {
    let mut cnf = Cnf::new(var_store);
    let c = tseitin_encode_inner(expr_input, &mut cnf);
    cnf.add_clause(vec![c]);
    return cnf;
}

/// Inner implementation of [`tseitin_encode`].
/// Takes [`Expr`] and [`Cnf`] to track extra variables used for tseiting encoding.\\
/// Returns tseitin variable used for tseitin transformation. `tseitin_encode_inner` decends the [`Expr`]-tree in postorder fashion
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

/// Adds boolean relation `c <==> a & b` to `Cnf` as CNF-conform formula. Here `c` is an additonal variable.
pub fn and_cnf(cnf: &mut Cnf, a: i32, b: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit, a]);
    cnf.add_clause(vec![-extra_lit, b]);
    cnf.add_clause(vec![extra_lit, -a,-b]);
    extra_lit
} 

/// Adds boolean relation `c <==> a | b` to `Cnf` as CNF-conform formula. Here `c` is an additional variable.
pub fn or_cnf(cnf: &mut Cnf, a: i32, b: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![extra_lit, -a]);
    cnf.add_clause(vec![extra_lit, -b]);
    cnf.add_clause(vec![-extra_lit, a, b]);
    extra_lit
} 

/// Adds boolean relation `c <==> !a` to `Cnf` as CNF-conform formula. Here `c` is an additional variable. 
pub fn not_cnf(cnf: &mut Cnf, a: i32) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit, -a]);
    cnf.add_clause(vec![extra_lit, a]);
    extra_lit
}

/// Adds boolean relation `c <==> true` to `Cnf` as CNF-conform formula. Here `c` is an additional variable.
pub fn true_cnf(cnf: &mut Cnf) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![extra_lit]);
    extra_lit
}

/// Adds boolean relation `c <==> false` to `Cnf` as CNF-conform formula. Here `c` is an additional variable.
pub fn false_cnf(cnf: &mut Cnf) -> i32 {
    let extra_lit = cnf.mut_var_store().new_extra_var() as i32;
    cnf.add_clause(vec![-extra_lit]);
    extra_lit
}
