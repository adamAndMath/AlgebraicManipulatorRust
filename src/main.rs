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
    let mut path = args.next().expect("expected file path");
    path.push_str("\\mod.alg");

    let file = std::fs::read_to_string(path).unwrap();

    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);

    let elements = parser::parse_file(&file);

    for element in elements {
        element.define(&mut env).unwrap();
    }
}