use predef::*;
use envs::*;
use env::{ LocalID, Path };
use super::{ Type, Pattern, Exp, Proof, ErrAst, ToID };
use variance::Variance::{ self, * };
use id::renamed::{ TypeID, ExpID, MatchEnv, ErrID, TypeCheck };

pub enum Element {
    Module(String, Vec<Element>),
    Using(Path),
    Struct(String, Vec<(Variance, String)>, Option<Type>),
    Enum(String, Vec<(Variance, String)>, Vec<(String, Option<Type>)>),
    Let(String, Vec<String>, Option<Type>, Exp),
    Func(String, Vec<String>, Option<Type>, Vec<(Pattern, Exp)>),
    Proof(String, Vec<String>, Option<Pattern>, Proof),
}

impl Element {
    pub fn define(&self, env: &mut Envs) -> Result<(), ErrAst> {
        match self {
            Element::Module(n, es) =>
                env.child_scope::<ErrAst,_>(n.clone(), |env| {
                    for e in es { e.define(env)? }
                    Ok(())
                })?,
            Element::Using(p) => {
                let mut err = true;
                if let Ok(exp) = env.exp.get_val(p).map(|v|v.clone()) {
                    env.exp.alias(p.name(), exp);
                    err = false;
                }
                if let Ok(ty) = env.ty.get_val(p).map(|v|v.clone()) {
                    env.ty.alias(p.name(), ty);
                    err = false;
                }
                if let Ok(truth) = env.truth.get_val(p).map(|v|v.clone()) {
                    env.truth.alias(p.name(), truth);
                    err = false;
                }
                if err { return Err(ErrAst::UndefinedPath(p.clone())); }
            },
            Element::Struct(n, gs, p) => {
                if let Some(p) = p {
                    let p = p.to_id(&env.local().scope_ty(gs.into_iter().map(|(_,g)|(g.clone(), TypeVal::new(vec![]))).collect::<Vec<_>>()))?;
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                    let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                    let f_id = env.exp.add(n.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, ty))), gs.len()));
                    env.ty.get_mut(ty_id).unwrap().push_comp(f_id);
                } else {
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                    let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                    let e_id = env.exp.add(n.clone(), ExpVal::new_empty(ty, gs.len()));
                    env.ty.get_mut(ty_id).unwrap().push_atom(e_id);
                }
            },
            Element::Enum(n, gs, vs) => {
                let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                let gs = gs.into_iter().map(|(_,g)|(g.clone(), TypeVal::new(vec![]))).collect::<Vec<_>>();
                let vs = vs.into_iter().map(|(n,t)|Ok((n.clone(), t.to_id(&env.local().scope_ty(gs.clone()))?))).collect::<Result<Vec<_>,ErrAst>>()?;
                let mut atoms = vec![];
                let mut comps = vec![];
                let val = {
                    let mut space = env.exp.child_scope();
                    for (v, p) in vs {
                        if let Some(p) = p {
                            comps.push(space.add(v.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, ty.clone()))), gs.len())));
                        } else {
                            atoms.push(space.add(v.clone(), ExpVal::new_empty(ty.clone(), gs.len())));
                        }
                    }
                    space.to_val()
                };
                env.exp.add_val(n.clone(), val);
                let ty = env.ty.get_mut(ty_id).unwrap();
                for atom in atoms {
                    ty.push_atom(atom);
                }
                for comp in comps {
                    ty.push_comp(comp);
                }
            },
            Element::Let(n, gs, an, e) => {
                let (e_id, e_ty) = {
                    let env = env.local();
                    let env = env.scope_ty(gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect());
                    let e_id = e.to_id(&env)?;
                    let e_ty = e_id.type_check(&env)?;
                    (e_id, e_ty)
                };
                if let Some(t) = an {
                    let t = t.to_id(&env.local())?;
                    if e_ty != t {
                        return Err(ErrAst::ErrID(ErrID::TypeMismatch(e_ty, t)));
                    }
                }
                env.exp.add(n.clone(), ExpVal::new(e_id, e_ty, gs.len()));
            },
            Element::Func(n, gs, None, ps) => {
                let f = {
                    let env = env.local();
                    let env = env.scope_ty(gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect());
                    let e = ExpID::Closure(ps.to_id(&env)?);
                    let t = e.type_check(&env)?;
                    ExpVal::new(e, t, gs.len())
                };

                env.exp.add(n.clone(), f);
            },
            Element::Func(n, gs, Some(re), ps) => {
                let gs: Vec<_> = gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect();
                let t = {
                    let env = env.local();
                    let env = env.scope_ty(gs.clone());
                    let re = re.to_id(&env)?;
                    let mut t_in = None;
                    for (p,_) in ps {
                        let t = p.to_id(&env)?.type_check(&env)?;
                        if let Some(ref t_in) = t_in {
                            if t != *t_in { return Err(ErrAst::ErrID(ErrID::TypeMismatch(t, t_in.clone()))); }
                        } else {
                            t_in = Some(t)
                        }
                    }
                    func(t_in.unwrap(), re)
                };
                let id = env.exp.add(n.clone(), ExpVal::new_empty(t.clone(), gs.len()));
                let f = {
                    let env = env.local();
                    let env = env.scope_ty(gs);
                    let e = ExpID::Closure(ps.to_id(&env)?);
                    let e_ty = e.type_check(&env)?;
                    if e_ty != t { return Err(ErrAst::ErrID(ErrID::TypeMismatch(e_ty, t))); }
                    e
                };

                env.exp.get_mut(id).unwrap().set_val(f);
            },
            Element::Proof(n, gs, p, proof) => {
                let proof = {
                    let env = env.local();
                    let gs = gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect();
                    let env = env.scope_ty(gs);
                    
                    if let Some(p) = p {
                        let ps = p.bound();
                        let p = p.to_id(&env)?;
                        let ps = ps.into_iter().zip(p.bound()).collect();
                        let env = env.scope(ps);
                        let proof = proof.to_id(&env)?.execute(&env, &MatchEnv::new())?;
                        let t = p.type_check(&env)?;
                        ExpID::Call(Box::new(ExpID::Var(FORALL_ID.into(), vec![t])), Box::new(ExpID::Closure(vec![(p, proof)])))
                    } else {
                        proof.to_id(&env)?.execute(&env, &MatchEnv::new())?
                    }
                };

                env.truth.add(n.to_owned(), TruthVal::new(proof, gs.len()));
            }
        }

        Ok(())
    }
}