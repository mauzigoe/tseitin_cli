pub struct VarStore(Vec<String>);

impl VarStore {
    pub fn new() -> Self {
	VarStore(Vec::<String>::new())
    }
    /// Returns a list of variable names.
    pub fn n_var(&self) -> usize {
	self.0.len()
    }
    pub fn try_get_by_index(&self, index: usize) -> Option<&String> {
	self.0.get(index)
    }
    pub fn try_get_by_string(&self, var_name: &String) -> Option<usize> {
	self.0.iter().position(|x| x == var_name).and_then(|x| Some(x+1))
    }
    pub fn contains(& self, var_name: &String) -> bool{
	self.0.contains(var_name)
    }
    pub fn insert(&mut self, var_name: String) {
	if !self.contains(&var_name) {
	    self.0.push(var_name);
	}
    }
    pub fn new_extra_var(&mut self) -> i32 {
	let new_var_index = self.n_var() + 1;
	self.insert(format!("EXTRA_VAR_{}",new_var_index).to_string());
	new_var_index as i32
    }
}

