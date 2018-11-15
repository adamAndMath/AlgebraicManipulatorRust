use env::ID;
use super::ExpVal;
use variance::Variance;


#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn contains_atom(&self, ty_id: ID<Self>, id: &ID<ExpVal>) -> bool {
        match (ty_id, id) {
            (ID::Predef(_, _), id) => self.atoms.contains(id),
            (ID::Normal(_, ty_up, _), ID::Normal(id, up, p)) if ty_up <= *up => self.atoms.contains(&ID::Normal(*id, up - ty_up, p.clone())),
            _ => false,
        }
    }

    pub fn contains_comp(&self, ty_id: ID<Self>, id: &ID<ExpVal>) -> bool {
        match (ty_id, id) {
            (ID::Predef(_, _), id) => self.comps.contains(id),
            (ID::Normal(_, ty_up, _), ID::Normal(id, up, p)) if ty_up <= *up => self.comps.contains(&ID::Normal(*id, up - ty_up, p.clone())),
            _ => false,
        }
    }
}
