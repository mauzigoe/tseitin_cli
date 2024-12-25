use std::{collections::BTreeMap, fs::File, io::Write, path::Path};

pub struct VarStore(BTreeMap<String,usize>);

impl VarStore {
    pub fn new() -> Self {
	VarStore(BTreeMap::<String,usize>::new())
    }
    /// Returns a list of variable names.
    pub fn n_var(&self) -> usize {
	self.0.len()
    }
    /// Returns `Some(index)` if `(var_name, index): (String, usize)` is contained in [`Self`]
    pub fn get(&self, var_name: &String) -> Option<usize> {
	self.0.get(var_name).and_then(|x| Some(*x))
    }
    /// Checks if [`VarStore`] contains `var_name`
    pub fn contains(& self, var_name: &String) -> bool {
	self.0.contains_key(var_name)
    }
    /// Adds `var_name` to [`VarStore`]
    pub fn insert_and_get_index(&mut self, var_name: String) -> usize {
	let index_if_new = self.n_var() + 1;
	if self.0.contains_key(&var_name) {
	    self.0[&var_name]
	} else {
	    self.0.insert(var_name,index_if_new);
	    index_if_new
	}
    }
    /// Adds extra variable to [`VarStore`]. Returns index of variable.
    pub fn new_extra_var(&mut self) -> usize {
	let new_var_index = self.n_var() + 1;
	self.insert_and_get_index(format!("EXTRA_VAR_{}",new_var_index).to_string());
	new_var_index
    }
    /// Outputs `csv` file to [`VarStore`]
    pub fn to_csv_file(&self, filename: &str) {
	let path = Path::new(filename);
        let mut file = File::create(path).expect("creating file failed");

        let content = self.0.iter().fold(
	    String::new(),
	    |mut old, (x,index)| {
		old.push_str(&format!("{} {}\n", index, x));
		old
	    });
        file.write_all(content.as_bytes()).unwrap();
    }
}

