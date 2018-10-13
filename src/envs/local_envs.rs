use env::LocalEnv;
use super::{ ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct LocalEnvs<'f: 'a, 'a> {
    pub exp: LocalEnv<'f, 'a, ExpVal>,
    pub ty: LocalEnv<'f, 'a, TypeVal>,
    pub truth: LocalEnv<'f, 'a, TruthVal>,
}

impl<'f, 'a> LocalEnvs<'f, 'a> {
    pub fn scope<'b>(&'b self, v: Vec<(&'f str, ExpVal)>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty.scope(vec![]),
            truth: self.truth.scope(vec![]),
        }
    }

    pub fn scope_ty<'b>(&'b self, v: Vec<(&'f str, TypeVal)>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(v),
            truth: self.truth.scope(vec![]),
        }
    }
    
    pub fn scope_truth<'b>(&'b self, v: Vec<(&'f str, TruthVal)>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(vec![]),
            truth: self.truth.scope(v),
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty.scope_anon(vec![]),
            truth: self.truth.scope_anon(vec![]),
        }
    }
    
    pub fn scope_ty_anon<'b>(&'b self, v: Vec<TypeVal>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(v),
            truth: self.truth.scope_anon(vec![]),
        }
    }
    
    pub fn scope_truth_anon<'b>(&'b self, v: Vec<TruthVal>) -> LocalEnvs<'f, 'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(vec![]),
            truth: self.truth.scope_anon(v),
        }
    }
}
