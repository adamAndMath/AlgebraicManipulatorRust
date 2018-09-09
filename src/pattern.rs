use envs::*;
use id::*;
use ty::{ Variance::*, TypeID };

#[derive(Debug)]
pub enum Pattern {
    Var(String),
    Func(String, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternID {
    Var(TypeID),
    Atom(ID<ExpVal>),
    Comp(ID<ExpVal>, Box<PatternID>),
    Tuple(Vec<PatternID>),
}

impl Pattern {
    pub fn to_id(&self, ty: &TypeID, env: &LocalEnvs) -> Option<(PatternID, Vec<(String, ExpVal)>)> {
        match (self, ty) {
            (Pattern::Var(s), ty) =>
                match env.exp.get_id(s) {
                    Some(LocalID::Global(id)) =>
                        if let TypeID::Gen(ty_id, _) = ty {
                            if env.ty.get(*ty_id)?.contains_atom(&id) {
                                Some((PatternID::Atom(id), vec!()))
                            } else {
                                None
                            }
                        } else {
                            None
                        },
                    _ =>  Some((PatternID::Var(ty.clone()), vec!((s.clone(), ExpVal::new_empty(ty.clone()))))),
                },
            (Pattern::Func(s, p), TypeID::Gen(f_id, gs)) if f_id != &ID::new(0) => {
                let (in_id, out_id) = match &gs[..] {
                    [(Contravariant, in_id), (Covariant, out_id)] => (in_id, out_id),
                    _ => return None,
                };
                let ty_out = match out_id {
                    TypeID::Gen(out_id, _) => env.ty.get(out_id.clone()),
                    _ => None,
                }?;
                let id = match env.exp.get_id(s)? {
                    LocalID::Global(id) => id,
                    _ => return None,
                };
                if !ty_out.contains_comp(&id) {
                    return None;
                }
                p.to_id(in_id, env).map(|(p,v)|(PatternID::Comp(id, Box::new(p)), v))
            },
            (Pattern::Tuple(ps), TypeID::Tuple(ts)) => {
                if ps.len() != ts.len() {
                    return None;
                }

                let mut ps_id = vec!();
                let mut vs = vec!();
                for (p, v) in ps.into_iter().zip(ts).map(|(p, t)|p.to_id(t, env)).collect::<Option<Vec<_>>>()? {
                    ps_id.push(p);
                    vs.extend(v);
                }

                Some((PatternID::Tuple(ps_id), vs))
            },
            _ => None,
        }
    }
}

impl PatternID {
    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            PatternID::Var(ty) => vec!(ExpVal::new_empty(ty.clone())),
            PatternID::Atom(_) => vec!(),
            PatternID::Comp(_, p) => p.bound(),
            PatternID::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }
}