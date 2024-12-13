use crate::{types::Atom, parser::{ParserAst, BiOp}};

pub fn tseitin_encoder(expr_input: ParserAst) -> Result<ParserAst, String> {
    let (_, optional_tseitin_expr) = process_expr(expr_input, 0);
    optional_tseitin_expr.ok_or("Tseitin Expression could not be created".to_string())
}

fn process_expr(expr: ParserAst, var_count: usize) -> (ParserAst, Option<ParserAst>) {
    let var_name = format!("EXTRA_VAR_{}", var_count);
    let expr_var = ParserAst::Atom(Atom::Var(var_name));
    match expr {
        ParserAst::BiOp(expr_left, BiOp::And, expr_right) => {
            let (extra_var_left, optional_expr_left) = process_expr(*expr_left, var_count + 1);
            let (extra_var_right, optional_expr_right) = process_expr(*expr_right, var_count + 2);

            let mut tseitin_expr = equivalent_expr_cnf(
                &expr_var,
                &ParserAst::BiOp(Box::new(extra_var_left), BiOp::And, Box::new(extra_var_right)),
            )
            .expect("Operands in equivalent_expr_cnf are not ParserAst::Atom(Atom::Var)");
            if let Some(expr_left) = optional_expr_left {
                tseitin_expr = ParserAst::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr_left));
            };
            if let Some(expr_right) = optional_expr_right {
                tseitin_expr = ParserAst::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr_right));
            };

            (expr_var, Some(tseitin_expr))
        }
        ParserAst::BiOp(expr_left, BiOp::Or, expr_right) => {
            let (extra_var_left, optional_expr_left) = process_expr(*expr_left, var_count + 1);
            let (extra_var_right, optional_expr_right) = process_expr(*expr_right, var_count + 2);

            let mut tseitin_expr = equivalent_expr_cnf(
                &expr_var,
                &ParserAst::BiOp(Box::new(extra_var_left), BiOp::Or, Box::new(extra_var_right)),
            )
            .expect("Operands in equivalent_expr_cnf are not ParserAst(Atom::Var)");
            if let Some(expr_left) = optional_expr_left {
                tseitin_expr = ParserAst::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr_left));
            };
            if let Some(expr_right) = optional_expr_right {
                tseitin_expr = ParserAst::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr_right));
            };

            (expr_var, Some(tseitin_expr))
        }
//        ParserAst::BiOp(Op::LeftBracket, expr_mid, Op::RightBracket) => process_expr(*expr_mid, var_count),
        ParserAst::Not(expr_right) => {
            let (extra_var, optional_expr) = process_expr(*expr_right, var_count + 1);

            let mut tseitin_expr =
                equivalent_expr_cnf(&expr_var, &ParserAst::Not(Box::new(extra_var)))
                    .expect("Operands in equivalent_expr_cnf are not ParserAst(Atom::Var)");
            if let Some(expr) = optional_expr {
                tseitin_expr = ParserAst::BiOp(Box::new(tseitin_expr), BiOp::And, Box::new(expr));
            }

            (expr_var, Some(tseitin_expr))
        }
        ParserAst::Atom(Atom::Var(_)) => (expr.clone(), None),
        ParserAst::Atom(_) => (expr.clone(), None),
    }
}

fn equivalent_expr_cnf(c: &ParserAst, a_op_b: &ParserAst) -> Option<ParserAst> {
    match (*a_op_b).clone() {
        ParserAst::BiOp(x, BiOp::And, y) => Some(ParserAst::BiOp(
            Box::new(ParserAst::BiOp(
                Box::new(ParserAst::BiOp(
                    Box::new(ParserAst::Not(x.clone())),
                    BiOp::Or,
                    Box::new(ParserAst::Not(y.clone())),
                )),
                BiOp::Or,
                Box::new((*c).clone()),
            )),
            BiOp::And,
            Box::new(ParserAst::BiOp(
                Box::new(ParserAst::BiOp(
                    x,
                    BiOp::Or,
                    Box::new(ParserAst::Not(Box::new((*c).clone()))),
                )),
                BiOp::And,
                Box::new(ParserAst::BiOp(
                    y,
                    BiOp::Or,
                    Box::new(ParserAst::Not(Box::new((*c).clone()))),
                )),
            )),
        )),
        ParserAst::BiOp(x, BiOp::Or, y) => Some(ParserAst::BiOp(
            Box::new(ParserAst::BiOp(
                Box::new(ParserAst::BiOp(x.clone(), BiOp::Or, y.clone())),
                BiOp::Or,
                Box::new(ParserAst::Not(Box::new((*c).clone()))),
            )),
            BiOp::And,
            Box::new(ParserAst::BiOp(
                Box::new(ParserAst::BiOp(
                    Box::new((*c).clone()),
                    BiOp::Or,
                    Box::new(ParserAst::Not(Box::new((*x).clone()))),
                )),
                BiOp::And,
                Box::new(ParserAst::BiOp(
                    Box::new((*c).clone()),
                    BiOp::Or,
                    Box::new(ParserAst::Not(Box::new((*y).clone()))),
                )),
            )),
        )),
        ParserAst::Not(x) => Some(ParserAst::BiOp(
            Box::new(ParserAst::BiOp(
                Box::new(ParserAst::Not(Box::new((*x).clone()))),
                BiOp::Or,
                Box::new(ParserAst::Not(Box::new((*c).clone()))),
            )),
            BiOp::And,
            Box::new(ParserAst::BiOp(Box::new((*x).clone()), BiOp::Or, Box::new((*c).clone()))),
        )),
        _ => None,
    }
}
