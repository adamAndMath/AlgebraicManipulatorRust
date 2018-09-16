use ast::{ Type, Pattern, Exp };
use tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forwards,
    Backwards,
}

#[derive(Debug)]
pub enum Proof {
    Sequence(String, Vec<Type>, Vec<Exp>, Vec<(Direction, String, Vec<Type>, Vec<Exp>, Tree)>),
    Block(Vec<(String, Proof)>, Box<Proof>),
    Match(Exp, Vec<(Pattern, Proof)>),
}