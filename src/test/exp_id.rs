use envs::*;
use exp_id::ExpID;
use ty::Variance::*;
use ty::{ Variance, TypeID };

#[test]
fn type_check() {
    let mut exps = vec![];
    let mut tys = vec![];
    let mut env = Envs::new(&mut exps, &mut tys);

    let fn_id = env.ty.add("fn".to_owned(), TypeVal::new(vec!(Contravariant, Covariant)));

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec!()));
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(type_id!(nat_id)));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(type_id!(fn_id[-nat_id, +nat_id])));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let env = env.local();

    let exp = exp_id!(succ_id(zero_id));

    assert_eq!(exp.type_check(&env), Some(type_id!(nat_id)));
}