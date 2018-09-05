use envs::Envs;
use exp_id::ExpID;
use exp::Exp;
use ty::{ Variance::*, Type };
use id::ID;

#[test]
fn succ_zero() {
    let mut exps = vec![];
    let mut tys = vec![];
    let mut env = Envs::new(&mut exps, &mut tys);
    
    let fn_id = env.ty.add("fn".to_owned(), (vec!(Contravariant, Covariant), vec!(), vec!()));

    let nat_id = env.ty.add("Nat".to_owned(), (vec!(), vec!(), vec!()));

    let zero_ty = Type::Gen("Nat".to_owned(), vec!()).to_id(&env.local()).unwrap();
    let zero_id = env.exp.add("Zero".to_owned(), (None, zero_ty));
    env.ty.get_mut(nat_id).unwrap().1.push(zero_id);

    let succ_ty = Type::Gen("fn".to_owned(), vec!(Type::Gen("Nat".to_owned(), vec!()), Type::Gen("Nat".to_owned(), vec!()))).to_id(&env.local()).unwrap();
    let succ_id = env.exp.add("Succ".to_owned(), (None, succ_ty));
    env.ty.get_mut(nat_id).unwrap().2.push(succ_id);

    let exp = Exp::Call(Box::new(Exp::Var("Succ".to_owned())), Box::new(Exp::Var("Zero".to_owned())));
    let exp_id = ExpID::Call(Box::new(ExpID::Var(ID::Global(succ_id))), Box::new(ExpID::Var(ID::Global(zero_id))));

    assert_eq!(exp.to_id(&env.local()), Some(exp_id));
}