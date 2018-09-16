use predef::*;
use envs::*;
use env::LocalID;
use super::{ Type, Exp, Proof };
use variance::Variance::{ self, * };
use id::renamed::{ TypeID, PatternID, ExpID };

pub enum Element {
    Struct(String, Vec<(Variance, String)>, Vec<Type>),
    Enum(String, Vec<(Variance, String)>, Vec<(String, Vec<Type>)>),
    Let(String, Vec<String>, Option<Type>, Exp),
    Func(String, Vec<String>, Vec<(String, Type)>, Option<Type>, Exp),
    Proof(String, Vec<String>, Vec<(String, Type)>, Proof),
}

impl Element {
    pub fn define(&self, env: &mut Envs) -> Option<()> {
        match self {
            Element::Struct(n, gs, ps) => {
                if ps.is_empty() {
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                    let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                    let e_id = env.exp.add(n.clone(), ExpVal::new_empty(ty, gs.len()));
                    env.ty.get_mut(ty_id).unwrap().push_atom(e_id);
                } else {
                    let p = if let [p] = &ps[..] {p.clone()} else {Type::Tuple(ps.clone())}.to_id(&env.local())?;
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                    let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                    let f_id = env.exp.add(n.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, ty))), gs.len()));
                    env.ty.get_mut(ty_id).unwrap().push_comp(f_id);
                }
            },
            Element::Enum(n, gs, vs) => {
                let ty_id = env.ty.add(n.clone(), TypeVal::new(gs.into_iter().map(|(v,_)|*v).collect()));
                let ty = TypeID::Gen(ty_id.into(), gs.into_iter().enumerate().map(|(i,(v,_))|(*v, TypeID::Gen(LocalID::new(i), vec![]))).collect());
                let gs = gs.into_iter().map(|(_,g)|(g.clone(), TypeVal::new(vec![]))).collect::<Vec<_>>();
                for (v, ps) in vs {
                    if ps.is_empty() {
                        let v_id = env.exp.add(v.clone(), ExpVal::new_empty(ty.clone(), gs.len()));
                        env.ty.get_mut(ty_id).unwrap().push_atom(v_id);
                    } else {
                        let p = if let [p] = &ps[..] {p.clone()} else {Type::Tuple(ps.clone())}.to_id(&env.local().scope_ty(gs.clone()))?;
                        let v_id = env.exp.add(v.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, ty.clone()))), gs.len()));
                        env.ty.get_mut(ty_id).unwrap().push_comp(v_id);
                    }
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
                    if t != e_ty {
                        return None;
                    }
                }
                env.exp.add(n.clone(), ExpVal::new(e_id, e_ty, gs.len()));
            },
            Element::Func(n, gs, ps, None, e) => {
                let f = {
                    let env = env.local();
                    let env = env.scope_ty(gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect());
                    let ps = ps.into_iter().map(|(p,t)|Some((p.clone(), ExpVal::new_empty(t.to_id(&env)?, 0)))).collect::<Option<Vec<_>>>()?;
                    let ts = ps.iter().map(|(_,e)|PatternID::Var(e.ty())).collect::<Vec<_>>();
                    let env = env.scope(ps);
                    let e_id = e.to_id(&env)?;
                    let e_ty = e_id.type_check(&env)?;
                    let p = if let [t] = &ts[..] {t.clone()} else {PatternID::Tuple(ts.clone())};
                    let t = TypeID::Gen(FN_ID.into(), vec![(Contravariant, p.type_check(&env)?), (Covariant, e_ty)]);
                    ExpVal::new(ExpID::Lambda(p, Box::new(e_id)), t, gs.len())
                };

                env.exp.add(n.clone(), f);
            },
            Element::Func(n, gs, ps, Some(re), e) => {
                let gs: Vec<_> = gs.into_iter().map(|g|(g.clone(), TypeVal::new(vec![]))).collect();
                let (re, p, t) = {
                    let env = env.local();
                    let env = env.scope_ty(gs.clone());
                    let re = re.to_id(&env)?;
                    let ts = ps.iter().map(|(_,t)|Some(PatternID::Var(t.to_id(&env)?))).collect::<Option<Vec<_>>>()?;
                    let p = if let [t] = &ts[..] {t.clone()} else {PatternID::Tuple(ts.clone())};
                    let t = TypeID::Gen(FN_ID.into(), vec![(Contravariant, p.type_check(&env)?), (Covariant, re.clone())]);
                    (re, p, t)
                };
                let id = env.exp.add(n.clone(), ExpVal::new_empty(t.clone(), gs.len()));
                let f = {
                    let env = env.local();
                    let env = env.scope_ty(gs);
                    let ps = ps.into_iter().map(|(p,t)|Some((p.clone(), ExpVal::new_empty(t.to_id(&env)?, 0)))).collect::<Option<Vec<_>>>()?;
                    let env = env.scope(ps);
                    let e_id = e.to_id(&env)?;
                    let e_ty = e_id.type_check(&env)?;
                    if re != e_ty { return None; }
                    ExpID::Lambda(p, Box::new(e_id))
                };

                env.exp.get_mut(id).unwrap().set_val(f);
            },
            Element::Proof(n, gs, ps, proof) => {
                unimplemented!();
            }
        }

        Some(())
    }
}