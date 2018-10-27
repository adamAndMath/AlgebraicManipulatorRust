use env::{ LocalID, LocalNamespace, Path };
use super::{ TypeVal, ExpVal, TruthVal };
use ast::ErrAst;

pub struct LocalNamespaces<'a> {
    pub types: LocalNamespace<'a, TypeVal>,
    pub exps: LocalNamespace<'a, ExpVal>,
    pub truths: LocalNamespace<'a, TruthVal>,
}

impl<'a> LocalNamespaces<'a> {
    pub fn get_type(&self, p: &Path) -> Result<LocalID<TypeVal>, ErrAst> {
        self.types.get(p).map_err(ErrAst::UnknownType)
    }

    pub fn get_exp(&self, p: &Path) -> Result<LocalID<ExpVal>, ErrAst> {
        self.exps.get(p).map_err(ErrAst::UnknownVar)
    }

    pub fn get_truth(&self, p: &Path) -> Result<LocalID<TruthVal>, ErrAst> {
        self.truths.get(p).map_err(ErrAst::UnknownTruth)
    }

    pub fn scope_type<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: Clone + Into<Path> {
        LocalNamespaces {
            exps: self.exps.scope_empty(),
            types: self.types.scope(v),
            truths: self.truths.scope_empty(),
        }
    }

    pub fn scope_exp<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: Clone + Into<Path> {
        LocalNamespaces {
            exps: self.exps.scope(v),
            types: self.types.scope_empty(),
            truths: self.truths.scope_empty(),
        }
    }
    
    pub fn scope_truth<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespaces<'b> where 'a: 'b, I::Item: Clone + Into<Path> {
        LocalNamespaces {
            exps: self.exps.scope_empty(),
            types: self.types.scope_empty(),
            truths: self.truths.scope(v),
        }
    }
}