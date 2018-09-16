use env::ID;
use super::ExpVal;
use variance::Variance;


#[derive(Debug, Clone, PartialEq)]
pub struct TypeVal {
    gen: Vec<Variance>,
    atoms: Vec<ID<ExpVal>>,
    comps: Vec<ID<ExpVal>>,
}

impl TypeVal {
    pub fn new(gen: Vec<Variance>) -> Self {
        TypeVal { gen, atoms: vec!(), comps: vec!() }
    }

    pub fn gen(&self) -> &Vec<Variance> {
        &self.gen
    }

    pub fn push_atom(&mut self, id: ID<ExpVal>) {
        self.atoms.push(id);
    }

    pub fn push_comp(&mut self, id: ID<ExpVal>) {
        self.comps.push(id);
    }

    pub fn contains_atom(&self, id: &ID<ExpVal>) -> bool {
        self.atoms.contains(id)
    }

    pub fn contains_comp(&self, id: &ID<ExpVal>) -> bool {
        self.comps.contains(id)
    }
}
