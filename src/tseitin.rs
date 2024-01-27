use crate::grammar::grammar_bool::Expr;

pub fn tseitin_encoder(expr_input: Expr) -> Result<Expr, String> {
    let (_, optional_tseitin_expr) = process_expr(expr_input, 0);
    optional_tseitin_expr.ok_or("Tseitin Expression could not be created".to_string())
}

fn process_expr(expr: Expr, var_count: usize) -> (Expr, Option<Expr>) {
    let var_name = format!("EXTRA_VAR_{}", var_count);
    let expr_var = Expr::Var(var_name);
    match expr {
        Expr::And(expr_left, (), expr_right) => {
            let (extra_var_left, optional_expr_left) = process_expr(*expr_left, var_count + 1);
            let (extra_var_right, optional_expr_right) = process_expr(*expr_right, var_count + 2);

            let mut tseitin_expr = equivalent_expr_cnf(
                &expr_var,
                &Expr::And(Box::new(extra_var_left), (), Box::new(extra_var_right)),
            )
            .expect("Operands in equivalent_expr_cnf are not Expr::Var");
            if let Some(expr_left) = optional_expr_left {
                tseitin_expr = Expr::And(Box::new(tseitin_expr), (), Box::new(expr_left));
            };
            if let Some(expr_right) = optional_expr_right {
                tseitin_expr = Expr::And(Box::new(tseitin_expr), (), Box::new(expr_right));
            };

            (expr_var, Some(tseitin_expr))
        }
        Expr::Or(expr_left, (), expr_right) => {
            let (extra_var_left, optional_expr_left) = process_expr(*expr_left, var_count + 1);
            let (extra_var_right, optional_expr_right) = process_expr(*expr_right, var_count + 2);

            let mut tseitin_expr = equivalent_expr_cnf(
                &expr_var,
                &Expr::Or(Box::new(extra_var_left), (), Box::new(extra_var_right)),
            )
            .expect("Operands in equivalent_expr_cnf are not Expr::Var");
            if let Some(expr_left) = optional_expr_left {
                tseitin_expr = Expr::And(Box::new(tseitin_expr), (), Box::new(expr_left));
            };
            if let Some(expr_right) = optional_expr_right {
                tseitin_expr = Expr::And(Box::new(tseitin_expr), (), Box::new(expr_right));
            };

            (expr_var, Some(tseitin_expr))
        }
        Expr::Bracket((), expr_mid, ()) => process_expr(*expr_mid, var_count),
        Expr::Neg((), expr) => {
            let (extra_var, optional_expr) = process_expr(*expr, var_count + 1);

            let mut tseitin_expr =
                equivalent_expr_cnf(&expr_var, &Expr::Neg((), Box::new(extra_var)))
                    .expect("Operands in equivalent_expr_cnf are not Expr::Var");
            if let Some(expr) = optional_expr {
                tseitin_expr = Expr::And(Box::new(tseitin_expr), (), Box::new(expr));
            }

            (expr_var, Some(tseitin_expr))
        }
        Expr::Var(_) => (expr.clone(), None),
    }
}

fn equivalent_expr_cnf(c: &Expr, a_op_b: &Expr) -> Option<Expr> {
    match (*a_op_b).clone() {
        Expr::And(x, _, y) => Some(Expr::And(
            Box::new(Expr::Or(
                Box::new(Expr::Or(
                    Box::new(Expr::Neg((), x.clone())),
                    (),
                    Box::new(Expr::Neg((), y.clone())),
                )),
                (),
                Box::new((*c).clone()),
            )),
            (),
            Box::new(Expr::And(
                Box::new(Expr::Or(
                    x,
                    (),
                    Box::new(Expr::Neg((), Box::new((*c).clone()))),
                )),
                (),
                Box::new(Expr::Or(
                    y,
                    (),
                    Box::new(Expr::Neg((), Box::new((*c).clone()))),
                )),
            )),
        )),
        Expr::Or(x, _, y) => Some(Expr::And(
            Box::new(Expr::Or(
                Box::new(Expr::Or(x.clone(), (), y.clone())),
                (),
                Box::new(Expr::Neg((), Box::new((*c).clone()))),
            )),
            (),
            Box::new(Expr::And(
                Box::new(Expr::Or(
                    Box::new((*c).clone()),
                    (),
                    Box::new(Expr::Neg((), Box::new((*x).clone()))),
                )),
                (),
                Box::new(Expr::Or(
                    Box::new((*c).clone()),
                    (),
                    Box::new(Expr::Neg((), Box::new((*y).clone()))),
                )),
            )),
        )),
        Expr::Neg(_, x) => Some(Expr::And(
            Box::new(Expr::Or(
                Box::new(Expr::Neg((), Box::new((*x).clone()))),
                (),
                Box::new(Expr::Neg((), Box::new((*c).clone()))),
            )),
            (),
            Box::new(Expr::Or(Box::new((*x).clone()), (), Box::new((*c).clone()))),
        )),
        _ => None,
    }
}
