use std::collections::HashSet;

use crate::grammar::grammar_bool::Expr;

#[derive(Clone, Copy, PartialEq)]
enum ExprIteratorTreeOrientation {
    Left,
    Right,
}

#[derive(Clone)]
pub struct ExprIterator {
    expr_ptr: Expr,
    _stack: Vec<(Expr, ExprIteratorTreeOrientation)>,
    came_from: ExprIteratorTreeOrientation,
}

impl IntoIterator for &Expr {
    type Item = Expr;
    type IntoIter = ExprIterator;

    fn into_iter(self) -> Self::IntoIter {
        ExprIterator::gen(self.clone())
    }
}

impl Expr {
    /// Checks if Expr is a terminal in the syntax tree
    fn is_terminal(&self) -> bool {
        matches!(self, Expr::Var(_))
    }
    /// Returns sorted list of variable names
    fn variables(&mut self) -> HashSet<String> {
        let mut name_list = HashSet::<String>::new();

        for expr in self.into_iter() {
            if let Expr::Var(name) = expr {
                if !name_list.contains(&name) {
                    name_list.insert(name);
                }
            };
        }
        name_list
    }
}

impl ExprIterator {
    fn gen(input_expr: Expr) -> ExprIterator {
        ExprIterator {
            expr_ptr: input_expr,
            _stack: Vec::new(),
            came_from: ExprIteratorTreeOrientation::Left,
        }
    }
    fn try_go_up(&mut self) -> Option<()> {
        match self._stack.pop() {
            Some((x, came_from)) => {
                self.expr_ptr = x;
                self.came_from = came_from;
                Some(())
            }
            _ => None,
        }
    }
    fn try_go_left(&mut self) -> Option<()> {
        match &self.expr_ptr {
            Expr::And(expr_left, _, _)
            | Expr::Or(expr_left, _, _)
            | Expr::Bracket(_, expr_left, _)
            | Expr::Neg(_, expr_left) => {
                self._stack
                    .push((self.expr_ptr.to_owned(), ExprIteratorTreeOrientation::Left));
                self.expr_ptr = *expr_left.to_owned();
                Some(())
            }
            Expr::Var(_) => None,
        }
    }
    fn try_go_right(&mut self) -> Option<()> {
        match &self.expr_ptr {
            Expr::And(_, _, expr_right) | Expr::Or(_, _, expr_right) => {
                self._stack
                    .push((self.expr_ptr.to_owned(), ExprIteratorTreeOrientation::Right));
                self.expr_ptr = *expr_right.to_owned();
                Some(())
            }
            _ => None,
        }
    }
}

impl Iterator for ExprIterator {
    type Item = Expr;
    fn next(&mut self) -> Option<Self::Item> {
        if self._stack.is_empty() && self.came_from == ExprIteratorTreeOrientation::Right {
            return None;
        }

        let ret = self.expr_ptr.to_owned();

        if self.try_go_left().is_none() {
            while self.try_go_up().is_some()
                && (!(self.came_from == ExprIteratorTreeOrientation::Left)
                    || self.try_go_right().is_none())
            {
                if self._stack.is_empty() && self.came_from == ExprIteratorTreeOrientation::Right {
                    return Some(ret);
                }
            }
        }

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{expression::ExprIterator, grammar::grammar_bool::Expr};

    // Implements Symbolic Equivalence
    // ( (x AND y) == (x AND y) evals to TRUE,
    // but (x AND y) == NOT ((NOT X) OR (NOT Y)) evals to FALSE
    impl PartialEq for Expr {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Expr::And(self_left_expr, _, self_right_expr),
                    Expr::And(other_left_expr, _, other_right_expr),
                ) => self_left_expr == other_left_expr && self_right_expr == other_right_expr,
                (
                    Expr::Or(self_left_expr, _, self_right_expr),
                    Expr::Or(other_left_expr, _, other_right_expr),
                ) => self_left_expr == other_left_expr && self_right_expr == other_right_expr,
                (Expr::Bracket(_, self_expr, _), Expr::Bracket(_, other_expr, _)) => {
                    self_expr == other_expr
                }
                (Expr::Neg(_, self_expr), Expr::Neg(_, other_expr)) => self_expr == other_expr,
                (Expr::Var(self_var), Expr::Var(other_var)) => self_var == other_var,
                _ => false,
            }
        }
    }

    #[test]
    fn test_next() {
        let x = Expr::Var("x".into());
        let y = Expr::Var("y".into());
        let not_y = Expr::Neg((), Box::new(y.clone()));

        let x_and_y = Expr::And(Box::new(x.clone()), (), Box::new(y.clone()));
        let x_or_not_y = Expr::Or(Box::new(x.clone()), (), Box::new(not_y.clone()));
        let x_or_not_y_in_bracket = Expr::Bracket((), Box::new(x_or_not_y.clone()), ());

        let expr = Expr::Or(
            Box::new(x_and_y.clone()),
            (),
            Box::new(x_or_not_y_in_bracket.clone()),
        );

        let mut expr_iter = expr.into_iter();

        assert_eq!(expr, expr_iter.next().unwrap());
        assert_eq!(x_and_y, expr_iter.next().unwrap());
        assert_eq!(x, expr_iter.next().unwrap());
        assert_eq!(y, expr_iter.next().unwrap());
        assert_eq!(x_or_not_y_in_bracket, expr_iter.next().unwrap());
        assert_eq!(x_or_not_y, expr_iter.next().unwrap());
        assert_eq!(x, expr_iter.next().unwrap());
        assert_eq!(not_y, expr_iter.next().unwrap());
        assert_eq!(y, expr_iter.next().unwrap());
    }
    #[test]
    fn test_next_with_no_right_node_at_root() {
        let x = Expr::Var("x".into());
        let y = Expr::Var("y".into());
        let not_y = Expr::Neg((), Box::new(y.clone()));

        let x_and_y = Expr::And(Box::new(x.clone()), (), Box::new(y.clone()));
        let x_or_not_y = Expr::Or(Box::new(x.clone()), (), Box::new(not_y.clone()));
        let x_or_not_y_in_bracket = Expr::Bracket((), Box::new(x_or_not_y.clone()), ());

        let x_and_y_or_x_or_not_y_in_bracket = Expr::Or(
            Box::new(x_and_y.clone()),
            (),
            Box::new(x_or_not_y_in_bracket.clone()),
        );

        let expr = Expr::Neg((), Box::new(x_and_y_or_x_or_not_y_in_bracket.clone()));

        let mut formula = ExprIterator::gen(expr.clone());

        assert_eq!(expr, formula.next().unwrap());
        assert_eq!(x_and_y_or_x_or_not_y_in_bracket, formula.next().unwrap());
        assert_eq!(x_and_y, formula.next().unwrap());
        assert_eq!(x, formula.next().unwrap());
        assert_eq!(y, formula.next().unwrap());
        assert_eq!(x_or_not_y_in_bracket, formula.next().unwrap());
        assert_eq!(x_or_not_y, formula.next().unwrap());
        assert_eq!(x, formula.next().unwrap());
        assert_eq!(not_y, formula.next().unwrap());
        assert_eq!(y, formula.next().unwrap());
    }
    #[test]
    fn test_variables() {
        let x = Expr::Var("x".into());
        let y = Expr::Var("y".into());
        let z = Expr::Var("z".into());

        let not_y = Expr::Neg((), Box::new(y));
        let not_z = Expr::Neg((), Box::new(z));

        let x_and_not_z = Expr::And(Box::new(x.clone()), (), Box::new(not_z));
        let x_or_not_y = Expr::Or(Box::new(x), (), Box::new(not_y));
        let x_or_not_y_in_bracket = Expr::Bracket((), Box::new(x_or_not_y), ());

        let mut expr = Expr::Or(Box::new(x_and_not_z), (), Box::new(x_or_not_y_in_bracket));

        let mut formula = ExprIterator::gen(expr.clone());

        let mut variables = expr.variables();

        let mut variables_expected = HashSet::<String>::new();
        variables_expected.insert("x".to_string());
        variables_expected.insert("y".to_string());
        variables_expected.insert("z".to_string());

        assert_eq!(variables, variables_expected);
    }
}
