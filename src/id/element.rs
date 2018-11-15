use predef::*;
use env::{ ID, PushID };
use envs::{ Envs, TypeVal, ExpVal, TruthVal };
use variance::Variance;
use super::{ Type, Pattern, Exp, Proof, MatchEnv, TypeCheck, ErrID };

#[derive(Debug)]
pub enum Element {
    Struct(Vec<Variance>, Option<Type>),
    Enum(Vec<Variance>, Vec<Option<Type>>),
    Let(usize, Option<Type>, Exp),
    Func(usize, Option<Type>, Vec<(Pattern, Exp)>),
    Proof(usize, Option<Pattern>, Proof),
}

impl Element {
    pub fn define(&self, env: &mut Envs) -> Result<(), ErrID> {
        match self {
            Element::Struct(gs, p) => {
                let ty_id = env.ty.add(TypeVal::new(gs.clone()));
                let ty = Type::Gen(ty_id.push_id(1), (0..gs.len()).map(|i|Type::Gen(ID::new(i), vec![])).collect());
                if let Some(p) = p {
                    let f_id = env.exp.add(ExpVal::new_empty(func(p.clone(), ty), gs.len()));
                    env.ty[ty_id].push_comp(f_id);
                } else {
                    let e_id = env.exp.add(ExpVal::new_empty(ty, gs.len()));
                    env.ty[ty_id].push_atom(e_id);
                }
            },
            Element::Enum(gs, vs) => {
                let ty_id = env.ty.add(TypeVal::new(gs.clone()));
                let ty = Type::Gen(ty_id.push_id(1), (0..gs.len()).map(|i|Type::Gen(ID::new(i), vec![])).collect());
                let ty_ref = &mut env.ty[ty_id];
                for p in vs {
                    if let Some(p) = p {
                        ty_ref.push_comp(env.exp.add(ExpVal::new_empty(func(p.clone(), ty.clone()), gs.len())));
                    } else {
                        ty_ref.push_atom(env.exp.add(ExpVal::new_empty(ty.clone(), gs.len())));
                    }
                }
            },
            Element::Let(gs, an, e) => {
                let e_ty = e.type_check(&env.scope_ty((0..*gs).map(|_|TypeVal::new(vec![])).collect()))?;
                if let Some(t) = an {
                    if e_ty != *t {
                        return Err(ErrID::TypeMismatch(e_ty, t.clone()));
                    }
                }
                env.exp.add(ExpVal::new(e.clone(), e_ty, *gs));
            },
            Element::Func(gs, None, ps) => {
                let e = Exp::Closure(ps.clone());
                let t = e.type_check(&env.scope_ty((0..*gs).map(|_|TypeVal::new(vec![])).collect()))?;
                env.exp.add(ExpVal::new(e, t, *gs));
            },
            Element::Func(gs, Some(re), ps) => {
                let gs: Vec<_> = (0..*gs).map(|_|TypeVal::new(vec![])).collect();
                let t = {
                    let env = &env.scope_ty(gs.clone());
                    let mut t_in = None;
                    for (p,_) in ps {
                        let t = p.type_check(&env.scope_empty())?;
                        if let Some(ref t_in) = t_in {
                            if t != *t_in { return Err(ErrID::TypeMismatch(t, t_in.clone())); }
                        } else {
                            t_in = Some(t)
                        }
                    }
                    func(t_in.unwrap().pop_id(1).unwrap(), re.clone())
                };
                let id = env.exp.add(ExpVal::new_empty(t.clone(), gs.len()));
                let f = {
                    let env = env.scope_ty(gs);
                    let e = Exp::Closure(ps.clone());
                    let e_ty = e.type_check(&env)?;
                    if e_ty != t { return Err(ErrID::TypeMismatch(e_ty, t)); }
                    e
                };

                env.exp[id].set_val(f);
            },
            Element::Proof(gs, p, proof) => {
                let proof = {
                    let env = env.scope_ty((0..*gs).map(|_|TypeVal::new(vec![])).collect());
                    
                    if let Some(p) = p {
                        let env = env.scope_exp(p.bound());
                        let proof = proof.execute(&env, &MatchEnv::new())?;
                        let t = p.type_check(&env)?;
                        Exp::Call(Box::new(Exp::Var(FORALL_ID.into(), vec![t])), Box::new(Exp::Closure(vec![(p.clone(), proof)])))
                    } else {
                        proof.execute(&env, &MatchEnv::new())?
                    }
                };

                env.truth.add(TruthVal::new(proof, *gs));
            }
        }

        Ok(())
    }
}