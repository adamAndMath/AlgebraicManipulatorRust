use envs::Envs;
use exp_id::ExpID;
use ty::Variance::*;
use ty::TypeID;
use id::ID;

#[test]
fn type_check() {
    let mut exps = vec![];
    let mut tys = vec![];
    let mut env = Envs::new(&mut exps, &mut tys);

    let fn_id = env.ty.add("fn".to_owned(), (vec!(Contravariant, Covariant), vec!(), vec!()));

    let nat_id = env.ty.add("Nat".to_owned(), (vec!(), vec!(), vec!()));
    let zero_id = env.exp.add("Zero".to_owned(), (None, TypeID::Gen(nat_id, vec!())));
    env.ty.get_mut(nat_id).unwrap().1.push(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), (None, TypeID::Gen(fn_id, vec!((Contravariant, TypeID::Gen(nat_id, vec!())), (Covariant, TypeID::Gen(nat_id, vec!()))))));
    env.ty.get_mut(nat_id).unwrap().2.push(succ_id);

    let env = env.local();

    //succ(zero)
    let exp = ExpID::Call(Box::new(ExpID::Var(ID::Global(succ_id))), Box::new(ExpID::Var(ID::Global(zero_id))));

    assert_eq!(exp.type_check(&env), Some(TypeID::Gen(nat_id, vec!())));
}