use env::{ ID, Env, LocalEnv };
use variance::Variance;
use id::{ Type, Exp };

#[derive(Debug, Clone, PartialEq)]
pub struct ExpVal {
    val: Option<Exp>,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVal {
    gen: Vec<Variance>,
    atoms: Vec<ID<ExpVal>>,
    comps: Vec<ID<ExpVal>>,
}

impl ExpVal {
    pub fn new_empty(ty: Type) -> Self {
        ExpVal { val: None, ty }
    }

    pub fn new(e: Exp, ty: Type) -> Self {
        ExpVal { val: Some(e), ty }
    }

    pub fn val(&self) -> Option<Exp> {
        self.val.clone()
    }

    pub fn ty(&self) -> Type {
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

    pub fn contains_atom(&self, id: &ID<ExpVal>) -> bool {
        self.atoms.contains(id)
    }

    pub fn contains_comp(&self, id: &ID<ExpVal>) -> bool {
        self.comps.contains(id)
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
    pub ty: LocalEnv<'a, TypeVal>,
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
            exp: LocalEnv::new(&self.exp),
            ty: LocalEnv::new(&self.ty),
        }
    }
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b>(&'b self, v: Vec<(String, ExpVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty.scope(vec![]),
        }
    }

    pub fn scope_ty<'b>(&'b self, v: Vec<(String, TypeVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(v),
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty.scope_anon(vec![]),
        }
    }
    
    pub fn scope_ty_anon<'b>(&'b self, v: Vec<TypeVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(v),
        }
    }
}