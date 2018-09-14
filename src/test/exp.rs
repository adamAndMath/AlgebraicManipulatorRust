use predef::*;
use envs::*;
use id::renamed::ExpID;
use ast::{ Type, Exp };

#[test]
fn succ_zero() {
    let (mut exps, mut tys) = predef();
    let mut env = Envs::new(&mut exps, &mut tys);
    env.ty.alias("fn".to_owned(), FN_ID);

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec!()));

    let zero_ty = ttype!(Nat).to_id(&env.local()).unwrap();
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(zero_ty));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);

    let succ_ty = ttype!(fn[Nat, Nat]).to_id(&env.local()).unwrap();
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(succ_ty));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let exp = exp!(Succ(Zero));

    assert_eq!(exp.to_id(&env.local()), Some(exp_id!(succ_id(zero_id))));
}