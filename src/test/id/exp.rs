use predef::*;
use envs::*;
use variance::Variance;
use id::renamed::{ TypeID, ExpID, TypeCheck };

#[test]
fn type_check() {
    let mut data = predef();
    let mut env = Envs::new(&mut data);

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec!()));
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(type_id!(nat_id), 0));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(type_id!(FN_ID[-nat_id, +nat_id]), 0));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let env = env.local();

    let exp = exp_id!(succ_id(zero_id));

    assert_eq!(exp.type_check(&env), Ok(type_id!(nat_id)));
}