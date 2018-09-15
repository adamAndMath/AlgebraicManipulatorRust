use predef::*;
use envs::*;
use super::{ Type, Exp };
use variance::Variance::*;
use id::renamed::{ TypeID, PatternID, ExpID };

pub enum Element {
    Struct(String, Vec<Type>),
    Enum(String, Vec<(String, Vec<Type>)>),
    Let(String, Option<Type>, Exp),
    Func(String, Vec<(String, Type)>, Option<Type>, Exp),
}

impl Element {
    pub fn define(&self, env: &mut Envs) -> Option<()> {
        match self {
            Element::Struct(n, ps) => {
                if ps.is_empty() {
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(vec!()));
                    let e_id = env.exp.add(n.clone(), ExpVal::new_empty(TypeID::Gen(ty_id.into(), vec!())));
                    env.ty.get_mut(ty_id).unwrap().push_atom(e_id);
                } else {
                    let p = if let [p] = &ps[..] {p.clone()} else {Type::Tuple(ps.clone())}.to_id(&env.local())?;
                    let ty_id = env.ty.add(n.clone(), TypeVal::new(vec!()));
                    let f_id = env.exp.add(n.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, TypeID::Gen(ty_id.into(), vec!()))))));
                    env.ty.get_mut(ty_id).unwrap().push_comp(f_id);
                }
            },
            Element::Enum(n, vs) => {
                let ty_id = env.ty.add(n.clone(), TypeVal::new(vec!()));
                for (v, ps) in vs {
                    if ps.is_empty() {
                        let v_id = env.exp.add(v.clone(), ExpVal::new_empty(TypeID::Gen(ty_id.into(), vec!())));
                        env.ty.get_mut(ty_id).unwrap().push_atom(v_id);
                    } else {
                        let p = if let [p] = &ps[..] {p.clone()} else {Type::Tuple(ps.clone())}.to_id(&env.local())?;
                        let v_id = env.exp.add(v.clone(), ExpVal::new_empty(TypeID::Gen(FN_ID.into(), vec!((Contravariant, p), (Covariant, TypeID::Gen(ty_id.into(), vec!()))))));
                        env.ty.get_mut(ty_id).unwrap().push_comp(v_id);
                    }
                }
            },
            Element::Let(n, an, e) => {
                let e_id = e.to_id(&env.local())?;
                let e_ty = e_id.type_check(&env.local())?;
                if let Some(t) = an {
                    let t = t.to_id(&env.local())?;
                    if t != e_ty {
                        return None;
                    }
                }
                env.exp.add(n.clone(), ExpVal::new(e_id, e_ty));
            },
            Element::Func(n, ps, an, e) => {
                let f = {
                    let env = env.local();
                    let ps = ps.into_iter().map(|(p,t)|Some((p.clone(), ExpVal::new_empty(t.to_id(&env)?)))).collect::<Option<Vec<_>>>()?;
                    let ts = ps.iter().map(|(_,e)|PatternID::Var(e.ty())).collect::<Vec<_>>();
                    let env = env.scope(ps);
                    let e_id = e.to_id(&env)?;
                    let e_ty = e_id.type_check(&env)?;
                    if let Some(t) = an {
                        let t = t.to_id(&env)?;
                        if t != e_ty {
                            return None;
                        }
                    }
                    let p = if let [t] = &ts[..] {t.clone()} else {PatternID::Tuple(ts.clone())};
                    let t = p.type_check(&env)?;
                    ExpVal::new(ExpID::Lambda(p, Box::new(e_id)), t)
                };

                env.exp.add(n.clone(), f);
            }
        }

        Some(())
    }
}