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

    let mut data = predef();
    let mut env = Envs::new(path, &mut data);
    alias_predef(&mut env);
    env.read_file();
}
