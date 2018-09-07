use envs::Envs;
use exp_id::ExpID;
use ty::Variance::*;
use ty::{ Variance, TypeID };
use id::LocalID;

#[test]
fn type_check() {
    let mut exps = vec![];
    let mut tys = vec![];
    let mut env = Envs::new(&mut exps, &mut tys);

    let fn_id = env.ty.add("fn".to_owned(), (vec!(Contravariant, Covariant), vec!(), vec!()));

    let nat_id = env.ty.add("Nat".to_owned(), (vec!(), vec!(), vec!()));
    let zero_id = env.exp.add("Zero".to_owned(), (None, type_id!(nat_id)));
    env.ty.get_mut(nat_id).unwrap().1.push(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), (None, type_id!(fn_id[-nat_id, +nat_id])));
    env.ty.get_mut(nat_id).unwrap().2.push(succ_id);

    let env = env.local();

    let exp = exp_id!(*succ_id(*zero_id));

    assert_eq!(exp.type_check(&env), Some(type_id!(nat_id)));
}