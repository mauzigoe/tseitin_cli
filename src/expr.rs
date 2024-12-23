use std::{fs::File, io::Write, path::Path};

pub struct VarStore(Vec<String>);

impl VarStore {
    pub fn new() -> Self {
	VarStore(Vec::<String>::new())
    }
    /// Returns a list of variable names.
    pub fn n_var(&self) -> usize {
	self.0.len()
    }
    /// Returns `Some(var_name)` if `(var_name, index): (String, usize)` is contained in [`Self`]
    pub fn try_get_by_index(&self, index: usize) -> Option<&String> {
	self.0.get(index)
    }
    /// Returns `Some(index)` if `(var_name, index): (String, usize)` is contained in [`Self`]
    pub fn try_get_by_string(&self, var_name: &String) -> Option<usize> {
	self.0.iter().position(|x| x == var_name).and_then(|x| Some(x+1))
    }
    /// Checks if [`VarStore`] contains `var_name`
    pub fn contains(& self, var_name: &String) -> bool{
	self.0.contains(var_name)
    }
    /// Adds `var_name` to [`VarStore`]
    pub fn insert(&mut self, var_name: String) {
	if !self.contains(&var_name) {
	    self.0.push(var_name);
	}
    }
    /// Adds extra variable to [`VarStore`]. Returns index of variable.
    pub fn new_extra_var(&mut self) -> i32 {
	let new_var_index = self.n_var() + 1;
	self.insert(format!("EXTRA_VAR_{}",new_var_index).to_string());
	new_var_index as i32
    }
    /// Outputs `csv` file to [`VarStore`]
    pub fn to_csv_file(&self, filename: &str) {
	let path = Path::new(filename);
        let mut file = File::create(path).expect("creating file failed");

        let content = self.0.iter().enumerate().fold(
	    String::new(),
	    |mut old, (index,x)| {
		old.push_str(&format!("{} {}\n", index + 1, x));
		old
	    });
        file.write_all(content.as_bytes()).unwrap();
    }
}

