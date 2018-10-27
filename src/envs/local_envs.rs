use env::LocalEnv;
use super::{ ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub ty: LocalEnv<'a, TypeVal>,
    pub exp: LocalEnv<'a, ExpVal>,
    pub truth: LocalEnv<'a, TruthVal>,
}

impl<'a> LocalEnvs<'a> {
    pub fn scope_ty<'b>(&'b self, v: Vec<TypeVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            ty: self.ty.scope(v),
            exp: self.exp.scope(vec![]),
            truth: self.truth.scope(vec![]),
        }
    }
    
    pub fn scope_exp<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            ty: self.ty.scope(vec![]),
            exp: self.exp.scope(v),
            truth: self.truth.scope(vec![]),
        }
    }

    pub fn scope_truth<'b>(&'b self, v: Vec<TruthVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            ty: self.ty.scope(vec![]),
            exp: self.exp.scope(vec![]),
            truth: self.truth.scope(v),
        }
    }
}
