use env::{ LocalID, LocalNamespace, Path };
use super::{ TypeVal, ExpVal, TruthVal };
use ast::ErrAst;

#[derive(Debug)]
pub struct LocalNamespaces<'a> {
    pub types: LocalNamespace<'a, TypeVal>,
    pub exps: LocalNamespace<'a, ExpVal>,
    pub truths: LocalNamespace<'a, TruthVal>,
}

impl<'a> LocalNamespaces<'a> {
    pub fn get_type<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<LocalID<TypeVal>, ErrAst<S>> {
        self.types.get(p).map_err(|e| if e.len() < p.len() { print!("{:?}", self); ErrAst::UndefinedPath(e) } else { ErrAst::UnknownType(e) })
    }

    pub fn get_exp<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<LocalID<ExpVal>, ErrAst<S>> {
        self.exps.get(p).map_err(|e| if e.len() < p.len() { ErrAst::UndefinedPath(e) } else { ErrAst::UnknownVar(e) })
    }

    pub fn get_truth<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<LocalID<TruthVal>, ErrAst<S>> {
        self.truths.get(p).map_err(|e| if e.len() < p.len() { ErrAst::UndefinedPath(e) } else { ErrAst::UnknownTruth(e) })
    }

    pub fn scope_type<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        LocalNamespaces {
            exps: self.exps.scope_empty(),
            types: self.types.scope(v),
            truths: self.truths.scope_empty(),
        }
    }

    pub fn scope_exp<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        LocalNamespaces {
            exps: self.exps.scope(v),
            types: self.types.scope_empty(),
            truths: self.truths.scope_empty(),
        }
    }
    
    pub fn scope_truth<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        LocalNamespaces {
            exps: self.exps.scope_empty(),
            types: self.types.scope_empty(),
            truths: self.truths.scope(v),
        }
    }
}