use std::{fs::File, io::Write, path::Path};

use crate::{parser::{BiOp, Expr}, types::Atom};

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
    pub fn variables(&self) -> Vec<String> {
        match self {
            Expr::BiOp(x, BiOp::And | BiOp::Or, y) => {
                let mut ret: Vec<String> = (*x)
                    .variables()
                    .into_iter()
                    .chain((*y).variables())
                    .collect();
                ret.sort();
                ret.dedup();
                ret
            }
            Expr::Not(x) => (*x).variables(),
            Expr::Atom(Atom::Var(x)) => vec![(*x).clone()],
            Expr::Atom(Atom::False | Atom::True) => vec![],
        }
    }
    pub fn to_cnf_file(&self, filename: &str) {
        let path = Path::new(filename);
        let mut file = File::create(path).expect("creating file failed");

        let content = self.to_cnf_string().unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
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
    fn clause_to_dimacs_line(expr: Expr, variables: &Vec<String>) -> Result<String, String> {
        let mut line = Expr::clause_to_dimacs_line_impl(expr, variables)?;
        line.push_str(" 0");
        Ok(line)
    }
    fn clause_to_dimacs_line_impl(expr: Expr, variables: &Vec<String>) -> Result<String, String> {
        match expr {
            Expr::BiOp(x, BiOp::Or, y) => {
                let x_string = Expr::clause_to_dimacs_line_impl(*x, variables)?;
                let y_string = Expr::clause_to_dimacs_line_impl(*y, variables)?;
                Ok(format!("{} {} ", x_string, y_string))
            }
            Expr::Not(x) => {
                let ret = Expr::clause_to_dimacs_line_impl(*x, variables)?;
                Ok(format!("-{}", ret))
            }
            Expr::Atom(Atom::Var(x)) => {
                let ret = (1 + variables.iter().position(|r| *r == x).unwrap()).to_string();
                Ok(ret)
            }
            _ => Err("Cnf Format is inconsistent in clause_to_dimacs_line".to_string()),
        }
    }
    pub fn to_clauses(&self) -> Result<Vec<Expr>, String> {
        if !self.is_cnf() {
            return Err("Expression is not in CNF format".to_string());
        }
        self.to_clauses_impl(CnfLevel::And)
    }
    fn to_clauses_impl(&self, old_level: CnfLevel) -> Result<Vec<Expr>, String> {
        match (self.clone(), old_level.clone()) {
            (Expr::BiOp(x, BiOp::And, y), CnfLevel::And) => {
                let mut clause_x = x.to_clauses_impl(CnfLevel::And)?;
                let mut clause_y = y.to_clauses_impl(CnfLevel::And)?;
                clause_x.append(&mut clause_y);
                Ok(clause_x)
            }
            (Expr::BiOp(_, BiOp::Or, _), CnfLevel::Or | CnfLevel::And)
		| (Expr::Not(_), _) 
		| (Expr::Atom(_), _) => Ok(vec![self.clone()]),
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
            Expr::BiOp(x, BiOp::And, y) => {
                level_transition_is_consistent(*x, CnfLevel::And)
                    & level_transition_is_consistent(*y, CnfLevel::And)
            }
            Expr::BiOp(x, BiOp::Or, y) => {
                level_transition_is_consistent(*x, CnfLevel::Or)
                    & level_transition_is_consistent(*y, CnfLevel::Or)
            }
            Expr::Not(x) => level_transition_is_consistent(*x, CnfLevel::Neg),
            Expr::Atom(_) => true,
        },
        CnfLevel::Or => match expr {
            Expr::BiOp(_, BiOp::And, _) => false,
            Expr::BiOp(x, BiOp::Or, y) => {
                level_transition_is_consistent(*x, CnfLevel::Or)
                    & level_transition_is_consistent(*y, CnfLevel::Or)
            }
            Expr::Not(x) => level_transition_is_consistent(*x, CnfLevel::Neg),
            Expr::Atom(_) => true,
        },
        CnfLevel::Neg => matches!(expr, Expr::Atom(_)),
    }
}
