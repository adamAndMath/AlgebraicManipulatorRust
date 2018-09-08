use env::Env;
use local_env::LocalEnv;
use exp_id::ExpID;
use ty::{ Variance, TypeID };
use id::ID;

pub type ExpVal = (Option<ExpID>, TypeID);
pub type TypeVal = (Vec<Variance>, Vec<ID<ExpVal>>, Vec<ID<ExpVal>>);

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