use predef::*;
use env::{ Env, LocalEnv };
use super::{ EnvsData, LocalEnvs, ExpVal, TypeVal, TruthVal };
use std::fs::read_to_string;
use parser::{ parse_file, Error };

#[derive(Debug)]
pub struct Envs<'a> {
    pub path: String,
    pub exp: Env<'a, ExpVal>,
    pub ty: Env<'a, TypeVal>,
    pub truth: Env<'a, TruthVal>,
}

impl<'a> Envs<'a> {
    pub fn new(path: String, data: &'a mut EnvsData) -> Self {
        Envs {
            path,
            exp: Env::new(&mut data.exps),
            ty: Env::new(&mut data.types),
            truth: Env::new(&mut data.truths),
        }
    }

    pub fn child_scope<E, F: Fn(&mut Envs) -> Result<(), E>>(&mut self, n: &str, f: F) -> Result<(), E> {
        let (exp, ty, truth) = {
            let mut child = Envs {
                path: format!("{}\\{}", self.path, n),
                exp: self.exp.child_scope(),
                ty: self.ty.child_scope(),
                truth: self.truth.child_scope(),
            };
            alias_predef(&mut child);
            f(&mut child)?;
            (child.exp.to_val(), child.ty.to_val(), child.truth.to_val())
        };
        self.exp.add_val(n, exp);
        self.ty.add_val(n, ty);
        self.truth.add_val(n, truth);
        Ok(())
    }

    pub fn local<'b>(&'b self) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: LocalEnv::new(&self.exp),
            ty: LocalEnv::new(&self.ty),
            truth: LocalEnv::new(&self.truth),
        }
    }

    pub fn read_file(&mut self) {
        if let Err(e) = self.try_read_file() {
            panic!("{}", e.with_path(&self.path));
        }
    }

    fn try_read_file(&mut self) -> Result<(), Error> {
        let file = read_to_string(format!("{}.alg", self.path.clone()))
            .or_else(|_|read_to_string(format!("{}\\mod.alg", self.path)))
            .expect(&format!("{}", self.path));

        let file = &file;
        let elements = parse_file(file)?;

        for element in elements {
            element.define(self).map_err(|e|e.into())?;
        }
        Ok(())
    }
}
