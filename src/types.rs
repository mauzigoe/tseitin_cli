#[derive(Clone, Copy, Debug,Eq,PartialEq)]
pub enum Op {
    And,
    Or,
    Not,
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Atom {
    Var(usize),
    True,
    False,
}

