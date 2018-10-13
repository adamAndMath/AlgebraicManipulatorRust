use env::LocalEnv;
use super::{ ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub exp: LocalEnv<'a, ExpVal>,
    pub ty: LocalEnv<'a, TypeVal>,
    pub truth: LocalEnv<'a, TruthVal>,
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b>(&'b self, v: Vec<(&str, ExpVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty.scope(vec![]),
            truth: self.truth.scope(vec![]),
        }
    }

    pub fn scope_ty<'b>(&'b self, v: Vec<(&str, TypeVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(v),
            truth: self.truth.scope(vec![]),
        }
    }
    
    pub fn scope_truth<'b>(&'b self, v: Vec<(&str, TruthVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(vec![]),
            truth: self.truth.scope(v),
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty.scope_anon(vec![]),
            truth: self.truth.scope_anon(vec![]),
        }
    }
    
    pub fn scope_ty_anon<'b>(&'b self, v: Vec<TypeVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(v),
            truth: self.truth.scope_anon(vec![]),
        }
    }
    
    pub fn scope_truth_anon<'b>(&'b self, v: Vec<TruthVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(vec![]),
            truth: self.truth.scope_anon(v),
        }
    }
}
