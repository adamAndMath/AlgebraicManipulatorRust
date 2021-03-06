use pest::Parser;
use pest::iterators::Pair as PestPair;
use pest::error::Error as PestError;
use tree::{ Tree, TreeChar };
use variance::Variance;
use id::Direction;
use env::Path;
use ast::*;

#[derive(Parser)]
#[grammar = "alg.pest"]
struct AlgParser;

type Pair<'f> = PestPair<'f, Rule>;
pub type Error = PestError<Rule>;

pub fn parse_file<'f>(file: &'f str) -> Result<Vec<Module<Word<'f>>>, Error> {
    let pairs = AlgParser::parse(Rule::file, file);
    pairs.map(|mut p|p.next().unwrap().into_inner().filter(|p|p.as_rule() != Rule::EOI).map(Module::parse_pair).collect())
}

pub trait Parse<'f>: Sized {
    const R: Rule;
    fn parse(file: &'f str) -> Self {
        match AlgParser::parse(Self::R, file).map(|mut p|Self::parse_pair(p.next().unwrap())) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        }
    }

    fn parse_pair(pair: Pair<'f>) -> Self;
}

fn parse<'f, T: Parse<'f>>(pairs: &mut Iterator<Item = Pair<'f>>) -> T {
    T::parse_pair(pairs.next().unwrap())
}

fn parse_vec<'f, T: Parse<'f>>(pairs: &mut Iterator<Item = Pair<'f>>) -> Vec<T> {
    pairs.next().unwrap().into_inner().map(T::parse_pair).collect()
}

fn parse_t2<'f, T: Parse<'f>, U: Parse<'f>>(pair: Pair<'f>) -> (T, U) {
    let mut inner = pair.into_inner();
    let t = T::parse_pair(inner.next().unwrap());
    let u = U::parse_pair(inner.next().unwrap());
    (t, u)
}

impl<'f> Parse<'f> for Word<'f> {
    const R: Rule = Rule::name;
    fn parse_pair(pair: Pair<'f>) -> Self {
        Word::new(pair.as_span())
    }
}

impl<'f> Parse<'f> for usize {
    const R: Rule = Rule::number;
    fn parse_pair(pair: Pair<'f>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

impl<'f> Parse<'f> for TreeChar {
    const R: Rule = Rule::tree_char;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_str() {
            "f" => TreeChar::Func,
            "t" => TreeChar::Tuple,
            r => unreachable!(r),
        }
    }
}

impl<'f> Parse<'f> for Direction {
    const R: Rule = Rule::truth_dir;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_str() {
            "." => Direction::Forwards,
            "~" => Direction::Backwards,
            r => unreachable!(r),
        }
    }
}

impl<'f> Parse<'f> for Variance {
    const R: Rule = Rule::variance;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_str() {
            "+" => Variance::Covariant,
            "-" => Variance::Contravariant,
            "" => Variance::Invariant,
            r => unreachable!(r),
        }
    }
}

impl<'f> Parse<'f> for Tree {
    const R: Rule = Rule::tree;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::tree => pair.into_inner().map(Tree::parse_pair).fold(Tree::default(), |a,b|a*b),
            Rule::tree_path => pair.into_inner().map(Tree::parse_pair).fold(Tree::default(), |a,b|a+b),
            Rule::tree_char => Tree::edge(TreeChar::parse_pair(pair)),
            Rule::number => Tree::edge(usize::parse_pair(pair)),
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for Path<Word<'f>> {
    const R: Rule = Rule::path;
    fn parse_pair(pair: Pair<'f>) -> Self {
        if pair.as_rule() != Rule::path { unreachable!("{:?}", pair.as_rule()) }
        Path::new(pair.into_inner().map(Parse::parse_pair).collect())
    }
}

impl<'f> Parse<'f> for Type<Word<'f>> {
    const R: Rule = Rule::ty;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::ty_gen => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                Type::Gen(name, gen)
            },
            Rule::ty_tuple => {
                let tys = pair.into_inner().map(Type::parse_pair).collect::<Vec<_>>();
                if tys.len() == 1 {
                    tys.into_iter().next().unwrap()
                } else {
                    Type::Tuple(tys)
                }
            },
            Rule::ty_fn => {
                let mut inner = pair.into_inner();
                let t_in = parse(&mut inner);
                let t_out = parse(&mut inner);
                Type::Func(Box::new(t_in), Box::new(t_out))
            }
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for Pattern<Word<'f>> {
    const R: Rule = Rule::pattern;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::pattern_var => {
                let mut inner = pair.into_inner();
                let name = Parse::parse_pair(inner.next().unwrap());
                let ty = Parse::parse_pair(inner.next().unwrap());
                Pattern::Var(name, ty)
            },
            Rule::pattern_atom => {
                let mut inner = pair.into_inner();
                let name = Parse::parse_pair(inner.next().unwrap());
                let gen = inner.next().unwrap().into_inner().map(Parse::parse_pair).collect();
                Pattern::Atom(name, gen)
            },
            Rule::pattern_comp => {
                let mut inner = pair.into_inner();
                let name = Parse::parse_pair(inner.next().unwrap());
                let gen = inner.next().unwrap().into_inner().map(Parse::parse_pair).collect();
                let arg = Parse::parse_pair(inner.next().unwrap());
                Pattern::Comp(name, gen, Box::new(arg))
            },
            Rule::pattern_tuple => {
                let ps = pair.into_inner().map(Pattern::parse_pair).collect::<Vec<_>>();
                if ps.len() == 1 {
                    ps.into_iter().next().unwrap()
                } else {
                    Pattern::Tuple(ps)
                }
            },
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for Option<Pattern<Word<'f>>> {
    const R: Rule = Rule::par;
    fn parse_pair(pair: Pair<'f>) -> Self {
        pair.into_inner().next().map(Pattern::parse_pair)
    }
}

impl<'f> Parse<'f> for Patterned<Word<'f>, Exp<Word<'f>>> {
    const R: Rule = Rule::exp_arm;
    fn parse_pair(pair: Pair<'f>) -> Self {
        let mut inner = pair.into_inner();
        let pattern = parse(&mut inner);
        let exp = parse(&mut inner);
        Patterned(pattern, exp)
    }
}

impl<'f> Parse<'f> for Exp<Word<'f>> {
    const R: Rule = Rule::exp;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::exp_var => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                Exp::Var(name, gen)
            },
            Rule::exp_call => {
                let mut es = pair.into_inner().map(Exp::parse_pair);
                let f = es.next().unwrap();
                es.fold(f, |f, e|Exp::Call(Box::new(f), Box::new(e)))
            },
            Rule::exp_tuple => {
                let es = pair.into_inner().map(Exp::parse_pair).collect::<Vec<_>>();
                if es.len() == 1 {
                    es.into_iter().next().unwrap()
                } else {
                    Exp::Tuple(es)
                }
            },
            Rule::exp_match => Exp::Closure(pair.into_inner().map(Patterned::parse_pair).collect()),
            Rule::exp_lambda => Exp::Closure(vec![Patterned::parse_pair(pair)]),
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for TruthRef<Word<'f>> {
    const R: Rule = Rule::truth_ref;
    fn parse_pair(pair: Pair<'f>) -> Self {
        let mut inner = pair.into_inner();
        let name = parse(&mut inner);
        let gen = parse_vec(&mut inner);
        let par = inner.next().map(Parse::parse_pair);
        TruthRef::new(name, gen, par)
    }
}

impl<'f> Parse<'f> for (Direction, TruthRef<Word<'f>>, Tree) {
    const R: Rule = Rule::substitute;
    fn parse_pair(pair: Pair<'f>) -> Self {
        let mut inner = pair.into_inner();
        let dir = parse(&mut inner);
        let truth = parse(&mut inner);
        let tree = parse(&mut inner);
        (dir, truth, tree)
    }
}

impl<'f> Parse<'f> for Patterned<Word<'f>, Proof<Word<'f>>> {
    const R: Rule = Rule::proof_arm;
    fn parse_pair(pair: Pair<'f>) -> Self {
        let mut inner = pair.into_inner();
        let pattern = parse(&mut inner);
        let proof = parse(&mut inner);
        Patterned(pattern, proof)
    }
}

impl<'f> Parse<'f> for Proof<Word<'f>> {
    const R: Rule = Rule::proof;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::proof_sequence => {
                let mut inner = pair.into_inner();
                let truth = parse(&mut inner);
                let v = inner.map(Parse::parse_pair).collect();
                Proof::Sequence(truth, v)
            },
            Rule::proof_block => {
                let mut inner = pair.into_inner();
                let elm = parse_vec(&mut inner);
                let p = parse(&mut inner);
                Proof::Block(elm, Box::new(p))
            },
            Rule::proof_match => {
                let mut inner = pair.into_inner();
                let e = parse(&mut inner);
                let v = parse_vec(&mut inner);
                Proof::Match(e, v)
            },
            Rule::proof_forall => {
                let mut inner = pair.into_inner();
                let pattern: Pattern<_> = parse(&mut inner);
                let proof = parse(&mut inner);
                Proof::Forall(vec![Patterned(pattern, proof)])
            },
            Rule::proof_forall_block => {
                let mut inner = pair.into_inner();
                let arms = parse_vec(&mut inner);
                Proof::Forall(arms)
            },
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for Element<Word<'f>> {
    const R: Rule = Rule::element;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::elm_struct => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = inner.next().unwrap().into_inner().map(parse_t2).collect();
                let ty = inner.next().unwrap().into_inner().next().map(Parse::parse_pair);
                Element::Struct(name, gen, ty)
            },
            Rule::elm_enum => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = inner.next().unwrap().into_inner().map(parse_t2).collect();
                inner.next();
                let v = inner.map(|pair| {
                    let mut inner = pair.into_inner();
                    let name = parse(&mut inner);
                    let ty = inner.next().unwrap().into_inner().next().map(Parse::parse_pair);
                    (name, ty)
                }).collect();
                Element::Enum(name, gen, v)
            },
            Rule::elm_let => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                let ty = inner.next().unwrap().into_inner().next().map(Parse::parse_pair);
                inner.next();
                let exp = parse(&mut inner);
                Element::Let(name, gen, ty, exp)
            },
            Rule::elm_func => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                let pattern = parse(&mut inner);
                let re = inner.next().unwrap().into_inner().next().map(Parse::parse_pair);
                inner.next();
                let exp = parse(&mut inner);
                Element::Func(name, gen, re, vec![Patterned(pattern, exp)])
            },
            Rule::elm_func_match => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                let re = inner.next().unwrap().into_inner().next().map(Parse::parse_pair);
                inner.next();
                let exp = parse_vec(&mut inner);
                Element::Func(name, gen, re, exp)
            },
            Rule::elm_proof => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                let par = parse(&mut inner);
                inner.next();
                let proof = parse(&mut inner);

                Element::Proof(name, gen, match par {
                    Some(par) => Proof::Forall(vec![Patterned(par, proof)]),
                    None => proof,
                })
            },
            Rule::elm_proof_forall => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let gen = parse_vec(&mut inner);
                inner.next();
                let arms = parse_vec(&mut inner);

                Element::Proof(name, gen, Proof::Forall(arms))
            },
            r => unreachable!("{:?}", r),
        }
    }
}

impl<'f> Parse<'f> for Module<Word<'f>> {
    const R: Rule = Rule::module;
    fn parse_pair(pair: Pair<'f>) -> Self {
        match pair.as_rule() {
            Rule::elm_mod => {
                let mut inner = pair.into_inner();
                let name = parse(&mut inner);
                let elements = inner.next().map(|p|p.into_inner().map(Module::parse_pair).collect());
                Module::Module(name, elements)
            },
            Rule::elm_use => Module::Using(parse(&mut pair.into_inner())),
            _ => Module::Element(Element::parse_pair(pair)),
        }
    }
}
