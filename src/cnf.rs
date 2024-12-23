use std::{fs::File, io::Write, path::Path};

use crate::expr::VarStore;

pub struct Cnf {
    // better?
    clauses: Vec<Vec<i32>>,
    var_store: VarStore,
}

impl Cnf {
    pub fn new(var_store: VarStore) -> Self {
	let clauses: Vec<Vec<i32>> = Vec::new();
	Self {
	    clauses,
	    var_store,
	}
    }
    pub fn var_store(&self) -> &VarStore {
	&self.var_store
    }
    pub fn mut_var_store(&mut self) -> &mut VarStore {
	&mut self.var_store
    }
    pub fn add_clause(&mut self, clause: Vec<i32>) {
	self.clauses.push(clause);
    }
    /// Write [`Self`] to file located at `filename` encoded in dimacs-format.
    pub fn to_cnf_file(&self, filename: &str) {
        let path = Path::new(filename);
        let mut file = File::create(path).expect("creating file failed");

        let content = self.to_dimacs_string().unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    pub fn to_dimacs_string(&self) -> Result<String, String> {
        let no_var = self.var_store.n_var();
        let no_clauses = self.clauses.len();

        let header = format!("p cnf {} {}\n", no_var, no_clauses);
        let mut body = "".to_string();

        for clause in self.clauses.iter() {
	    let dimacs_line = Cnf::to_dimacs_line(clause)?;
            body.push_str(format!("{}\n",dimacs_line).as_str());
        }

        Ok(format!("{}{}", header.as_str(), body.as_str()))
    }
    fn to_dimacs_line(clause: &Vec<i32>) -> Result<String, String> {
	let mut ret = String::new();
	clause.iter().for_each(|x| ret.push_str(format!("{} ",x).as_str()));
	ret.push_str("0");
	Ok(ret)
    }
}
