use env::Env;
use local_env::LocalEnv;
use exp_id::ExpID;
use ty::{ Variance, TypeID };
use id::ID;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpVal {
    val: Option<ExpID>,
    ty: TypeID,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVal {
    gen: Vec<Variance>,
    atoms: Vec<ID<ExpVal>>,
    comps: Vec<ID<ExpVal>>,
}

impl ExpVal {
    pub fn new_empty(ty: TypeID) -> Self {
        ExpVal { val: None, ty }
    }

    pub fn new(e: ExpID, ty: TypeID) -> Self {
        ExpVal { val: Some(e), ty }
    }

    pub fn val(&self) -> Option<ExpID> {
        self.val.clone()
    }

    pub fn ty(&self) -> TypeID {
        self.ty.clone()
    }
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
}

#[derive(Debug)]
pub struct Envs<'a> {
    pub exp: Env<'a, ExpVal>,
    pub ty: Env<'a, TypeVal>,
}

#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub exp: LocalEnv<'a, ExpVal>,
    pub ty: &'a Env<'a, TypeVal>,
}

impl<'a> Envs<'a> {
    pub fn new(exps: &'a mut Vec<ExpVal>, tys: &'a mut Vec<TypeVal>) -> Self {
        Envs {
            exp: Env::new(exps),
            ty: Env::new(tys),
        }
    }

    pub fn local<'b>(&'b self) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.local(),
            ty: &self.ty,
        }
    }
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b>(&'b self, v: Vec<(String, ExpVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty,
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty,
        }
    }
}