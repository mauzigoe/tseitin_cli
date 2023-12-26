use std::io::Empty;

use crate::{expression::ExprIterator, grammar::grammar_bool::Expr};

enum SatLevel {
    And,
    Or,
    Neg,
    Var,
}

fn tseitin_encoder(expr_input: Expr) -> Expr {
    let expr_stack = Vec::<Expr>::new();

    let sat_level: SatLevel = SatLevel::And;
}

fn get_sat_level_from_expr(expr_input: Expr, sat_level_old: SatLevel) {
    let expr_iter = expr_input.into_iter();
    for expr in expr_iter {
        let sat_level_new = match expr {
            Expr::And(_, _, _) => SatLevel::And,
            Expr::Or(_, _, _) => SatLevel::Or,
            Expr::Bracket(_, _, _) => sat_level_old,
            Expr::Neg(_, _) => SatLevel::Neg,
            Expr::Var(_) => SatLevel::Var,
        };

        if !sat_level_transition_is_cnf_conform(sat_level_old, sat_level_new) {
            let transformed_expr = match expr {
                Expr::And(expr_left, (), expr_right) => tseitin_transform_and_expression(
                    expr,
                    (*expr_left).clone(),
                    (*expr_right).clone(),
                ),
                Expr::Or(expr_left, (), expr_right) => tseitin_transform_or_expression(
                    expr,
                    (*expr_left).clone(),
                    (*expr_right).clone(),
                ),
                Expr::Neg(_, expr_left) => {
                    tseitin_transform_neg_expression(expr, (*expr_left).clone())
                }
                Expr::Var(_) => tseitin_transform_var_expression(expr),
                Expr::Bracket(_, _, _) => continue,
            };
        }
    }
}

fn tseitin_transform_and_expression(expr: Expr, expr_left: Expr, expr_right: Expr) -> Expr {}

fn tseitin_transform_or_expression(expr: Expr, expr_left: Expr, expr_right: Expr) -> Expr {}

fn tseitin_transform_neg_expression(expr: Expr, expr_left: Expr) -> Expr {}

fn tseitin_transform_var_expression(expr: Expr) -> Expr {}

fn sat_level_transition_is_cnf_conform(sat_level_old: SatLevel, sat_level_new: SatLevel) -> bool {
    (sat_level_old as u32) > (sat_level_old as u32)
}
