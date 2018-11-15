use env::{ Namespace, Space, ID, Path };
use super::{ TypeVal, ExpVal, TruthVal };
use ast::ErrAst;

#[derive(Debug)]
pub struct NameData {
    types: Vec<Path<String>>,
    exps: Vec<Path<String>>,
    truths: Vec<Path<String>>,
}

#[derive(Debug)]
pub struct Namespaces<'a> {
    pub types: Namespace<'a, TypeVal>,
    pub exps: Namespace<'a, ExpVal>,
    pub truths: Namespace<'a, TruthVal>,
}

impl NameData {
    pub fn new() -> Self {
        NameData {
            types: vec![],
            exps: vec![],
            truths: vec![],
        }
    }
}

impl<'a> Namespaces<'a> {
    pub fn new<S: AsRef<str>, T, E, P>(data: &'a mut NameData, types: T, exps: E, truths: P) -> Self
            where T: IntoIterator<Item = (S, ID<TypeVal>)>, E: IntoIterator<Item = (S, ID<ExpVal>)>, P: IntoIterator<Item = (S, ID<TruthVal>)>{
        Namespaces {
            types: Namespace::new(&mut data.types, Space::new(types)),
            exps: Namespace::new(&mut data.exps, Space::new(exps)),
            truths: Namespace::new(&mut data.truths, Space::new(truths)),
        }
    }

    pub fn sub_space<'b, S: AsRef<str>>(&'b mut self, n: &S) -> Namespaces<'b> where 'a: 'b {
        Namespaces {
            types: self.types.sub_space(n),
            exps: self.exps.sub_space(n),
            truths: self.truths.sub_space(n),
        }
    }

    pub fn alias<S: Clone + AsRef<str>>(&mut self, n: &S, p: &Path<S>) -> Result<(), Path<S>> {
        let v = vec![
            self.types.alias(n, p),
            self.exps.alias(n, p),
            self.truths.alias(n, p),
        ];
        
        let v: Vec<_> = v.into_iter().filter_map(|r|r.err()).collect();
        if v.len() == 3 {
            Err(v.into_iter().max_by_key(|p|p.len()).unwrap())
        } else {
            Ok(())
        }
    }

    pub fn get_type<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<ID<TypeVal>, ErrAst<S>> {
        self.types.get(p).map_err(|e| if e.len() < p.len() { ErrAst::UndefinedPath(e) } else { ErrAst::UnknownType(e) })
    }

    pub fn get_exp<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<ID<ExpVal>, ErrAst<S>> {
        self.exps.get(p).map_err(|e| if e.len() < p.len() { ErrAst::UndefinedPath(e) } else { ErrAst::UnknownVar(e) })
    }

    pub fn get_truth<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<ID<TruthVal>, ErrAst<S>> {
        self.truths.get(p).map_err(|e| if e.len() < p.len() { ErrAst::UndefinedPath(e) } else { ErrAst::UnknownTruth(e) })
    }

    pub fn scope_empty<'b>(&'b self, names: &'a mut NameData) -> Namespaces<'b> where 'a: 'b {
        Namespaces {
            exps: self.exps.scope_empty(&mut names.exps),
            types: self.types.scope_empty(&mut names.types),
            truths: self.truths.scope_empty(&mut names.truths),
        }
    }

    pub fn scope_type<'b, I: IntoIterator>(&'b self, names: &'a mut NameData, v: I) -> Namespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        Namespaces {
            exps: self.exps.scope_empty(&mut names.exps),
            types: self.types.scope(&mut names.types, v),
            truths: self.truths.scope_empty(&mut names.truths),
        }
    }

    pub fn scope_exp<'b, I: IntoIterator>(&'b self, names: &'a mut NameData, v: I) -> Namespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        Namespaces {
            exps: self.exps.scope(&mut names.exps, v),
            types: self.types.scope_empty(&mut names.types),
            truths: self.truths.scope_empty(&mut names.truths),
        }
    }
    
    pub fn scope_truth<'b, I: IntoIterator>(&'b self, names: &'a mut NameData, v: I) -> Namespaces<'b> where 'a: 'b, I::Item: AsRef<str> {
        Namespaces {
            exps: self.exps.scope_empty(&mut names.exps),
            types: self.types.scope_empty(&mut names.types),
            truths: self.truths.scope(&mut names.truths, v),
        }
    }
}