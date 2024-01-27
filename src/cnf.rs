use crate::grammar::grammar_bool::Expr;
use std::{fs::File, io::Write, path::Path};

#[derive(Clone)]
enum CnfLevel {
    And,
    Or,
    Neg,
}

impl Expr {
    pub fn is_cnf(&self) -> bool {
        level_transition_is_consistent(self.clone(), CnfLevel::And)
    }
    /// TODO does not work correctly
    pub fn variables(&self) -> Vec<String> {
        match self {
            Expr::And(x, (), y) => {
                let mut ret: Vec<String> = (*x)
                    .variables()
                    .into_iter()
                    .chain((*y).variables())
                    .collect();
                ret.sort();
                ret.dedup();
                ret
            }
            Expr::Or(x, (), y) => {
                let mut ret: Vec<String> = (*x)
                    .variables()
                    .into_iter()
                    .chain((*y).variables())
                    .collect();
                ret.sort();
                ret.dedup();
                ret
            }
            Expr::Neg((), x) => (*x).variables(),
            Expr::Bracket((), x, ()) => (*x).variables(),
            Expr::Var(x) => vec![(*x).clone().replace('\n', "")],
        }
    }
    pub fn to_cnf_file(&self, filename: &str) {
        let path = Path::new(filename);
        let mut file = File::create(path).expect("creating file failed");

        let content = self.to_cnf_string().unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    /// TODO erroneous line breaks (see cmd ouput)
    /// TODO map variables to number
    pub fn to_cnf_string(&self) -> Result<String, String> {
        let variables = self.variables();

        let clauses = self.to_clauses()?;

        let no_var = variables.len();
        let no_clauses = clauses.len();

        let header = format!("p cnf {} {}\n", no_var, no_clauses);
        let mut body = "".to_string();

        for expr in clauses {
            let result_dimacs_line = Expr::clause_to_dimacs_line(expr, &variables);
            body = format!(
                "{}{}\n",
                body.as_str(),
                result_dimacs_line.clone()?.as_str()
            );
        }

        Ok(format!("{}{}", header.as_str(), body.as_str()))
    }
    /// TODO erroneuous line break
    /// negated variables
    /// variables index + 1
    fn clause_to_dimacs_line(expr: Expr, variables: &Vec<String>) -> Result<String, String> {
        let mut line = Expr::clause_to_dimacs_line_inner(expr, variables)?;
        line.push_str(" 0");
        Ok(line)
    }
    fn clause_to_dimacs_line_inner(expr: Expr, variables: &Vec<String>) -> Result<String, String> {
        match expr {
            Expr::Or(x, _, y) => {
                let x_string = Expr::clause_to_dimacs_line_inner(*x, variables)?;
                let y_string = Expr::clause_to_dimacs_line_inner(*y, variables)?;
                Ok(format!("{} {} ", x_string, y_string))
            }
            Expr::Neg(_, x) => {
                let ret = Expr::clause_to_dimacs_line_inner(*x, variables)?;
                Ok(format!("-{}", ret))
            }
            Expr::Var(x) => {
                let ret = (1 + variables.iter().position(|r| *r == x).unwrap()).to_string();
                Ok(ret)
            }
            Expr::Bracket(_, x, _) => {
                let ret = Expr::clause_to_dimacs_line_inner(*x, variables)?;
                Ok(ret)
            }
            _ => Err("Cnf Format is inconsistent in clause_to_dimacs_line".to_string()),
        }
    }
    pub fn to_clauses(&self) -> Result<Vec<Expr>, String> {
        if !self.is_cnf() {
            return Err("Expression is not in CNF format".to_string());
        }
        self.to_clauses_impl_and(CnfLevel::And)
    }
    /// TODO Work on order (first check old_level, then self)
    fn to_clauses_impl_and(&self, old_level: CnfLevel) -> Result<Vec<Expr>, String> {
        match (self.clone(), old_level.clone()) {
            (Expr::And(x, (), y), CnfLevel::And) => {
                let mut clause_x = x.to_clauses_impl_and(CnfLevel::And)?;
                let mut clause_y = y.to_clauses_impl_and(CnfLevel::And)?;
                clause_x.append(&mut clause_y);
                Ok(clause_x)
            }
            _ => Ok(vec![self.to_clauses_impl_or(CnfLevel::And)?]),
        }
    }
    /// TODO Work on order (first check old_level, then self)
    fn to_clauses_impl_or(&self, old_level: CnfLevel) -> Result<Expr, String> {
        match (self.clone(), old_level.clone()) {
            (Expr::Or(_, (), _), CnfLevel::And | CnfLevel::Or) => Ok(self.clone()),
            (Expr::Bracket((), x, ()), _) => Ok(x.to_clauses_impl_or(CnfLevel::Neg)?),
            (Expr::Neg((), _), _) => Ok(self.clone()),
            (Expr::Var(_), _) => Ok(self.clone()),
            _ => Err(format!(
                "Cnf is inconsitent. Error in Expression {:?}",
                self
            )),
        }
    }
}

fn level_transition_is_consistent(expr: Expr, old_level: CnfLevel) -> bool {
    match old_level {
        CnfLevel::And => match expr {
            Expr::And(x, _, y) => {
                level_transition_is_consistent(*x, CnfLevel::And)
                    & level_transition_is_consistent(*y, CnfLevel::And)
            }
            Expr::Or(x, _, y) => {
                level_transition_is_consistent(*x, CnfLevel::Or)
                    & level_transition_is_consistent(*y, CnfLevel::Or)
            }
            Expr::Bracket(_, x, _) => level_transition_is_consistent(*x, old_level),
            Expr::Neg(_, x) => level_transition_is_consistent(*x, CnfLevel::Neg),
            Expr::Var(_) => true,
        },
        CnfLevel::Or => match expr {
            Expr::And(_, _, _) => false,
            Expr::Or(x, _, y) => {
                level_transition_is_consistent(*x, CnfLevel::Or)
                    & level_transition_is_consistent(*y, CnfLevel::Or)
            }
            Expr::Bracket(_, x, _) => level_transition_is_consistent(*x, old_level),
            Expr::Neg(_, x) => level_transition_is_consistent(*x, CnfLevel::Neg),
            Expr::Var(_) => true,
        },
        CnfLevel::Neg => matches!(expr, Expr::Var(_)),
    }
}
