use env::Env;
use super::{ ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct EnvData {
    pub types: Vec<TypeVal>,
    pub exps: Vec<ExpVal>,
    pub truths: Vec<TruthVal>,
}

#[derive(Debug)]
pub struct Envs<'a> {
    pub ty: Env<'a, TypeVal>,
    pub exp: Env<'a, ExpVal>,
    pub truth: Env<'a, TruthVal>,
}

impl<'a> Envs<'a> {
    pub fn new(data: &'a mut EnvData) -> Self {
        Envs {
            exp: Env::new(&mut data.exps),
            ty: Env::new(&mut data.types),
            truth: Env::new(&mut data.truths),
        }
    }

    pub fn scope_empty<'b>(&'b self) -> Envs<'b> where 'a: 'b {
        Envs {
            ty: self.ty.scope(vec![]),
            exp: self.exp.scope(vec![]),
            truth: self.truth.scope(vec![]),
        }
    }

    pub fn scope_ty<'b>(&'b self, v: Vec<TypeVal>) -> Envs<'b> where 'a: 'b {
        Envs {
            ty: self.ty.scope(v),
            exp: self.exp.scope(vec![]),
            truth: self.truth.scope(vec![]),
        }
    }
    
    pub fn scope_exp<'b>(&'b self, v: Vec<ExpVal>) -> Envs<'b> where 'a: 'b {
        Envs {
            ty: self.ty.scope(vec![]),
            exp: self.exp.scope(v),
            truth: self.truth.scope(vec![]),
        }
    }

    pub fn scope_truth<'b>(&'b self, v: Vec<TruthVal>) -> Envs<'b> where 'a: 'b {
        Envs {
            ty: self.ty.scope(vec![]),
            exp: self.exp.scope(vec![]),
            truth: self.truth.scope(v),
        }
    }

    #[cfg(test)]
    pub fn lens(&self) -> (usize, usize, usize) {
        (self.ty.len(), self.exp.len(), self.truth.len())
    }
}
