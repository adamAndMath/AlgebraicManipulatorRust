#![feature(range_contains, box_patterns, transpose_result)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
mod parser;

mod env;
mod envs;
mod ast;
mod id;
mod predef;
mod variance;
mod tree;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;

fn main() {
    use predef::*;
    use envs::Envs;
    
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("expected file path");

    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(path, &mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    read_file(&mut env);
}

use envs::Envs;
fn read_file(env: &mut Envs) {
    let file = std::fs::read_to_string(format!("{}.alg", env.path.clone())).or_else(|_|std::fs::read_to_string(format!("{}\\mod.alg", env.path))).expect(&format!("{}", env.path));
    let elements = parser::parse_file(&file);

    for element in elements {
        element.define(env).unwrap();
    }
}