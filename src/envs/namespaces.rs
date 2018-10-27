use env::{ Namespace, LocalNamespace, ID, Path };
use super::{ TypeVal, ExpVal, TruthVal, LocalNamespaces };
use ast::ErrAst;

pub struct Namespaces {
    pub types: Namespace<TypeVal>,
    pub exps: Namespace<ExpVal>,
    pub truths: Namespace<TruthVal>,
}

impl Namespaces {
    pub fn new<T, E, P>(types: T, exps: E, truths: P) -> Self
            where T: IntoIterator<Item = (String, ID<TypeVal>)>, E: IntoIterator<Item = (String, ID<ExpVal>)>, P: IntoIterator<Item = (String, ID<TruthVal>)> {
        Namespaces {
            types: Namespace::new(types),
            exps: Namespace::new(exps),
            truths: Namespace::new(truths),
        }
    }

    pub fn local<'a>(&'a self) -> LocalNamespaces<'a> {
        LocalNamespaces {
            types: LocalNamespace::new(&self.types),
            exps: LocalNamespace::new(&self.exps),
            truths: LocalNamespace::new(&self.truths),
        }
    }

    pub fn sub_space<T, F: Fn(&mut Self) -> T>(&mut self, n: String, f: F) -> T {
        let mut sub_space = Namespaces {
            types: self.types.sub_space(n.clone()),
            exps: self.exps.sub_space(n.clone()),
            truths: self.truths.sub_space(n.clone()),
        };
        let re = f(&mut sub_space);
        self.types.add_space(n.clone(), sub_space.types);
        self.exps.add_space(n.clone(), sub_space.exps);
        self.truths.add_space(n.clone(), sub_space.truths);
        re
    }

    pub fn alias(&mut self, n: String, p: &Path) -> Result<(), Path> {
        let v = vec![
            self.types.alias(n.clone(), p),
            self.exps.alias(n.clone(), p),
            self.truths.alias(n.clone(), p),
        ];
        
        match v.into_iter().filter_map(|r|r.err()).max_by_key(|p|p.len()) {
            Some(p) => Err(p),
            None => Ok(()),
        }
    }

    pub fn get_type(&self, p: &Path) -> Result<ID<TypeVal>, ErrAst> {
        self.types.get(p).map_err(ErrAst::UnknownType)
    }

    pub fn get_exp(&self, p: &Path) -> Result<ID<ExpVal>, ErrAst> {
        self.exps.get(p).map_err(ErrAst::UnknownVar)
    }

    pub fn get_truth(&self, p: &Path) -> Result<ID<TruthVal>, ErrAst> {
        self.truths.get(p).map_err(ErrAst::UnknownTruth)
    }
}