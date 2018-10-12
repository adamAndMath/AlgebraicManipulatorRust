use env::{ Env, LocalEnv };
use super::{ LocalEnvs, ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct Envs<'a> {
    pub exp: Env<'a, ExpVal>,
    pub ty: Env<'a, TypeVal>,
    pub truth: Env<'a, TruthVal>,
}

impl<'a> Envs<'a> {
    pub fn new(exps: &'a mut Vec<ExpVal>, tys: &'a mut Vec<TypeVal>, truths: &'a mut Vec<TruthVal>) -> Self {
        Envs {
            exp: Env::new(exps),
            ty: Env::new(tys),
            truth: Env::new(truths),
        }
    }

    pub fn child_scope<'b>(&'b mut self) -> Envs<'b> where 'b: 'a {
        Envs {
            exp: self.exp.child_scope(),
            ty: self.ty.child_scope(),
            truth: self.truth.child_scope(),
        }
    }

    pub fn local<'b>(&'b self) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: LocalEnv::new(&self.exp),
            ty: LocalEnv::new(&self.ty),
            truth: LocalEnv::new(&self.truth),
        }
    }
}
