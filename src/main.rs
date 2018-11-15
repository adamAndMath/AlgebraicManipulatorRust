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
    match run() {
        Ok(()) => (),
        Err(e) => panic!("{}", e),
    }
}

use parser::{ parse_file, Error };
use std::fs::read_to_string;
use envs::{ NameData, Envs };
use ast::ErrAst;

fn run() -> Result<(), Error> {
    use predef::*;
    
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("expected file path");

    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let file = read_file(&path);
    parse_file(&file)?.into_iter().flat_map(|e|e.to_id(&path, &mut space)).collect::<Result<Vec<_>,_>>()?.into_iter().map(|e|e.define(&mut env)).collect::<Result<(),_>>().map_err(|e|ErrAst::from(e).into())
}

fn read_file(path: &str) -> String {
    read_to_string(format!("{}.alg", path))
        .or_else(|_|read_to_string(format!("{}\\mod.alg", path)))
        .expect(&format!("{}", path))
}